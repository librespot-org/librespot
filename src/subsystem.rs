use std::thread;

pub trait Subsystem : Send + Sized + 'static {
    fn run(self);
    fn start(self) {
        thread::spawn(move || self.run());
    }
}

