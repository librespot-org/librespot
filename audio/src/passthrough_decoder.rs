// Passthrough decoder for librespot
use super::{AudioDecoder, AudioError, AudioPacket};
use ogg::{OggReadError, Packet, PacketReader, PacketWriteEndInfo, PacketWriter};
use std::fmt;
use std::io::{Read, Seek};
use std::time::{SystemTime, UNIX_EPOCH};

fn write_headers<T: Read + Seek>(
    rdr: &mut PacketReader<T>,
    wtr: &mut PacketWriter<Vec<u8>>,
) -> Result<u32, PassthroughError> {
    let mut stream_serial: u32 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u32;

    // search for ident, comment, setup
    get_header(1, rdr, wtr, &mut stream_serial, PacketWriteEndInfo::EndPage)?;
    get_header(
        3,
        rdr,
        wtr,
        &mut stream_serial,
        PacketWriteEndInfo::NormalPacket,
    )?;
    get_header(5, rdr, wtr, &mut stream_serial, PacketWriteEndInfo::EndPage)?;

    // remove un-needed packets
    rdr.delete_unread_packets();
    return Ok(stream_serial);
}

fn get_header<T>(
    code: u8,
    rdr: &mut PacketReader<T>,
    wtr: &mut PacketWriter<Vec<u8>>,
    stream_serial: &mut u32,
    info: PacketWriteEndInfo,
) -> Result<u32, PassthroughError>
where
    T: Read + Seek,
{
    let pck: Packet = rdr.read_packet_expected()?;

    // set a unique serial number
    if pck.stream_serial() != 0 {
        *stream_serial = pck.stream_serial();
    }

    let pkt_type = pck.data[0];
    debug!("Vorbis header type{}", &pkt_type);

    // all headers are mandatory
    if pkt_type != code {
        return Err(PassthroughError(OggReadError::InvalidData));
    }

    // headers keep original granule number
    let absgp_page = pck.absgp_page();
    wtr.write_packet(
        pck.data.into_boxed_slice(),
        *stream_serial,
        info,
        absgp_page,
    )
    .unwrap();

    return Ok(*stream_serial);
}

pub struct PassthroughDecoder<R: Read + Seek> {
    rdr: PacketReader<R>,
    wtr: PacketWriter<Vec<u8>>,
    lastgp_page: Option<u64>,
    absgp_page: u64,
    stream_serial: u32,
}

pub struct PassthroughError(ogg::OggReadError);

impl<R: Read + Seek> PassthroughDecoder<R> {
    /// Constructs a new Decoder from a given implementation of `Read + Seek`.
    pub fn new(rdr: R) -> Result<Self, PassthroughError> {
        let mut rdr = PacketReader::new(rdr);
        let mut wtr = PacketWriter::new(Vec::new());

        let stream_serial = write_headers(&mut rdr, &mut wtr)?;
        info!("Starting passthrough track with serial {}", stream_serial);

        return Ok(PassthroughDecoder {
            rdr,
            wtr,
            lastgp_page: Some(0),
            absgp_page: 0,
            stream_serial,
        });
    }
}

impl<R: Read + Seek> AudioDecoder for PassthroughDecoder<R> {
    fn seek(&mut self, ms: i64) -> Result<(), AudioError> {
        info!("Seeking to {}", ms);
        self.lastgp_page = match ms {
            0 => Some(0),
            _ => None,
        };

        // hard-coded to 44.1 kHz
        match self.rdr.seek_absgp(None, (ms * 44100 / 1000) as u64) {
            Ok(_) => return Ok(()),
            Err(err) => return Err(AudioError::PassthroughError(err.into())),
        }
    }

    fn next_packet(&mut self) -> Result<Option<AudioPacket>, AudioError> {
        let mut skip = self.lastgp_page.is_none();
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
            let lastgp_page = self.lastgp_page.get_or_insert(pckgp_page);

            // consume packets till next page to get a granule reference
            if skip {
                if *lastgp_page == pckgp_page {
                    debug!("skipping packet");
                    continue;
                }
                skip = false;
                info!("skipped at {}", pckgp_page);
            }

            // now we can calculate absolute granule
            self.absgp_page += pckgp_page - *lastgp_page;
            self.lastgp_page = Some(pckgp_page);

            // set packet type
            let inf = if pck.last_in_stream() {
                self.lastgp_page = Some(0);
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
                    self.absgp_page,
                )
                .unwrap();

            let data = self.wtr.inner_mut();

            if data.len() > 0 {
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
