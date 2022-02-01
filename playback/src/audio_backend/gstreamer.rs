use std::{ops::Drop, thread};

use gst::{
    event::{FlushStart, FlushStop},
    prelude::*,
    State,
};
use zerocopy::AsBytes;

use super::{Open, Sink, SinkAsBytes, SinkError, SinkResult};

use crate::{
    config::AudioFormat, convert::Converter, decoder::AudioPacket, NUM_CHANNELS, SAMPLE_RATE,
};

#[allow(dead_code)]
pub struct GstreamerSink {
    appsrc: gst_app::AppSrc,
    bufferpool: gst::BufferPool,
    pipeline: gst::Pipeline,
    format: AudioFormat,
}

impl Open for GstreamerSink {
    fn open(device: Option<String>, format: AudioFormat) -> Self {
        info!("Using GStreamer sink with format: {:?}", format);
        gst::init().expect("failed to init GStreamer!");

        let gst_format = match format {
            AudioFormat::F64 => gst_audio::AUDIO_FORMAT_F64,
            AudioFormat::F32 => gst_audio::AUDIO_FORMAT_F32,
            AudioFormat::S32 => gst_audio::AUDIO_FORMAT_S32,
            AudioFormat::S24 => gst_audio::AUDIO_FORMAT_S2432,
            AudioFormat::S24_3 =>  gst_audio::AUDIO_FORMAT_S24,
            AudioFormat::S16 => gst_audio::AUDIO_FORMAT_S16,
        };

        let gst_info = gst_audio::AudioInfo::builder(gst_format, SAMPLE_RATE, NUM_CHANNELS as u32).build().expect("Failed to create GStreamer audio format");
        let gst_caps = gst_info.to_caps().expect("Failed to create GStreamer caps");

        let sample_size = format.size();
        let gst_bytes = NUM_CHANNELS as usize * 1024 * sample_size;

        let pipeline_str_preamble = "appsrc block=true name=appsrc0 ";
        // no need to dither twice; use librespot dithering instead
        let pipeline_str_rest = r#" ! audioconvert dithering=none ! autoaudiosink"#;
        let pipeline_str: String = match device {
            Some(x) => format!("{}{}", pipeline_str_preamble, x),
            None => format!("{}{}", pipeline_str_preamble, pipeline_str_rest),
        };
        info!("Pipeline: {}", pipeline_str);

        let pipelinee = gst::parse_launch(&*pipeline_str).expect("Couldn't launch pipeline; likely a GStreamer issue or an error in the pipeline string you specified in the 'device' argument to librespot.");
        let pipeline = pipelinee
            .dynamic_cast::<gst::Pipeline>()
            .expect("couldn't cast pipeline element at runtime!");
        let bus = pipeline.bus().expect("couldn't get bus from pipeline");
        let mainloop = glib::MainLoop::new(None, false);
        let appsrce: gst::Element = pipeline
            .by_name("appsrc0")
            .expect("couldn't get appsrc from pipeline");
        let appsrc: gst_app::AppSrc = appsrce
            .dynamic_cast::<gst_app::AppSrc>()
            .expect("couldn't cast AppSrc element at runtime!");

        appsrc.set_caps(Some(&gst_caps));
        appsrc.set_max_bytes(gst_bytes as u64);

        let bufferpool = gst::BufferPool::new();

        let mut conf = bufferpool.config();
        conf.set_params(Some(&gst_caps), gst_bytes as u32, 0, 0);
        bufferpool
            .set_config(conf)
            .expect("couldn't configure the buffer pool");

        thread::spawn(move || {
            let thread_mainloop = mainloop;
            let watch_mainloop = thread_mainloop.clone();
            bus.add_watch(move |_, msg| {
                match msg.view() {
                    gst::MessageView::Eos(_) => {
                        println!("gst signaled end of stream");
                        watch_mainloop.quit();
                    }
                    gst::MessageView::Error(err) => {
                        println!(
                            "Error from {:?}: {} ({:?})",
                            err.src().map(|s| s.path_string()),
                            err.error(),
                            err.debug()
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
            .set_state(State::Ready)
            .expect("unable to set the pipeline to the `Ready` state");

        Self {
            appsrc,
            bufferpool,
            pipeline,
            format,
        }
    }
}

impl Sink for GstreamerSink {
    fn start(&mut self) -> SinkResult<()> {
        self.appsrc.send_event(FlushStop::new(true));
        self.bufferpool
            .set_active(true)
            .expect("couldn't activate buffer pool");
        self.pipeline
            .set_state(State::Playing)
            .map_err(|e| SinkError::StateChange(e.to_string()))?;
        Ok(())
    }

    fn stop(&mut self) -> SinkResult<()> {
        self.appsrc.send_event(FlushStart::new());
        self.pipeline
            .set_state(State::Paused)
            .map_err(|e| SinkError::StateChange(e.to_string()))?;
        self.bufferpool
            .set_active(false)
            .expect("couldn't deactivate buffer pool");
        Ok(())
    }

    sink_as_bytes!();
}

impl Drop for GstreamerSink {
    fn drop(&mut self) {
        let _ = self.pipeline.set_state(State::Null);
    }
}

impl SinkAsBytes for GstreamerSink {
    fn write_bytes(&mut self, data: &[u8]) -> SinkResult<()> {
        let mut buffer = self
            .bufferpool
            .acquire_buffer(None)
            .map_err(|e| SinkError::OnWrite(e.to_string()))?;

        let mutbuf = buffer.make_mut();
        mutbuf.set_size(data.len());
        mutbuf
            .copy_from_slice(0, data.as_bytes())
            .expect("Failed to copy from slice");

        self.appsrc
            .push_buffer(buffer)
            .map_err(|e| SinkError::OnWrite(e.to_string()))?;

        Ok(())
    }
}

impl GstreamerSink {
    pub const NAME: &'static str = "gstreamer";
}
