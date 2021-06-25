// Passthrough decoder for librespot
use super::{AudioDecoder, AudioError, AudioPacket};
use crate::SAMPLE_RATE;
use ogg::{OggReadError, Packet, PacketReader, PacketWriteEndInfo, PacketWriter};
use std::fmt;
use std::io::{Read, Seek};
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};

fn get_header<T>(code: u8, rdr: &mut PacketReader<T>) -> Result<Box<[u8]>, PassthroughError>
where
    T: Read + Seek,
{
    let pck: Packet = rdr.read_packet_expected()?;

    let pkt_type = pck.data[0];
    debug!("Vorbis header type {}", &pkt_type);

    if pkt_type != code {
        return Err(PassthroughError(OggReadError::InvalidData));
    }

    Ok(pck.data.into_boxed_slice())
}

pub struct PassthroughDecoder<R: Read + Seek> {
    rdr: PacketReader<R>,
    wtr: PacketWriter<Vec<u8>>,
    eos: bool,
    bos: bool,
    ofsgp_page: u64,
    stream_serial: u32,
    ident: Box<[u8]>,
    comment: Box<[u8]>,
    setup: Box<[u8]>,
}

pub struct PassthroughError(ogg::OggReadError);

impl<R: Read + Seek> PassthroughDecoder<R> {
    /// Constructs a new Decoder from a given implementation of `Read + Seek`.
    pub fn new(rdr: R) -> Result<Self, PassthroughError> {
        let mut rdr = PacketReader::new(rdr);
        let stream_serial = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u32;

        info!("Starting passthrough track with serial {}", stream_serial);

        // search for ident, comment, setup
        let ident = get_header(1, &mut rdr)?;
        let comment = get_header(3, &mut rdr)?;
        let setup = get_header(5, &mut rdr)?;

        // remove un-needed packets
        rdr.delete_unread_packets();

        Ok(PassthroughDecoder {
            rdr,
            wtr: PacketWriter::new(Vec::new()),
            ofsgp_page: 0,
            stream_serial,
            ident,
            comment,
            setup,
            eos: false,
            bos: false,
        })
    }
}

impl<R: Read + Seek> AudioDecoder for PassthroughDecoder<R> {
    fn seek(&mut self, ms: i64) -> Result<(), AudioError> {
        info!("Seeking to {}", ms);

        // add an eos to previous stream if missing
        if self.bos && !self.eos {
            match self.rdr.read_packet() {
                Ok(Some(pck)) => {
                    let absgp_page = pck.absgp_page() - self.ofsgp_page;
                    self.wtr
                        .write_packet(
                            pck.data.into_boxed_slice(),
                            self.stream_serial,
                            PacketWriteEndInfo::EndStream,
                            absgp_page,
                        )
                        .unwrap();
                }
                _ => warn! {"Cannot write EoS after seeking"},
            };
        }

        self.eos = false;
        self.bos = false;
        self.ofsgp_page = 0;
        self.stream_serial += 1;

        // hard-coded to 44.1 kHz
        match self.rdr.seek_absgp(
            None,
            Duration::from_millis(ms as u64 * SAMPLE_RATE as u64).as_secs(),
        ) {
            Ok(_) => {
                // need to set some offset for next_page()
                let pck = self.rdr.read_packet().unwrap().unwrap();
                self.ofsgp_page = pck.absgp_page();
                debug!("Seek to offset page {}", self.ofsgp_page);
                Ok(())
            }
            Err(err) => Err(AudioError::PassthroughError(err.into())),
        }
    }

    fn next_packet(&mut self) -> Result<Option<AudioPacket>, AudioError> {
        // write headers if we are (re)starting
        if !self.bos {
            self.wtr
                .write_packet(
                    self.ident.clone(),
                    self.stream_serial,
                    PacketWriteEndInfo::EndPage,
                    0,
                )
                .unwrap();
            self.wtr
                .write_packet(
                    self.comment.clone(),
                    self.stream_serial,
                    PacketWriteEndInfo::NormalPacket,
                    0,
                )
                .unwrap();
            self.wtr
                .write_packet(
                    self.setup.clone(),
                    self.stream_serial,
                    PacketWriteEndInfo::EndPage,
                    0,
                )
                .unwrap();
            self.bos = true;
            debug!("Wrote Ogg headers");
        }

        loop {
            let pck = match self.rdr.read_packet() {
                Ok(Some(pck)) => pck,
                Ok(None) | Err(OggReadError::NoCapturePatternFound) => {
                    info!("end of streaming");
                    return Ok(None);
                }
                Err(err) => return Err(AudioError::PassthroughError(err.into())),
            };

            let pckgp_page = pck.absgp_page();

            // skip till we have audio and a calculable granule position
            if pckgp_page == 0 || pckgp_page == self.ofsgp_page {
                continue;
            }

            // set packet type
            let inf = if pck.last_in_stream() {
                self.eos = true;
                PacketWriteEndInfo::EndStream
            } else if pck.last_in_page() {
                PacketWriteEndInfo::EndPage
            } else {
                PacketWriteEndInfo::NormalPacket
            };

            self.wtr
                .write_packet(
                    pck.data.into_boxed_slice(),
                    self.stream_serial,
                    inf,
                    pckgp_page - self.ofsgp_page,
                )
                .unwrap();

            let data = self.wtr.inner_mut();

            if !data.is_empty() {
                let result = AudioPacket::OggData(std::mem::take(data));
                return Ok(Some(result));
            }
        }
    }
}

impl fmt::Debug for PassthroughError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl From<ogg::OggReadError> for PassthroughError {
    fn from(err: OggReadError) -> PassthroughError {
        PassthroughError(err)
    }
}

impl fmt::Display for PassthroughError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}
