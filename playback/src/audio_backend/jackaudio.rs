use super::{Open, Sink, SinkResult};
use crate::config::AudioFormat;
use crate::convert::Converter;
use crate::decoder::AudioPacket;
use crate::NUM_CHANNELS;
use jack::{
    AsyncClient, AudioOut, Client, ClientOptions, Control, Port, ProcessHandler, ProcessScope,
};
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};

pub struct JackSink {
    send: SyncSender<f32>,
    // We have to keep hold of this object, or the Sink can't play...
    #[allow(dead_code)]
    active_client: AsyncClient<(), JackData>,
}

pub struct JackData {
    rec: Receiver<f32>,
    port_l: Port<AudioOut>,
    port_r: Port<AudioOut>,
}

impl ProcessHandler for JackData {
    fn process(&mut self, _: &Client, ps: &ProcessScope) -> Control {
        // get output port buffers
        let mut out_r = self.port_r.as_mut_slice(ps);
        let mut out_l = self.port_l.as_mut_slice(ps);
        let buf_r: &mut [f32] = &mut out_r;
        let buf_l: &mut [f32] = &mut out_l;
        // get queue iterator
        let mut queue_iter = self.rec.try_iter();

        let buf_size = buf_r.len();
        for i in 0..buf_size {
            buf_r[i] = queue_iter.next().unwrap_or(0.0);
            buf_l[i] = queue_iter.next().unwrap_or(0.0);
        }
        Control::Continue
    }
}

impl Open for JackSink {
    fn open(client_name: Option<String>, format: AudioFormat) -> Self {
        if format != AudioFormat::F32 {
            warn!("JACK currently does not support {:?} output", format);
        }
        info!("Using JACK sink with format {:?}", AudioFormat::F32);

        let client_name = client_name.unwrap_or_else(|| "librespot".to_string());
        let (client, _status) =
            Client::new(&client_name[..], ClientOptions::NO_START_SERVER).unwrap();
        let ch_r = client.register_port("out_0", AudioOut::default()).unwrap();
        let ch_l = client.register_port("out_1", AudioOut::default()).unwrap();
        // buffer for samples from librespot (~10ms)
        let (tx, rx) = sync_channel::<f32>(NUM_CHANNELS as usize * 1024 * AudioFormat::F32.size());
        let jack_data = JackData {
            rec: rx,
            port_l: ch_l,
            port_r: ch_r,
        };
        let active_client = AsyncClient::new(client, (), jack_data).unwrap();

        Self {
            send: tx,
            active_client,
        }
    }
}

impl Sink for JackSink {
    fn write(&mut self, packet: &AudioPacket, converter: &mut Converter) -> SinkResult<()> {
        let samples_f32: &[f32] = &converter.f64_to_f32(packet.samples());
        for sample in samples_f32.iter() {
            let res = self.send.send(*sample);
            if res.is_err() {
                error!("cannot write to channel");
            }
        }
        Ok(())
    }
}

impl JackSink {
    pub const NAME: &'static str = "jackaudio";
}
