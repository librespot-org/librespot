use hyper;
use hyper::net::Openssl;
use hyper::net::NetworkListener;
use hyper::server::Request;
use hyper::server::Response;
use hyper::uri::RequestUri;
use hyper::header::AccessControlAllowOrigin;
use openssl::ssl::{SslContext, SslMethod, SSL_VERIFY_NONE};
use openssl::ssl::error::SslError;
use openssl::crypto::pkey::PKey;
use openssl::x509::X509;
use rand::{self, Rng};
use rustc_serialize::json;
use std::collections::BTreeMap;
use std::io::{Cursor, Read};
use std::sync::{mpsc, Arc, Mutex};
use url;

use protocol::authentication::AuthenticationType;
use authentication::Credentials;

static SPOTILOCAL_CERT : &'static [u8] = include_bytes!("data/spotilocal.cert");
static SPOTILOCAL_KEY : &'static [u8] = include_bytes!("data/spotilocal.key");

fn spotilocal_ssl_context() -> Result<Openssl, SslError> {
    let cert = try!(X509::from_pem(&mut Cursor::new(SPOTILOCAL_CERT)));
    let key = try!(PKey::private_key_from_pem(&mut Cursor::new(SPOTILOCAL_KEY)));

    let mut ctx = try!(SslContext::new(SslMethod::Sslv23));
    try!(ctx.set_cipher_list("DEFAULT"));
    try!(ctx.set_private_key(&key));
    try!(ctx.set_certificate(&cert));
    ctx.set_verify(SSL_VERIFY_NONE, None);
    Ok(Openssl { context: Arc::new(ctx) })
}

struct ServerHandler {
    token_tx: Mutex<mpsc::Sender<String>>,
    csrf: String,
}

impl ServerHandler {
    fn handle_login(&self, params: &BTreeMap<String, String>) -> hyper::status::StatusCode {
        let token = params.get("access_token").unwrap();
        let csrf = params.get("csrf").unwrap();

        if *csrf == self.csrf {
            self.token_tx.lock().unwrap().send(token.to_owned()).unwrap();
            hyper::status::StatusCode::Ok
        } else {
            hyper::status::StatusCode::Forbidden
        }
    }
}

impl hyper::server::Handler for ServerHandler {
    fn handle<'a, 'k>(&'a self, request: Request<'a, 'k>, mut response: Response<'a, hyper::net::Fresh>) {
        response.headers_mut().set(AccessControlAllowOrigin::Value("https://login.spotify.com".to_owned()));
        *response.status_mut() = if let RequestUri::AbsolutePath(path) = request.uri {
            let (path, query, _) = url::parse_path(&path).unwrap();
            let params = query.map_or(vec![], |q| url::form_urlencoded::parse(q.as_bytes()))
                              .into_iter().collect::<BTreeMap<_,_>>();

            if request.method == hyper::method::Method::Get && path == vec!["login", "facebook_login_sso.json"] {
                self.handle_login(&params)
            } else {
                hyper::status::StatusCode::NotFound
            }
        } else {
            hyper::status::StatusCode::NotFound
        }
    }
}

fn facebook_get_me_id(token: &str) -> Result<String, ()> {
    let url = format!("https://graph.facebook.com/me?fields=id&access_token={}", token);

    let client = hyper::Client::new();
    let mut response = client.get(&url).send().unwrap();
    let mut body = String::new();
    response.read_to_string(&mut body).unwrap();

    let mut result : BTreeMap<String, String> = json::decode(&body).unwrap();
    Ok(result.remove("id").unwrap())
}

pub fn facebook_login() -> Result<Credentials, ()> {
    let (tx, rx) = mpsc::channel();

    let csrf = rand::thread_rng().gen_ascii_chars().take(32).collect::<String>();
    let handler = ServerHandler {
        token_tx: Mutex::new(tx),
        csrf: csrf.clone()
    };

    let ssl = spotilocal_ssl_context().unwrap();

    let mut listener = hyper::net::HttpsListener::new("127.0.0.1:0", ssl).unwrap();
    let port = listener.local_addr().unwrap().port();

    let mut server = hyper::Server::new(listener).handle(handler).unwrap();

    println!("Logging in using Facebook, please visit https://login.spotify.com/login-facebook-sso/?csrf={}&port={} in your browser.",
             csrf, port);

    let token = rx.recv().unwrap();
    let user_id = facebook_get_me_id(&token).unwrap();
    let cred = Credentials {
        username: user_id,
        auth_type: AuthenticationType::AUTHENTICATION_FACEBOOK_TOKEN,
        auth_data: token.as_bytes().to_owned(),
    };

    server.close().unwrap();
    Ok(cred)
}
