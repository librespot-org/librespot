use super::{Open, Sink};
use std::io;
use libpulse_sys::*;
use std::ptr::{null, null_mut};
use std::mem::{transmute};
use std::ffi::CString;

pub struct PulseAudioSink(*mut pa_simple);

impl Open for PulseAudioSink {
   fn open(device: Option<String>) -> PulseAudioSink {
        debug!("Using PulseAudio sink");

        if device.is_some() {
            panic!("pulseaudio sink does not support specifying a device name");
        }

        let ss = pa_sample_spec {
            format: PA_SAMPLE_S16LE,
            channels: 2, // stereo
            rate: 44100
        };
        
        let name = CString::new("librespot").unwrap();
        let description = CString::new("A spoty client library").unwrap();

        let s = unsafe {
            pa_simple_new(null(),               // Use the default server.
                          name.as_ptr(),        // Our application's name.
                          PA_STREAM_PLAYBACK,
                          null(),               // Use the default device.
                          description.as_ptr(), // Description of our stream.
                          &ss,                  // Our sample format.
                          null(),               // Use default channel map
                          null(),               // Use default buffering attributes.
                          null_mut(),           // Ignore error code.
            )
        };
        assert!(s != null_mut());
        
        PulseAudioSink(s)
    }
}

impl Sink for PulseAudioSink {
    fn start(&mut self) -> io::Result<()> {
        Ok(())
    }

    fn stop(&mut self) -> io::Result<()> {
        Ok(())
    }

    fn write(&mut self, data: &[i16]) -> io::Result<()> {
        unsafe {
            let ptr = transmute(data.as_ptr());
            let bytes = data.len() as usize * 2;
            pa_simple_write(self.0, ptr, bytes, null_mut());
        };
        
        Ok(())
    }
}




