use crate::state::STATE;
use std::panic;

pub fn cleanup_backend() {
    STATE.backend.lock().unwrap().process.kill().ok().unwrap();
}

pub fn cleanup_onexit() {
    panic::set_hook(Box::new(|_| {
        cleanup_backend();
    }));
    ctrlc::set_handler(move || {
        cleanup_backend();
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");
}
