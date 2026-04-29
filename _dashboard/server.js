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

async function spotifyGet(p, accessToken) {
  const r = await fetch(`https://api.spotify.com/v1${p}`, {
    headers: { Authorization: `Bearer ${accessToken}` },
  });
  const text = await r.text();
  let data = null;
  try { data = text ? JSON.parse(text) : null; } catch { data = { _raw: text }; }
  return { status: r.status, data };
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

app.get('/auth/status', async (_req, res) => {
  const t = await refreshIfNeeded();
  if (!t) return res.json({ authenticated: false });
  const me = await spotifyGet('/me', t.access_token);
  res.json({
    authenticated: true,
    expires_at: t.obtained_at + t.expires_in * 1000,
    scopes: (t.scope || '').split(' '),
    me: me.status === 200 ? { id: me.data.id, display_name: me.data.display_name, product: me.data.product } : null,
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

// --- librespot lifecycle (stubs filled in later phase) ---

const librespotState = {
  proc: null,
  status: 'idle', // idle | building | running | crashed
  log: [],
  buildLog: [],
  mercuryEvents: [],
  startedAt: null,
};

function pushLog(line) {
  const stamped = `[${new Date().toISOString()}] ${line}`;
  librespotState.log.push(stamped);
  if (librespotState.log.length > 2000) librespotState.log.splice(0, librespotState.log.length - 2000);
  // Naive: capture lines that look like our event-service traffic.
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
  const proc = spawn('cargo', ['build', '--release', '-p', 'librespot', '--no-default-features', '--features', 'rodio-backend'], { cwd: REPO_ROOT });
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
    '--verbose',
  ], { cwd: REPO_ROOT, env: { ...process.env, RUST_LOG: 'librespot=debug,librespot_playback=trace' } });
  librespotState.proc = proc;
  librespotState.status = 'running';
  librespotState.startedAt = Date.now();
  librespotState.log = [];
  librespotState.mercuryEvents = [];
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
    log_tail: librespotState.log.slice(-200),
    mercury_events: librespotState.mercuryEvents.slice(-100),
    build_log_tail: librespotState.buildLog.slice(-100),
  });
});

app.listen(PORT, HOST, () => {
  console.log(`Dashboard listening at http://${HOST}:${PORT}`);
});
