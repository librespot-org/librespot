use std;
use std::fmt;
use util::u128;
use byteorder::{BigEndian, ByteOrder};
use std::ascii::AsciiExt;

#[derive(Debug,Copy,Clone,PartialEq,Eq,Hash)]
pub struct SpotifyId(u128);

const BASE62_DIGITS: &'static [u8] =
    b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
const BASE16_DIGITS: &'static [u8] = b"0123456789abcdef";

impl SpotifyId {
    pub fn from_base16(id: &str) -> SpotifyId {
        assert!(id.is_ascii());
        let data = id.as_bytes();

        let mut n: u128 = u128::zero();
        for c in data {
            let d = BASE16_DIGITS.iter().position(|e| e == c).unwrap() as u8;
            n = n * u128::from(16);
            n = n + u128::from(d);
        }

        SpotifyId(n)
    }

    pub fn from_base62(id: &str) -> SpotifyId {
        assert!(id.is_ascii());
        let data = id.as_bytes();

        let mut n: u128 = u128::zero();
        for c in data {
            let d = BASE62_DIGITS.iter().position(|e| e == c).unwrap() as u8;
            n = n * u128::from(62);
            n = n + u128::from(d);
        }

        SpotifyId(n)
    }

    pub fn from_raw(data: &[u8]) -> SpotifyId {
        assert_eq!(data.len(), 16);

        let high = BigEndian::read_u64(&data[0..8]);
        let low = BigEndian::read_u64(&data[8..16]);

        SpotifyId(u128::from_parts(high, low))
    }

    pub fn to_base16(&self) -> String {
        let &SpotifyId(ref n) = self;
        let (high, low) = n.parts();

        let mut data = [0u8; 32];
        for i in 0..16 {
            data[31 - i] = BASE16_DIGITS[(low.wrapping_shr(4 * i as u32) & 0xF) as usize];
        }
        for i in 0..16 {
            data[15 - i] = BASE16_DIGITS[(high.wrapping_shr(4 * i as u32) & 0xF) as usize];
        }

        std::str::from_utf8(&data).unwrap().to_owned()
    }

    pub fn to_raw(&self) -> [u8; 16] {
        let &SpotifyId(ref n) = self;
        let (high, low) = n.parts();

        let mut data = [0u8; 16];

        BigEndian::write_u64(&mut data[0..8], high);
        BigEndian::write_u64(&mut data[8..16], low);

        data
    }
}

#[derive(Copy,Clone,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub struct FileId(pub [u8; 20]);

impl FileId {
    pub fn to_base16(&self) -> String {
        self.0
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<String>>()
            .concat()
    }
}

impl fmt::Debug for FileId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("FileId").field(&self.to_base16()).finish()
    }
}

impl fmt::Display for FileId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.to_base16())
    }
}
