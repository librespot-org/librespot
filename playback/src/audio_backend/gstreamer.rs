use super::{Open, Sink};
use std::{io, thread, time, process::exit};
use std::sync::mpsc::{sync_channel, SyncSender};
use gst::prelude::*;
use gst::*;
use gst_app::*;
use glib::MainLoop;
use zerocopy::*;

pub struct GstreamerSink {
    tx: SyncSender<Vec<i16>>,
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

        gst::init().unwrap();
        let pipelinee = gst::parse_launch(&*pipeline_str).expect("New Pipeline error");
        let pipeline = pipelinee.dynamic_cast::<gst::Pipeline>().expect("Couldnt cast pipeline element at runtime!");
        let mut bus = pipeline.get_bus().expect("Couldn't get bus from pipeline");
        let mut mainloop = glib::MainLoop::new(None, false);
        let mut appsrce : gst::Element = pipeline.get_by_name("appsrc0").expect("Couldn't get appsrc from pipeline");
        let mut appsrc : gst_app::AppSrc = appsrce.dynamic_cast::<gst_app::AppSrc>().expect("Couldnt cast AppSrc element at runtime!");
        //let mut appsrc = gst_app::AppSrc::new_from_element(appsrc_element.to_element());
        let bufferpool = gst::BufferPool::new();
        let appsrc_caps = appsrc.get_caps().expect("get appsrc caps failed");
        let mut conf = bufferpool.get_config();
        conf.set_params(Some(&appsrc_caps), 2048 * 2, 0, 0);
        if bufferpool.set_active(true).is_err(){
            panic!("Couldn't activate buffer pool");
        }

        /*
        thread::spawn(move || {
        let bus_receiver = bus.receiver();
            for message in bus_receiver.iter() {
                match message.parse() {
                    gst::message::StateChanged(x) =>
                        println!("element `{}` state changed", message.src_name()),
                    gst::message::Error(x) => {
                        println!("error msg from element `{}`: {}, quitting", message.src_name(), error.message());
                        break;
                    },
                    gst::message::Eos(ref _msg) => {
                        println!("eos received; quitting");
                        break;
                    },
                    _ =>
                        println!("Pipe message: {} from {} at {}", message.type_name(), message.src_name(), message.timestamp())
                }
            }
        });*/

        let (tx, rx) = sync_channel::<Vec<i16>>(64);
        thread::spawn(move || {
            for data in rx {
                let mut buffer = bufferpool.acquire_buffer(None).expect("acquire buffer");

                //assert!(data.len() <= buffer.len::<i16>());
                let mutbuf = buffer.make_mut();
                mutbuf.set_size(data.len() * 2);
                mutbuf.map_writable().unwrap().as_mut_slice().clone_from_slice(&data[..].as_bytes());

                //buffer.set_live(true);
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
        //self.pipeline.play();
        self.pipeline.set_state(gst::State::Playing).expect("Unable to set the pipeline to the `Playing` state");
        Ok(())
    }
    fn stop(&mut self) -> io::Result<()> {
        //self.pipeline.pause();
        self.pipeline.set_state(gst::State::Paused).expect("Unable to set the pipeline to the `Paused` state");
        Ok(())
    }
    fn write(&mut self, data: &[i16]) -> io::Result<()> {
        // Copy expensively to avoid thread synchronization
        let data = data.to_vec();
        self.tx.send(data).expect("tx send failed in write function");

        Ok(())
    }
}