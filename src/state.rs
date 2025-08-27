use std::{
    path::PathBuf,
    sync::{LazyLock, Mutex},
};

use crate::{
    backend::{Backend, ChildErr, ChildIn, ChildOut},
    shared::embed_error::UnwrapPrintln,
    start,
};

pub static STATE: LazyLock<State> = LazyLock::new(|| {
    let state = start().unwrap_or_println();
    return state;
});

pub struct State {
    pub verbose: bool,
    pub dev_tools: bool,
    pub icon: PathBuf,
    pub backend_dir: PathBuf,
    pub webview_dir: PathBuf,
    pub backend: Mutex<Backend>,
    pub backend_in: Mutex<ChildIn>,
    pub backend_out: Mutex<ChildOut>,
    pub backend_err: Mutex<ChildErr>,
}
