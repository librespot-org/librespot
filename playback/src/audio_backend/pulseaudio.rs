use super::{Open, Sink};
use libc;
use libpulse_sys::*;
use std::ffi::CStr;
use std::ffi::CString;
use std::io;
use std::mem;
use std::ptr::{null, null_mut};

pub struct PulseAudioSink {
    s: *mut pa_simple,
    ss: pa_sample_spec,
    name: CString,
    desc: CString,
}

fn call_pulseaudio<T, F, FailCheck>(f: F, fail_check: FailCheck, kind: io::ErrorKind) -> io::Result<T>
where
    T: Copy,
    F: Fn(*mut libc::c_int) -> T,
    FailCheck: Fn(T) -> bool,
{
    let mut error: libc::c_int = 0;
    let ret = f(&mut error);
    if fail_check(ret) {
        let err_cstr = unsafe { CStr::from_ptr(pa_strerror(error)) };
        let errstr = err_cstr.to_string_lossy().into_owned();
        Err(io::Error::new(kind, errstr))
    } else {
        Ok(ret)
    }
}

impl PulseAudioSink {
    fn free_connection(&mut self) {
        if self.s != null_mut() {
            unsafe {
                pa_simple_free(self.s);
            }
            self.s = null_mut();
        }
    }
}

impl Drop for PulseAudioSink {
    fn drop(&mut self) {
        self.free_connection();
    }
}

impl Open for PulseAudioSink {
    fn open(device: Option<String>) -> PulseAudioSink {
        debug!("Using PulseAudio sink");

        if device.is_some() {
            panic!("pulseaudio sink does not support specifying a device name");
        }

        let ss = pa_sample_spec {
            format: PA_SAMPLE_S16LE,
            channels: 2, // stereo
            rate: 44100,
        };

        let name = CString::new("librespot").unwrap();
        let description = CString::new("Spotify endpoint").unwrap();

        PulseAudioSink {
            s: null_mut(),
            ss: ss,
            name: name,
            desc: description,
        }
    }
}

impl Sink for PulseAudioSink {
    fn start(&mut self) -> io::Result<()> {
        if self.s == null_mut() {
            self.s = call_pulseaudio(
                |err| unsafe {
                    pa_simple_new(
                        null(),             // Use the default server.
                        self.name.as_ptr(), // Our application's name.
                        PA_STREAM_PLAYBACK,
                        null(),             // Use the default device.
                        self.desc.as_ptr(), // desc of our stream.
                        &self.ss,           // Our sample format.
                        null(),             // Use default channel map
                        null(),             // Use default buffering attributes.
                        err,
                    )
                },
                |ptr| ptr == null_mut(),
                io::ErrorKind::ConnectionRefused,
            )?;
        }
        Ok(())
    }

    fn stop(&mut self) -> io::Result<()> {
        self.free_connection();
        Ok(())
    }

    fn write(&mut self, data: &[i16]) -> io::Result<()> {
        if self.s == null_mut() {
            Err(io::Error::new(
                io::ErrorKind::NotConnected,
                "Not connected to pulseaudio",
            ))
        } else {
            let ptr = data.as_ptr() as *const libc::c_void;
            let len = data.len() as usize * mem::size_of::<i16>();
            assert!(len > 0);
            call_pulseaudio(
                |err| unsafe { pa_simple_write(self.s, ptr, len, err) },
                |ret| ret < 0,
                io::ErrorKind::BrokenPipe,
            )?;
            Ok(())
        }
    }
}
