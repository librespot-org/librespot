use hyper::net::Openssl;
use openssl::crypto::pkey::PKey;
use openssl::ssl::{SslContext, SslMethod, SSL_VERIFY_NONE};
use openssl::ssl::error::SslError;
use openssl::x509::X509;
use std::io::Cursor;
use std::sync::Arc;

static SPOTILOCAL_CERT : &'static [u8] = include_bytes!("data/spotilocal.cert");
static SPOTILOCAL_KEY : &'static [u8] = include_bytes!("data/spotilocal.key");

pub fn ssl_context() -> Result<Openssl, SslError> {
    let cert = try!(X509::from_pem(&mut Cursor::new(SPOTILOCAL_CERT)));
    let key = try!(PKey::private_key_from_pem(&mut Cursor::new(SPOTILOCAL_KEY)));

    let mut ctx = try!(SslContext::new(SslMethod::Sslv23));
    try!(ctx.set_cipher_list("DEFAULT"));
    try!(ctx.set_private_key(&key));
    try!(ctx.set_certificate(&cert));
    ctx.set_verify(SSL_VERIFY_NONE, None);
    Ok(Openssl { context: Arc::new(ctx) })
}
