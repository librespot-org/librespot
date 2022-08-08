use std::io;

use aes::cipher::{KeyIvInit, StreamCipher, StreamCipherSeek};

type Aes128Ctr = ctr::Ctr128BE<aes::Aes128>;

use librespot_core::audio_key::AudioKey;

const AUDIO_AESIV: [u8; 16] = [
    0x72, 0xe0, 0x67, 0xfb, 0xdd, 0xcb, 0xcf, 0x77, 0xeb, 0xe8, 0xbc, 0x64, 0x3f, 0x63, 0x0d, 0x93,
];

pub struct AudioDecrypt<T: io::Read> {
    // a `None` cipher is a convenience to make `AudioDecrypt` pass files unaltered
    cipher: Option<Aes128Ctr>,
    reader: T,
}

impl<T: io::Read> AudioDecrypt<T> {
    pub fn new(key: Option<AudioKey>, reader: T) -> AudioDecrypt<T> {
        let cipher = if let Some(key) = key {
            Aes128Ctr::new_from_slices(&key.0, &AUDIO_AESIV).ok()
        } else {
            // some files are unencrypted
            None
        };

        AudioDecrypt { cipher, reader }
    }
}

impl<T: io::Read> io::Read for AudioDecrypt<T> {
    fn read(&mut self, output: &mut [u8]) -> io::Result<usize> {
        let len = self.reader.read(output)?;

        if let Some(ref mut cipher) = self.cipher {
            cipher.apply_keystream(&mut output[..len]);
        }

        Ok(len)
    }
}

impl<T: io::Read + io::Seek> io::Seek for AudioDecrypt<T> {
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        let newpos = self.reader.seek(pos)?;

        if let Some(ref mut cipher) = self.cipher {
            cipher.seek(newpos);
        }

        Ok(newpos)
    }
}
