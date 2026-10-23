pub use imp::if_indices_for_ips;

#[cfg(unix)]
mod imp {
    use std::{
        collections::{BTreeMap, BTreeSet},
        ffi::CStr,
        io,
        net::{IpAddr, Ipv4Addr, Ipv6Addr},
        ptr,
    };

    pub fn if_indices_for_ips(ips: &[IpAddr]) -> Result<Vec<i32>, io::Error> {
        let ifaddrs = InterfaceAddresses::load()?;
        let mut indices_by_ip = BTreeMap::<IpAddr, BTreeSet<i32>>::new();
        let mut current = ifaddrs.head();

        while let Some(interface) = InterfaceAddress::from_ptr(current) {
            if let (Some(ip), Some(index)) = (interface.ip_addr(), interface.index()) {
                indices_by_ip.entry(ip).or_default().insert(index);
            }
            current = interface.next();
        }

        let mut matched_indices = BTreeSet::new();
        for ip in ips {
            if let Some(indices) = indices_by_ip.get(ip) {
                matched_indices.extend(indices.iter().copied());
            } else {
                log::warn!("Ignoring unrecognised zeroconf IP {}", ip);
            }
        }

        Ok(matched_indices.into_iter().collect())
    }

    struct InterfaceAddresses(*mut libc::ifaddrs);

    impl InterfaceAddresses {
        fn load() -> Result<Self, io::Error> {
            let mut ifaddrs = ptr::null_mut();
            let result = unsafe {
                // SAFETY: `getifaddrs` initializes `ifaddrs` on success and does not retain the pointer.
                libc::getifaddrs(&mut ifaddrs)
            };

            if result == 0 {
                Ok(Self(ifaddrs))
            } else {
                Err(io::Error::last_os_error())
            }
        }

        fn head(&self) -> *mut libc::ifaddrs {
            self.0
        }
    }

    impl Drop for InterfaceAddresses {
        fn drop(&mut self) {
            if !self.0.is_null() {
                unsafe {
                    // SAFETY: `self.0` was returned by `getifaddrs` and is freed exactly once here.
                    libc::freeifaddrs(self.0);
                }
            }
        }
    }

    struct InterfaceAddress<'a>(&'a libc::ifaddrs);

    impl<'a> InterfaceAddress<'a> {
        fn from_ptr(ptr: *mut libc::ifaddrs) -> Option<Self> {
            unsafe {
                // SAFETY: The `ifaddrs` list is valid for the lifetime of `InterfaceAddresses`.
                ptr.as_ref().map(Self)
            }
        }

        fn next(&self) -> *mut libc::ifaddrs {
            self.0.ifa_next
        }

        fn ip_addr(&self) -> Option<IpAddr> {
            sockaddr_to_ip_addr(self.0.ifa_addr)
        }

        fn index(&self) -> Option<i32> {
            if self.0.ifa_name.is_null() {
                return None;
            }

            let name = unsafe {
                // SAFETY: `ifa_name` is a valid, null-terminated C string for a live `ifaddrs` entry.
                CStr::from_ptr(self.0.ifa_name)
            };
            let raw_index = unsafe {
                // SAFETY: `ifa_name` points to the current interface name for this `ifaddrs` entry.
                libc::if_nametoindex(self.0.ifa_name)
            };

            if raw_index == 0 {
                log::warn!(
                    "Failed to resolve interface index for {}: {}",
                    name.to_string_lossy(),
                    io::Error::last_os_error()
                );
                return None;
            }

            match i32::try_from(raw_index) {
                Ok(index) => Some(index),
                Err(_) => {
                    log::warn!(
                        "Ignoring interface {} because index {} does not fit in i32",
                        name.to_string_lossy(),
                        raw_index
                    );
                    None
                }
            }
        }
    }

    fn sockaddr_to_ip_addr(sockaddr: *const libc::sockaddr) -> Option<IpAddr> {
        if sockaddr.is_null() {
            return None;
        }

        let family = unsafe {
            // SAFETY: `sockaddr` points to a valid socket address owned by the `ifaddrs` list.
            (*sockaddr).sa_family as libc::c_int
        };

        match family {
            libc::AF_INET => {
                let sockaddr = unsafe {
                    // SAFETY: The family check above guarantees the cast to `sockaddr_in`.
                    &*sockaddr.cast::<libc::sockaddr_in>()
                };
                Some(IpAddr::V4(Ipv4Addr::from(u32::from_be(
                    sockaddr.sin_addr.s_addr,
                ))))
            }
            libc::AF_INET6 => {
                let sockaddr = unsafe {
                    // SAFETY: The family check above guarantees the cast to `sockaddr_in6`.
                    &*sockaddr.cast::<libc::sockaddr_in6>()
                };
                Some(IpAddr::V6(Ipv6Addr::from(sockaddr.sin6_addr.s6_addr)))
            }
            _ => None,
        }
    }

    #[cfg(test)]
    mod tests {
        use super::if_indices_for_ips;
        use std::net::IpAddr;

        #[test]
        fn loopback_ip_resolves_to_positive_index() {
            let indices = if_indices_for_ips(&["127.0.0.1".parse::<IpAddr>().unwrap()]).unwrap();

            assert!(!indices.is_empty());
            assert!(indices.iter().all(|index| *index > 0));
        }

        #[test]
        fn unknown_ip_is_skipped_without_error() {
            let indices = if_indices_for_ips(&["192.0.2.1".parse::<IpAddr>().unwrap()]).unwrap();

            assert!(indices.is_empty());
        }
    }
}

#[cfg(not(unix))]
mod imp {
    use std::{io, net::IpAddr};

    pub fn if_indices_for_ips(_ips: &[IpAddr]) -> Result<Vec<i32>, io::Error> {
        Err(io::Error::other(
            "Avahi interface resolution is only supported on Unix platforms",
        ))
    }
}
