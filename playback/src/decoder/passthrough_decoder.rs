// Passthrough decoder for librespot
use super::{AudioDecoder, AudioPacket, DecoderError, DecoderResult};
use ogg::{OggReadError, Packet, PacketReader, PacketWriteEndInfo, PacketWriter};
use std::io::{Read, Seek};
use std::time::{SystemTime, UNIX_EPOCH};

fn get_header<T>(code: u8, rdr: &mut PacketReader<T>) -> DecoderResult<Box<[u8]>>
where
    T: Read + Seek,
{
    let pck: Packet = rdr
        .read_packet_expected()
        .map_err(|e| DecoderError::PassthroughDecoder(e.to_string()))?;

    let pkt_type = pck.data[0];
    debug!("Vorbis header type {}", &pkt_type);

    if pkt_type != code {
        return Err(DecoderError::PassthroughDecoder("Invalid Data".to_string()));
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

impl<R: Read + Seek> PassthroughDecoder<R> {
    /// Constructs a new Decoder from a given implementation of `Read + Seek`.
    pub fn new(rdr: R) -> DecoderResult<Self> {
        let mut rdr = PacketReader::new(rdr);
        let since_epoch = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| DecoderError::PassthroughDecoder(e.to_string()))?;
        let stream_serial = since_epoch.as_millis() as u32;

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
    fn seek(&mut self, absgp: u64) -> DecoderResult<()> {
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
                        .map_err(|e| DecoderError::PassthroughDecoder(e.to_string()))?;
                }
                _ => warn! {"Cannot write EoS after seeking"},
            };
        }

        self.eos = false;
        self.bos = false;
        self.ofsgp_page = 0;
        self.stream_serial += 1;

        match self.rdr.seek_absgp(None, absgp) {
            Ok(_) => {
                // need to set some offset for next_page()
                let pck = self
                    .rdr
                    .read_packet()
                    .map_err(|e| DecoderError::PassthroughDecoder(e.to_string()))?;
                match pck {
                    Some(pck) => {
                        self.ofsgp_page = pck.absgp_page();
                        debug!("Seek to offset page {}", self.ofsgp_page);
                        Ok(())
                    }
                    None => Err(DecoderError::PassthroughDecoder(
                        "Packet is None".to_string(),
                    )),
                }
            }
            Err(e) => Err(DecoderError::PassthroughDecoder(e.to_string())),
        }
    }

    fn next_packet(&mut self) -> DecoderResult<Option<AudioPacket>> {
        // write headers if we are (re)starting
        if !self.bos {
            self.wtr
                .write_packet(
                    self.ident.clone(),
                    self.stream_serial,
                    PacketWriteEndInfo::EndPage,
                    0,
                )
                .map_err(|e| DecoderError::PassthroughDecoder(e.to_string()))?;
            self.wtr
                .write_packet(
                    self.comment.clone(),
                    self.stream_serial,
                    PacketWriteEndInfo::NormalPacket,
                    0,
                )
                .map_err(|e| DecoderError::PassthroughDecoder(e.to_string()))?;
            self.wtr
                .write_packet(
                    self.setup.clone(),
                    self.stream_serial,
                    PacketWriteEndInfo::EndPage,
                    0,
                )
                .map_err(|e| DecoderError::PassthroughDecoder(e.to_string()))?;
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
                Err(e) => return Err(DecoderError::PassthroughDecoder(e.to_string())),
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
                .map_err(|e| DecoderError::PassthroughDecoder(e.to_string()))?;

            let data = self.wtr.inner_mut();

            if !data.is_empty() {
                let ogg_data = AudioPacket::OggData(std::mem::take(data));
                return Ok(Some(ogg_data));
            }
        }
    }
}
