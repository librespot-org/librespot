#[cfg(feature = "dns-sd")]
pub use dns_sd::*;
#[cfg(not(feature = "dns-sd"))]
pub use self::stub::*;

#[cfg(not(feature = "dns-sd"))]
pub mod stub {
    use std;
    use std::io::Write;

    #[derive(Debug)]
    pub struct DNSService;

    pub type DNSError = ();

    impl DNSService {
        pub fn register(_: Option<&str>,
                        _: &str,
                        _: Option<&str>,
                        _: Option<&str>,
                        _: u16,
                        _: &[&str])
                        -> std::result::Result<DNSService, DNSError> {
            writeln!(&mut std::io::stderr(),
                     "WARNING: dns-sd is not enabled. Service will probably not be visible").unwrap();
            Ok(DNSService)
        }
    }
}
