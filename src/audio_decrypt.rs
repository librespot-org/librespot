use crypto::aes;
use crypto::symmetriccipher::SynchronousStreamCipher;
use readall::ReadAllExt;
use std::io;

use audio_key::AudioKey;

const AUDIO_AESIV : &'static [u8] = &[
    0x72,0xe0,0x67,0xfb,0xdd,0xcb,0xcf,0x77,0xeb,0xe8,0xbc,0x64,0x3f,0x63,0x0d,0x93,
];

pub struct AudioDecrypt<T : io::Read> {
    cipher: Box<SynchronousStreamCipher + 'static>,
    key: AudioKey,
    reader: T,
}

impl <T : io::Read> AudioDecrypt<T> {
    pub fn new(key: AudioKey, mut reader: T) -> AudioDecrypt<T> {
        let mut cipher = aes::ctr(aes::KeySize::KeySize128,
                              &key,
                              AUDIO_AESIV);

        let mut buf = [0; 0xa7];
        let mut buf2 = [0; 0xa7];
        reader.read_all(&mut buf).unwrap();
        cipher.process(&buf, &mut buf2);

        AudioDecrypt {
            cipher: cipher,
            key: key,
            reader: reader,
        }
    }
}

impl <T : io::Read> io::Read for AudioDecrypt<T> {
    fn read(&mut self, output: &mut [u8]) -> io::Result<usize> {
        let mut buffer = vec![0u8; output.len()];
        let len = try!(self.reader.read(&mut buffer));

        self.cipher.process(&buffer[..len], &mut output[..len]);

        Ok(len)
    }
}

impl <T : io::Read> io::Seek for AudioDecrypt<T> {
    fn seek(&mut self, _pos: io::SeekFrom) -> io::Result<u64> {
        Err(io::Error::new(io::ErrorKind::Other, "Cannot seek"))
    }
}


