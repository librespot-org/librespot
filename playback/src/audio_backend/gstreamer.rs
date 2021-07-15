use super::{Open, Sink, SinkAsBytes, SinkResult};
use crate::config::AudioFormat;
use crate::convert::Converter;
use crate::decoder::AudioPacket;
use crate::{NUM_CHANNELS, SAMPLE_RATE};

use gstreamer as gst;
use gstreamer_app as gst_app;

use gst::prelude::*;
use zerocopy::AsBytes;

use std::sync::mpsc::{sync_channel, SyncSender};
use std::thread;

#[allow(dead_code)]
pub struct GstreamerSink {
    tx: SyncSender<Vec<u8>>,
    pipeline: gst::Pipeline,
    format: AudioFormat,
}

impl Open for GstreamerSink {
    fn open(device: Option<String>, format: AudioFormat) -> Self {
        info!("Using GStreamer sink with format: {:?}", format);
        gst::init().expect("failed to init GStreamer!");

        // GStreamer calls S24 and S24_3 different from the rest of the world
        let gst_format = match format {
            AudioFormat::S24 => "S24_32".to_string(),
            AudioFormat::S24_3 => "S24".to_string(),
            _ => format!("{:?}", format),
        };
        let sample_size = format.size();
        let gst_bytes = 2048 * sample_size;

        #[cfg(target_endian = "little")]
        const ENDIANNESS: &str = "LE";
        #[cfg(target_endian = "big")]
        const ENDIANNESS: &str = "BE";

        let pipeline_str_preamble = format!(
            "appsrc caps=\"audio/x-raw,format={}{},layout=interleaved,channels={},rate={}\" block=true max-bytes={} name=appsrc0 ",
            gst_format, ENDIANNESS, NUM_CHANNELS, SAMPLE_RATE, gst_bytes
        );
        // no need to dither twice; use librespot dithering instead
        let pipeline_str_rest = r#" ! audioconvert dithering=none ! autoaudiosink"#;
        let pipeline_str: String = match device {
            Some(x) => format!("{}{}", pipeline_str_preamble, x),
            None => format!("{}{}", pipeline_str_preamble, pipeline_str_rest),
        };
        info!("Pipeline: {}", pipeline_str);

        gst::init().unwrap();
        let pipelinee = gst::parse_launch(&*pipeline_str).expect("Couldn't launch pipeline; likely a GStreamer issue or an error in the pipeline string you specified in the 'device' argument to librespot.");
        let pipeline = pipelinee
            .dynamic_cast::<gst::Pipeline>()
            .expect("couldn't cast pipeline element at runtime!");
        let bus = pipeline.get_bus().expect("couldn't get bus from pipeline");
        let mainloop = glib::MainLoop::new(None, false);
        let appsrce: gst::Element = pipeline
            .get_by_name("appsrc0")
            .expect("couldn't get appsrc from pipeline");
        let appsrc: gst_app::AppSrc = appsrce
            .dynamic_cast::<gst_app::AppSrc>()
            .expect("couldn't cast AppSrc element at runtime!");
        let bufferpool = gst::BufferPool::new();
        let appsrc_caps = appsrc.get_caps().expect("couldn't get appsrc caps");
        let mut conf = bufferpool.get_config();
        conf.set_params(Some(&appsrc_caps), 4096 * sample_size as u32, 0, 0);
        bufferpool
            .set_config(conf)
            .expect("couldn't configure the buffer pool");
        bufferpool
            .set_active(true)
            .expect("couldn't activate buffer pool");

        let (tx, rx) = sync_channel::<Vec<u8>>(64 * sample_size);
        thread::spawn(move || {
            for data in rx {
                let buffer = bufferpool.acquire_buffer(None);
                if let Ok(mut buffer) = buffer {
                    let mutbuf = buffer.make_mut();
                    mutbuf.set_size(data.len());
                    mutbuf
                        .copy_from_slice(0, data.as_bytes())
                        .expect("Failed to copy from slice");
                    let _eat = appsrc.push_buffer(buffer);
                }
            }
        });

        thread::spawn(move || {
            let thread_mainloop = mainloop;
            let watch_mainloop = thread_mainloop.clone();
            bus.add_watch(move |_, msg| {
                match msg.view() {
                    gst::MessageView::Eos(..) => watch_mainloop.quit(),
                    gst::MessageView::Error(err) => {
                        println!(
                            "Error from {:?}: {} ({:?})",
                            err.get_src().map(|s| s.get_path_string()),
                            err.get_error(),
                            err.get_debug()
                        );
                        watch_mainloop.quit();
                    }
                    _ => (),
                };

                glib::Continue(true)
            })
            .expect("failed to add bus watch");
            thread_mainloop.run();
        });

        pipeline
            .set_state(gst::State::Playing)
            .expect("unable to set the pipeline to the `Playing` state");

        Self {
            tx,
            pipeline,
            format,
        }
    }
}

impl Sink for GstreamerSink {
    sink_as_bytes!();
}

impl SinkAsBytes for GstreamerSink {
    fn write_bytes(&mut self, data: &[u8]) -> SinkResult<()> {
        // Copy expensively (in to_vec()) to avoid thread synchronization
        self.tx
            .send(data.to_vec())
            .expect("tx send failed in write function");
        Ok(())
    }
}

impl GstreamerSink {
    pub const NAME: &'static str = "gstreamer";
}
