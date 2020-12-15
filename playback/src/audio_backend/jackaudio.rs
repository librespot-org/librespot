use super::{Open, Sink};
use jack::{
    AsyncClient, AudioOut, Client, ClientOptions, Control, Port, ProcessHandler, ProcessScope,
};
use std::io;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};

pub struct JackSink {
    send: SyncSender<i16>,
    active_client: AsyncClient<(), JackData>,
}

pub struct JackData {
    rec: Receiver<i16>,
    port_l: Port<AudioOut>,
    port_r: Port<AudioOut>,
}

fn pcm_to_f32(sample: i16) -> f32 {
    sample as f32 / 32768.0
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
            buf_r[i] = pcm_to_f32(queue_iter.next().unwrap_or(0));
            buf_l[i] = pcm_to_f32(queue_iter.next().unwrap_or(0));
        }
        Control::Continue
    }
}

impl Open for JackSink {
    fn open(client_name: Option<String>) -> JackSink {
        info!("Using jack sink!");

        let client_name = client_name.unwrap_or("librespot".to_string());
        let (client, _status) =
            Client::new(&client_name[..], ClientOptions::NO_START_SERVER).unwrap();
        let ch_r = client.register_port("out_0", AudioOut::default()).unwrap();
        let ch_l = client.register_port("out_1", AudioOut::default()).unwrap();
        // buffer for samples from librespot (~10ms)
        let (tx, rx) = sync_channel(2 * 1024 * 4);
        let jack_data = JackData {
            rec: rx,
            port_l: ch_l,
            port_r: ch_r,
        };
        let active_client = AsyncClient::new(client, (), jack_data).unwrap();

        JackSink {
            send: tx,
            active_client: active_client,
        }
    }
}

impl Sink for JackSink {
    fn start(&mut self) -> io::Result<()> {
        Ok(())
    }

    fn stop(&mut self) -> io::Result<()> {
        Ok(())
    }

    fn write(&mut self, data: &[i16]) -> io::Result<()> {
        for s in data.iter() {
            let res = self.send.send(*s);
            if res.is_err() {
                error!("jackaudio: cannot write to channel");
            }
        }
        Ok(())
    }
}
