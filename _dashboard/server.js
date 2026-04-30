// Dashboard backend for the librespot play-count fix project.
//
// Runs on http://127.0.0.1:8898 — same host:port as librespot's registered
// OAuth redirect URI, so we can reuse its public client_id without registering
// a new Spotify developer app.

import express from 'express';
import crypto from 'node:crypto';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { spawn } from 'node:child_process';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const PORT = 8898;
const HOST = '127.0.0.1';
const SPOTIFY_CLIENT_ID = '65b708073fc0480ea92a077233ca87bd';
const SPOTIFY_REDIRECT_URI = `http://${HOST}:${PORT}/login`;
const SPOTIFY_SCOPES = [
  'streaming',
  'user-read-recently-played',
  'user-read-currently-playing',
  'user-read-playback-state',
  'user-modify-playback-state',
  'user-read-private',
  'user-read-email',
].join(' ');

const TOKEN_FILE = path.join(__dirname, '.tokens.json');
const REPO_ROOT = path.resolve(__dirname, '..');

function b64url(buf) {
  return buf.toString('base64').replace(/\+/g, '-').replace(/\//g, '_').replace(/=+$/, '');
}
function sha256(buf) {
  return crypto.createHash('sha256').update(buf).digest();
}

const oauthState = { verifier: null, state: null };

function loadTokens() {
  try {
    return JSON.parse(fs.readFileSync(TOKEN_FILE, 'utf8'));
  } catch {
    return null;
  }
}
function saveTokens(t) {
  fs.writeFileSync(TOKEN_FILE, JSON.stringify(t, null, 2));
}

async function refreshIfNeeded() {
  const t = loadTokens();
  if (!t) return null;
  const expiresAt = t.obtained_at + (t.expires_in * 1000);
  if (Date.now() < expiresAt - 60_000) return t;
  if (!t.refresh_token) return null;
  const body = new URLSearchParams({
    grant_type: 'refresh_token',
    refresh_token: t.refresh_token,
    client_id: SPOTIFY_CLIENT_ID,
  });
  const r = await fetch('https://accounts.spotify.com/api/token', {
    method: 'POST',
    headers: { 'Content-Type': 'application/x-www-form-urlencoded' },
    body,
  });
  if (!r.ok) return null;
  const j = await r.json();
  const next = {
    ...t,
    access_token: j.access_token,
    expires_in: j.expires_in,
    obtained_at: Date.now(),
    refresh_token: j.refresh_token || t.refresh_token,
    scope: j.scope || t.scope,
  };
  saveTokens(next);
  return next;
}

// Spotify enforces per-user rolling rate limits and returns 429 with a
// Retry-After header (seconds). We track the soonest moment we're allowed
// to call again and refuse outbound calls until then so the dashboard's
// background polling doesn't keep extending the lockout.
const RATE_LIMIT = { retry_after_at: 0 };

function rateLimitNow(retryAfterSec) {
  const ms = (Number(retryAfterSec) || 5) * 1000;
  RATE_LIMIT.retry_after_at = Math.max(RATE_LIMIT.retry_after_at, Date.now() + ms);
}
function secondsUntilAllowed() {
  const ms = RATE_LIMIT.retry_after_at - Date.now();
  return ms > 0 ? Math.ceil(ms / 1000) : 0;
}
function rateLimited() {
  return secondsUntilAllowed() > 0;
}

async function spotifyFetch(p, accessToken, init) {
  if (rateLimited()) {
    return { status: 429, data: { error: { status: 429, message: 'rate-limited (cached)', retry_after: secondsUntilAllowed() } } };
  }
  const r = await fetch(`https://api.spotify.com/v1${p}`, {
    ...init,
    headers: { Authorization: `Bearer ${accessToken}`, ...(init?.headers || {}) },
  });
  if (r.status === 429) {
    const ra = r.headers.get('retry-after');
    rateLimitNow(ra);
  }
  const text = await r.text();
  let data = null;
  try { data = text ? JSON.parse(text) : null; } catch { data = { _raw: text }; }
  return { status: r.status, data };
}

async function spotifyGet(p, accessToken) {
  return spotifyFetch(p, accessToken, { method: 'GET' });
}

async function spotifyPut(p, accessToken, body) {
  return spotifyFetch(p, accessToken, {
    method: 'PUT',
    headers: { 'Content-Type': 'application/json' },
    body: body ? JSON.stringify(body) : undefined,
  });
}

const app = express();
app.use(express.json());
app.use(express.static(path.join(__dirname, 'public')));

// --- OAuth flow ---

app.get('/auth/start', (_req, res) => {
  const verifier = b64url(crypto.randomBytes(48));
  const challenge = b64url(sha256(verifier));
  const state = b64url(crypto.randomBytes(16));
  oauthState.verifier = verifier;
  oauthState.state = state;
  const u = new URL('https://accounts.spotify.com/authorize');
  u.searchParams.set('client_id', SPOTIFY_CLIENT_ID);
  u.searchParams.set('response_type', 'code');
  u.searchParams.set('redirect_uri', SPOTIFY_REDIRECT_URI);
  u.searchParams.set('scope', SPOTIFY_SCOPES);
  u.searchParams.set('code_challenge_method', 'S256');
  u.searchParams.set('code_challenge', challenge);
  u.searchParams.set('state', state);
  // Force the Spotify auth dialog so the user can pick an account, even if
  // an old session cookie is still present in the browser. Without this,
  // Spotify silently re-issues a token for whoever owns the cookie — which
  // burned us when the user "switched accounts" but kept getting the old
  // user's identity back.
  u.searchParams.set('show_dialog', 'true');
  res.redirect(u.toString());
});

app.get('/login', async (req, res) => {
  const { code, state, error } = req.query;
  if (error) return res.status(400).send(`OAuth error: ${error}`);
  if (!code || state !== oauthState.state) return res.status(400).send('Bad state');
  const body = new URLSearchParams({
    grant_type: 'authorization_code',
    code: String(code),
    redirect_uri: SPOTIFY_REDIRECT_URI,
    client_id: SPOTIFY_CLIENT_ID,
    code_verifier: oauthState.verifier,
  });
  const r = await fetch('https://accounts.spotify.com/api/token', {
    method: 'POST',
    headers: { 'Content-Type': 'application/x-www-form-urlencoded' },
    body,
  });
  if (!r.ok) {
    const txt = await r.text();
    return res.status(500).send(`Token exchange failed: ${txt}`);
  }
  const j = await r.json();
  saveTokens({
    access_token: j.access_token,
    refresh_token: j.refresh_token,
    expires_in: j.expires_in,
    scope: j.scope,
    token_type: j.token_type,
    obtained_at: Date.now(),
  });
  res.redirect('/');
});

app.post('/auth/logout', (_req, res) => {
  try { fs.unlinkSync(TOKEN_FILE); } catch {}
  res.json({ ok: true });
});

// Hard reset: deletes our token and bounces the browser through Spotify's
// global logout, then back to /auth/start. This kills any session cookie
// that show_dialog=true alone wouldn't override.
app.get('/auth/hard-reset', (_req, res) => {
  try { fs.unlinkSync(TOKEN_FILE); } catch {}
  const ret = encodeURIComponent('http://127.0.0.1:8898/auth/start');
  res.redirect(`https://www.spotify.com/logout/?continue=${ret}`);
});

app.get('/api/rate-limit', (_req, res) => {
  res.json({ seconds_until_allowed: secondsUntilAllowed() });
});

app.get('/auth/status', async (_req, res) => {
  // Local check only — no Spotify call. The presence of a refreshable
  // token is sufficient to confirm the user signed in. Calling /me here
  // costs us a Web API call on every page load, which contributes to
  // rate-limit lockouts.
  const t = await refreshIfNeeded();
  if (!t) return res.json({ authenticated: false });
  res.json({
    authenticated: true,
    expires_at: t.obtained_at + t.expires_in * 1000,
    scopes: (t.scope || '').split(' '),
    me: null,
  });
});

// --- Spotify Web API proxies ---

app.get('/api/recently-played', async (_req, res) => {
  const t = await refreshIfNeeded();
  if (!t) return res.status(401).json({ error: 'not_authenticated' });
  const r = await spotifyGet('/me/player/recently-played?limit=20', t.access_token);
  res.status(r.status).json(r.data);
});

app.get('/api/currently-playing', async (_req, res) => {
  const t = await refreshIfNeeded();
  if (!t) return res.status(401).json({ error: 'not_authenticated' });
  const r = await spotifyGet('/me/player/currently-playing', t.access_token);
  res.status(r.status).json(r.data);
});

app.get('/api/playback-state', async (_req, res) => {
  const t = await refreshIfNeeded();
  if (!t) return res.status(401).json({ error: 'not_authenticated' });
  const r = await spotifyGet('/me/player', t.access_token);
  res.status(r.status).json(r.data);
});

app.get('/api/devices', async (_req, res) => {
  const t = await refreshIfNeeded();
  if (!t) return res.status(401).json({ error: 'not_authenticated' });
  const r = await spotifyGet('/me/player/devices', t.access_token);
  res.status(r.status).json(r.data);
});

app.post('/api/transfer/:device_id', async (req, res) => {
  const t = await refreshIfNeeded();
  if (!t) return res.status(401).json({ error: 'not_authenticated' });
  const r = await spotifyPut('/me/player', t.access_token, {
    device_ids: [req.params.device_id],
    play: false,
  });
  res.status(r.status).json(r.data ?? { ok: true });
});

const DEFAULT_TEST_TRACK = 'spotify:track:4uLU6hMCjMI75M1A2tKUQC'; // "Never Gonna Give You Up" — picked because it's universally licensed and >30s.

app.post('/api/play', async (req, res) => {
  const t = await refreshIfNeeded();
  if (!t) return res.status(401).json({ error: 'not_authenticated' });
  const { device_id, track_uri } = req.body || {};
  const uri = track_uri || DEFAULT_TEST_TRACK;
  const url = device_id ? `/me/player/play?device_id=${encodeURIComponent(device_id)}` : '/me/player/play';
  const r = await spotifyPut(url, t.access_token, { uris: [uri], position_ms: 0 });
  res.status(r.status).json(r.data ?? { ok: true, played: uri });
});

const E2E_RUN_STATE = { running: false };

// One-click end-to-end check. Uses the device id that librespot prints
// during connect-state registration ("successfully put connect state for
// <hex>"), so we don't need to call /me/player/devices at all — that
// endpoint is the most rate-limit-prone in our setup.
//
// Total Web API calls per run: 1 transfer + 1 play + up to 6 recently-played
// polls = 3-8 calls in 75 seconds.
app.post('/api/run-e2e', async (_req, res) => {
  if (E2E_RUN_STATE.running) return res.status(409).json({ error: 'already_running' });
  E2E_RUN_STATE.running = true;
  try {
    const t = await refreshIfNeeded();
    if (!t) return res.status(401).json({ error: 'not_authenticated' });
    if (rateLimited()) return res.status(429).json({ error: 'rate_limited', retry_after: secondsUntilAllowed() });

    const log = [];
    const stamp = (msg) => { const s = `[${new Date().toISOString()}] ${msg}`; log.push(s); return s; };

    if (librespotState.status !== 'running') return res.status(412).json({ error: 'librespot_not_running', log: [stamp('librespot is not running. Start it in panel 5.')] });
    if (!librespotState.deviceId) return res.status(412).json({ error: 'device_id_unknown', log: [stamp('librespot is running but did not yet register with Spotify. Wait 5-10s and try again.')] });

    const deviceId = librespotState.deviceId;
    stamp(`Using librespot device id from logs: ${deviceId}`);

    stamp('Transferring playback to librespot…');
    const xfer = await spotifyPut('/me/player', t.access_token, { device_ids: [deviceId], play: false });
    stamp(`Transfer status ${xfer.status}${xfer.status >= 400 ? ' — ' + JSON.stringify(xfer.data) : ''}`);
    if (xfer.status === 429) return res.status(429).json({ error: 'rate_limited', retry_after: secondsUntilAllowed(), log });
    if (xfer.status === 403) return res.status(403).json({ error: 'premium_required', detail: xfer.data, log: [...log, stamp('403 — account is not Premium; /me/player needs Premium.')] });
    if (xfer.status === 404) return res.status(404).json({ error: 'device_unknown_to_spotify', detail: xfer.data, log: [...log, stamp('404 — Spotify does not yet know the device. librespot may have connected but not finished registration. Try again in 10s.')] });
    if (xfer.status >= 400) return res.status(xfer.status).json({ error: 'transfer_failed', detail: xfer.data, log });
    await new Promise(r => setTimeout(r, 1500));

    stamp(`Playing ${DEFAULT_TEST_TRACK}…`);
    const play = await spotifyPut(`/me/player/play?device_id=${encodeURIComponent(deviceId)}`, t.access_token, { uris: [DEFAULT_TEST_TRACK], position_ms: 0 });
    stamp(`Play status ${play.status}${play.status >= 400 ? ' — ' + JSON.stringify(play.data) : ''}`);
    if (play.status === 429) return res.status(429).json({ error: 'rate_limited', retry_after: secondsUntilAllowed(), log });
    if (play.status >= 400) return res.status(play.status).json({ error: 'play_failed', detail: play.data, log });

    stamp('Waiting 45s for playback to accumulate >30s of listening time…');
    await new Promise(r => setTimeout(r, 45_000));

    stamp('Polling /me/player/recently-played for the test track…');
    let found = null;
    for (let i = 0; i < 6 && !found; i++) {
      await new Promise(r => setTimeout(r, 5_000));
      const rec = await spotifyGet('/me/player/recently-played?limit=10', t.access_token);
      if (rec.status === 429) { stamp(`poll ${i + 1}/6 — rate limited (cached or fresh), retry_after=${secondsUntilAllowed()}s`); continue; }
      const items = rec.data?.items || [];
      found = items.find(it => it.track.uri === DEFAULT_TEST_TRACK && (Date.now() - new Date(it.played_at).getTime()) < 5 * 60_000);
      stamp(`poll ${i + 1}/6 — recent count=${items.length} match=${found ? 'YES' : 'no'}`);
    }
    if (!found) return res.status(200).json({ ok: false, reason: 'not_in_recently_played_yet', log });
    stamp(`SUCCESS — track is in recently-played at ${found.played_at}`);
    res.json({ ok: true, played_at: found.played_at, log });
  } finally {
    E2E_RUN_STATE.running = false;
  }
});

// --- librespot lifecycle (stubs filled in later phase) ---

const librespotState = {
  proc: null,
  status: 'idle', // idle | building | running | crashed
  log: [],
  buildLog: [],
  mercuryEvents: [],
  startedAt: null,
  /// librespot prints "successfully put connect state for <40-char hex> with
  /// connection-id …" once it has registered with Spotify. We capture that
  /// hex device id so the E2E runner can transfer to it without calling
  /// /me/player/devices (which is the most rate-limit-prone Web API call).
  deviceId: null,
};

const DEVICE_ID_REGEX = /successfully put connect state for ([a-f0-9]{40})/i;

function pushLog(line) {
  const stamped = `[${new Date().toISOString()}] ${line}`;
  librespotState.log.push(stamped);
  if (librespotState.log.length > 2000) librespotState.log.splice(0, librespotState.log.length - 2000);
  if (!librespotState.deviceId) {
    const m = line.match(DEVICE_ID_REGEX);
    if (m) {
      librespotState.deviceId = m[1];
    }
  }
  if (/event-service|TRACK_TRANSITION|NEW_PLAYBACK_ID|NEW_SESSION_ID|PlaybackMetrics/i.test(line)) {
    librespotState.mercuryEvents.push(stamped);
    if (librespotState.mercuryEvents.length > 500) librespotState.mercuryEvents.splice(0, 100);
  }
}

app.post('/librespot/build', (_req, res) => {
  if (librespotState.status === 'building' || librespotState.status === 'running') {
    return res.status(409).json({ error: 'busy', status: librespotState.status });
  }
  librespotState.status = 'building';
  librespotState.buildLog = [];
  const proc = spawn('cargo', ['build', '--release', '-p', 'librespot', '--no-default-features', '--features', 'rodio-backend,native-tls'], { cwd: REPO_ROOT });
  proc.stdout.on('data', d => librespotState.buildLog.push(d.toString()));
  proc.stderr.on('data', d => librespotState.buildLog.push(d.toString()));
  proc.on('close', code => {
    librespotState.status = code === 0 ? 'idle' : 'crashed';
    librespotState.buildLog.push(`[exit ${code}]`);
  });
  res.json({ ok: true });
});

app.post('/librespot/start', async (_req, res) => {
  if (librespotState.proc) return res.status(409).json({ error: 'already_running' });
  const t = await refreshIfNeeded();
  if (!t) return res.status(401).json({ error: 'not_authenticated' });
  const bin = path.join(REPO_ROOT, 'target', 'release', 'librespot');
  if (!fs.existsSync(bin)) return res.status(412).json({ error: 'binary_missing', hint: 'POST /librespot/build first' });
  const proc = spawn(bin, [
    '--name', 'librespot-playcount-test',
    '--bitrate', '160',
    '--backend', 'rodio',
    '--access-token', t.access_token,
    '--disable-credential-cache',
    '--verbose',
  ], { cwd: REPO_ROOT, env: { ...process.env, RUST_LOG: 'librespot=debug,librespot_core=debug,librespot_connect=debug,librespot_playback=debug' } });
  librespotState.proc = proc;
  librespotState.status = 'running';
  librespotState.startedAt = Date.now();
  librespotState.log = [];
  librespotState.mercuryEvents = [];
  librespotState.deviceId = null;
  proc.stdout.on('data', d => d.toString().split('\n').filter(Boolean).forEach(pushLog));
  proc.stderr.on('data', d => d.toString().split('\n').filter(Boolean).forEach(pushLog));
  proc.on('close', code => {
    pushLog(`[exit ${code}]`);
    librespotState.proc = null;
    librespotState.status = code === 0 ? 'idle' : 'crashed';
  });
  res.json({ ok: true });
});

app.post('/librespot/stop', (_req, res) => {
  if (!librespotState.proc) return res.json({ ok: true, note: 'not_running' });
  librespotState.proc.kill('SIGTERM');
  res.json({ ok: true });
});

app.get('/librespot/status', (_req, res) => {
  res.json({
    status: librespotState.status,
    pid: librespotState.proc?.pid ?? null,
    started_at: librespotState.startedAt,
    device_id: librespotState.deviceId,
    log_tail: librespotState.log.slice(-200),
    mercury_events: librespotState.mercuryEvents.slice(-100),
    build_log_tail: librespotState.buildLog.slice(-100),
  });
});

app.listen(PORT, HOST, () => {
  console.log(`Dashboard listening at http://${HOST}:${PORT}`);
});
