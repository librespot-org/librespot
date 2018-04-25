use crypto::aes;
use crypto::symmetriccipher::SynchronousStreamCipher;
use num_bigint::BigUint;
use num_traits::FromPrimitive;
use std::io;
use std::ops::Add;

use core::audio_key::AudioKey;

const AUDIO_AESIV: &'static [u8] = &[
    0x72, 0xe0, 0x67, 0xfb, 0xdd, 0xcb, 0xcf, 0x77, 0xeb, 0xe8, 0xbc, 0x64, 0x3f, 0x63, 0x0d, 0x93,
];

pub struct AudioDecrypt<T: io::Read> {
    cipher: Box<SynchronousStreamCipher + 'static>,
    key: AudioKey,
    reader: T,
}

impl<T: io::Read> AudioDecrypt<T> {
    pub fn new(key: AudioKey, reader: T) -> AudioDecrypt<T> {
        let cipher = aes::ctr(aes::KeySize::KeySize128, &key.0, AUDIO_AESIV);
        AudioDecrypt {
            cipher: cipher,
            key: key,
            reader: reader,
        }
    }
}

impl<T: io::Read> io::Read for AudioDecrypt<T> {
    fn read(&mut self, output: &mut [u8]) -> io::Result<usize> {
        let mut buffer = vec![0u8; output.len()];
        let len = try!(self.reader.read(&mut buffer));

        self.cipher.process(&buffer[..len], &mut output[..len]);

        Ok(len)
    }
}

impl<T: io::Read + io::Seek> io::Seek for AudioDecrypt<T> {
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        let newpos = try!(self.reader.seek(pos));
        let skip = newpos % 16;

        let iv = BigUint::from_bytes_be(AUDIO_AESIV)
            .add(BigUint::from_u64(newpos / 16).unwrap())
            .to_bytes_be();
        self.cipher = aes::ctr(aes::KeySize::KeySize128, &self.key.0, &iv);

        let buf = vec![0u8; skip as usize];
        let mut buf2 = vec![0u8; skip as usize];
        self.cipher.process(&buf, &mut buf2);

        Ok(newpos as u64)
    }
}
