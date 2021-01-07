// Passthrough decoder for librespot
//
// Copyright (c) 2021 Philippe <philippe_44@outlook.com>
// and contributors. All rights reserved.
// Licensed under MIT license, or Apache 2 license,
// at your option. Please see the LICENSE file
// attached to this source distribution for details.

use super::{AudioDecoder, AudioError, AudioPacket};
use ogg::{OggReadError, Packet, PacketReader, PacketWriteEndInfo, PacketWriter};
use std::io::{Error, Read, Seek, Write};
use std::sync::{Arc, Mutex};
use std::{fmt, slice};

// A few macros to easily change the type of inner storage, including moving
// from Arc<Mutex>> to Rc<RefCell>
// When inner_mut will be available in PacketWriter, the WrappedWriter and
// Passthrough::data can be removed and access to written data is simply
// self.wtr.inner_mut()
type BufCell = Arc<Mutex<Vec<u8>>>;
macro_rules! to_buf {
    ($x:expr) => {
        $x.data.lock().unwrap()
    };
}

// a wrapper to allow interior mutation if inner_mut is not in PacketWriter
pub struct WrappedWriter {
    data: BufCell,
}

impl WrappedWriter {
    pub fn new(data: BufCell) -> Self {
        return WrappedWriter { data };
    }
}

impl Write for WrappedWriter {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        let mut data = to_buf!(self);
        data.write(buf)
    }
    fn flush(&mut self) -> Result<(), Error> {
        Ok(())
    }
}

fn read_headers<T: Read + Seek>(
    rdr: &mut PacketReader<T>,
) -> Result<(Vec<u8>, u32), PassthroughError> {
    let mut header = PacketWriter::new(Vec::new());
    let mut stream_serial = 0;

    // search for ident, comment, setup
    add_header(
        1,
        &mut header,
        rdr,
        &mut stream_serial,
        PacketWriteEndInfo::EndPage,
    )?;
    add_header(
        3,
        &mut header,
        rdr,
        &mut stream_serial,
        PacketWriteEndInfo::NormalPacket,
    )?;
    add_header(
        5,
        &mut header,
        rdr,
        &mut stream_serial,
        PacketWriteEndInfo::EndPage,
    )?;

    // remove un-needed packets
    rdr.delete_unread_packets();
    return Ok((header.into_inner(), stream_serial));
}

fn add_header<T>(
    code: u8,
    header: &mut PacketWriter<Vec<u8>>,
    rdr: &mut PacketReader<T>,
    stream_serial: &mut u32,
    info: PacketWriteEndInfo,
) -> Result<(), PassthroughError>
where
    T: Read + Seek,
{
    let mut pck: Packet = rdr.read_packet_expected()?;
    while *stream_serial != 0 && pck.stream_serial() != *stream_serial {
        pck = rdr.read_packet_expected()?;
    }

    *stream_serial = pck.stream_serial();

    let pkt_type = pck.data[0];
    debug!("Vorbis header type{}", &pkt_type);

    // all headers are mandatory
    if pkt_type != code {
        return Err(PassthroughError::NotVorbisError);
    }

    let absgp_page = pck.absgp_page();
    header.write_packet(
        pck.data.into_boxed_slice(),
        *stream_serial,
        info,
        absgp_page,
    )?;

    return Ok(());
}

static mut BASEGP_PAGE: u64 = 0;

pub struct PassthroughDecoder<R: Read + Seek> {
    rdr: PacketReader<R>,
    wtr: PacketWriter<WrappedWriter>,
    data: BufCell,
    header: Vec<u8>,
    prime: bool,
}

impl<R: Read + Seek> PassthroughDecoder<R> {
    /// Constructs a new Decoder from a given implementation of `Read + Seek`.
    pub fn new(rdr: R) -> Result<Self, PassthroughError> {
        info!("Starting passthrough track");
        let mut rdr = PacketReader::new(rdr);
        let data: BufCell = Arc::new(Mutex::new(Vec::new()));
        let wtr = PacketWriter::new(WrappedWriter::new(Arc::clone(&data)));
        let (header, _stream_serial) = read_headers(&mut rdr)?;

        return Ok(PassthroughDecoder {
            rdr,
            wtr,
            header,
            data,
            prime: true,
        });
    }

    /// Seeks to the specified absolute granule position, with a page granularity.
    ///
    /// The granularity is per-page, and the obtained position is
    /// then <= the seeked absgp.
    ///
    /// In the case of ogg/vorbis, the absolute granule position is given
    /// as number of PCM samples, on a per channel basis.
    pub fn seek_absgp_pg(&mut self, absgp: u64) -> Result<(), PassthroughError> {
        (self.rdr.seek_absgp(None, absgp))?;
        Ok(())
    }
}

impl<R: Read + Seek> AudioDecoder for PassthroughDecoder<R> {
    fn seek(&mut self, ms: i64) -> Result<(), AudioError> {
        let absgp = ms * 44100 / 1000;
        info!("Seeking to {}", ms);
        self.seek_absgp_pg(absgp as u64)?;
        self.prime = true;

        Ok(())
    }

    fn next_packet(&mut self) -> Result<Option<AudioPacket>, AudioError> {
        // need to send headers first
        if self.prime {
            self.prime = false;
            let mut data = to_buf!(self);
            data.clear();
            data.append(&mut self.header.clone());
        }

        loop {
            let pck = match self.rdr.read_packet() {
                Ok(Some(pck)) => pck,

                Ok(None) | Err(OggReadError::NoCapturePatternFound) => {
                    let mut data = to_buf!(self);
                    let len = data.len();

                    // we might have one trailing byte
                    if len == 1 {
                        info!("ending packet of one byte");
                        let last = data.drain(..);
                        return Ok(Some(AudioPacket(vec![i16::from(last.as_slice()[0])])));
                    } else if len > 0 {
                        warn!("unexpected end of streaming {:?}", &len);
                        return Ok(None);
                    } else {
                        info!("normal end of streaming");
                        return Ok(None);
                    }
                }

                Err(err) => return Err(err.into()),
            };

            // then do normal packet processing
            let inf = if pck.last_in_stream() {
                PacketWriteEndInfo::EndStream
            } else if pck.last_in_page() {
                PacketWriteEndInfo::EndPage
            } else {
                PacketWriteEndInfo::NormalPacket
            };

            // NB: we don't handle multiple streams (change of serial)
            let stream_serial = pck.stream_serial();
            let mut absgp_page = pck.absgp_page();

            // stich streams if needed
            unsafe {
                absgp_page += BASEGP_PAGE;
                if inf == PacketWriteEndInfo::EndStream {
                    BASEGP_PAGE = absgp_page;
                }
            }

            self.wtr
                .write_packet(pck.data.into_boxed_slice(), stream_serial, inf, absgp_page)?;

            // we need an even number of bytes to map to i16
            let mut data = to_buf!(self);
            let len = data.len() / 2;

            if len > 0 {
                //eprintln!("O-Len: {:?}", &len);
                let data16 = unsafe { slice::from_raw_parts(data.as_ptr() as *const i16, len) };
                let result = AudioPacket(Vec::<i16>::from(data16));

                data.drain(0..2 * len);
                data.shrink_to_fit();

                return Ok(Some(result));
            }
        }
    }
}

#[derive(Debug)]
pub enum PassthroughError {
    NotVorbisError,
    OggError(OggReadError),
    IOError(Error),
}

impl From<ogg::OggReadError> for PassthroughError {
    fn from(err: OggReadError) -> PassthroughError {
        PassthroughError::OggError(err)
    }
}

impl From<Error> for PassthroughError {
    fn from(err: Error) -> PassthroughError {
        PassthroughError::IOError(err)
    }
}

impl fmt::Display for PassthroughError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}

impl From<ogg::OggReadError> for AudioError {
    fn from(err: OggReadError) -> AudioError {
        AudioError::PassthroughError(PassthroughError::OggError(err))
    }
}

impl From<Error> for AudioError {
    fn from(err: Error) -> AudioError {
        AudioError::PassthroughError(PassthroughError::IOError(err))
    }
}

impl From<PassthroughError> for AudioError {
    fn from(err: PassthroughError) -> AudioError {
        AudioError::PassthroughError(err)
    }
}
