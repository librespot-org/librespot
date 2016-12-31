use hyper;
use hyper::header::AccessControlAllowOrigin;
use hyper::net::HttpsConnector;
use hyper::net::NetworkListener;
use hyper::server::Request;
use hyper::server::Response;
use hyper::uri::RequestUri;
use hyper_rustls;
use rand::{self, Rng};
use rustls;
use serde_json;
use std::collections::BTreeMap;
use std::io::Read;
use std::sync::{mpsc, Mutex};
use url;

use protocol::authentication::AuthenticationType;
use authentication::Credentials;
use spotilocal::{SPOTILOCAL_CERT, SPOTILOCAL_KEY};

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

            debug!("{:?} {:?} {:?}", request.method, path, params);

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

    let connector = HttpsConnector::new(hyper_rustls::TlsClient::new());
    let client = hyper::Client::with_connector(connector);

    let mut response = client.get(&url).send().unwrap();
    let mut body = String::new();
    response.read_to_string(&mut body).unwrap();

    let mut result : BTreeMap<String, String> = serde_json::from_str(&body).unwrap();
    Ok(result.remove("id").unwrap())
}

pub fn facebook_login() -> Result<Credentials, ()> {
    let (tx, rx) = mpsc::channel();

    let csrf = rand::thread_rng().gen_ascii_chars().take(32).collect::<String>();
    let handler = ServerHandler {
        token_tx: Mutex::new(tx),
        csrf: csrf.clone()
    };

    let mut cert_data = SPOTILOCAL_CERT;
    let mut key_data = SPOTILOCAL_KEY;
    let certs = rustls::internal::pemfile::certs(&mut cert_data).unwrap();
    let key = rustls::internal::pemfile::rsa_private_keys(&mut key_data).unwrap().swap_remove(0);
    let tls = hyper_rustls::TlsServer::new(certs, key);

    let mut listener = hyper::net::HttpsListener::new("127.0.0.1:0", tls).unwrap();
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
