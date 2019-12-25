use super::{Open, Sink};
use std::{io, thread, time, process::exit};
use std::sync::mpsc::{sync_channel, SyncSender};
use gst::prelude::*;
use gst::*;
use gst_app::*;
use glib::MainLoop;
use zerocopy::*;

pub struct GstreamerSink {
    tx: SyncSender<Vec<u8>>,
    pipeline: gst::Pipeline
}

impl Open for GstreamerSink {
    fn open(device: Option<String>) -> GstreamerSink {
        gst::init();
        let pipeline_str_preamble = r#"appsrc caps="audio/x-raw,format=S16LE,layout=interleaved,channels=2,rate=44100" block=true max-bytes=4096 name=appsrc0 "#;
        let pipeline_str_rest = r#" ! audioconvert ! autoaudiosink"#;
        let pipeline_str : String = match device {
            Some(x) => format!("{}{}", pipeline_str_preamble, x),
            None => format!("{}{}", pipeline_str_preamble, pipeline_str_rest)
        };
        println!("Pipeline: {}", pipeline_str);

        gst::init().unwrap();
        let pipelinee = gst::parse_launch(&*pipeline_str).expect("New Pipeline error");
        let pipeline = pipelinee.dynamic_cast::<gst::Pipeline>().expect("Couldnt cast pipeline element at runtime!");
        let mut bus = pipeline.get_bus().expect("Couldn't get bus from pipeline");
        let mut mainloop = glib::MainLoop::new(None, false);
        let mut appsrce : gst::Element = pipeline.get_by_name("appsrc0").expect("Couldn't get appsrc from pipeline");
        let mut appsrc : gst_app::AppSrc = appsrce.dynamic_cast::<gst_app::AppSrc>().expect("Couldnt cast AppSrc element at runtime!");
        let bufferpool = gst::BufferPool::new();
        let appsrc_caps = appsrc.get_caps().expect("get appsrc caps failed");
        let mut conf = bufferpool.get_config();
        conf.set_params(Some(&appsrc_caps), 8192, 0, 0);
        bufferpool.set_config(conf).expect("Couldn't configure the buffer pool");
        bufferpool.set_active(true).expect("Couldn't activate buffer pool");

        let (tx, rx) = sync_channel::<Vec<u8>>(128);
        thread::spawn(move || {
            for data in rx {
                let mut buffer = bufferpool.acquire_buffer(None).expect("acquire buffer");
                let mutbuf = buffer.make_mut();
                mutbuf.set_size(data.len());
                mutbuf.copy_from_slice(0, data.as_bytes());
                let res = appsrc.push_buffer(buffer).expect("Failed to push buffer");
            }
        });

        thread::spawn(move || {
            unsafe {
                let thread_mainloop = mainloop;
                let watch_mainloop = thread_mainloop.clone();
                bus.add_watch(move |_, msg| {
                    use gst::MessageView;
                    match msg.view() {
                        MessageView::Eos(..) => watch_mainloop.quit(),
                        MessageView::Error(err) => {
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
                .expect("Failed to add bus watch");
                thread_mainloop.run();
            }
        });

        GstreamerSink {
            tx: tx,
            pipeline: pipeline
        }
    }
}

impl Sink for GstreamerSink {
    fn start(&mut self) -> io::Result<()> {
        self.pipeline.set_state(gst::State::Playing).expect("Unable to set the pipeline to the `Playing` state");
        Ok(())
    }
    fn stop(&mut self) -> io::Result<()> {
        self.pipeline.set_state(gst::State::Paused).expect("Unable to set the pipeline to the `Paused` state");
        Ok(())
    }
    fn write(&mut self, data: &[i16]) -> io::Result<()> {
        // Copy expensively (in to_vec()) to avoid thread synchronization
        let deighta : &[u8] = data.as_bytes();
        self.tx.send(deighta.to_vec()).expect("tx send failed in write function");
        Ok(())
    }
}