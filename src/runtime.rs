use crate::backend::Backend;
use crate::cleanup::cleanup_onexit;
use crate::shared::embed_error::{EmbedError, UnwrapPrintln};
use crate::temp::get_temp_dir;
use crate::{app::App, state::State};
use clap::{command, Parser};
use serde::{Deserialize, Serialize};
use std::{env, path::PathBuf, sync::Mutex};

#[macro_use]
mod macros;
mod app;
mod backend;
mod cleanup;
mod shared;
mod state;
mod temp;
mod webview;

pub enum WebviewIcon<'a> {
    Embedded(&'a [u8]),
    Path(String),
    None,
}

#[derive(Parser, Debug, Serialize, Deserialize, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    #[clap(long, short, default_value = "1.0")]
    pub set_version: String,
    #[clap(long, short)]
    pub dev_tools: bool,
    #[clap(long, short)]
    pub terminal: bool,
    #[clap(long, short)]
    pub verbose: bool,
    #[clap(long, short)]
    pub out: Option<String>,
    #[clap(long, short, default_value = "./")]
    pub include: PathBuf,
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    pub command: Vec<String>,
}

fn main() {
    cleanup_onexit();
    let manager = webview::webview_manager::WebViewManager::new();
    manager.start(Some(vec![]));
}

fn start() -> Result<State, EmbedError> {
    let mut embedded = App::load().unwrap_or_println();
    let mut command = embedded.take_strings()?;
    let version = embedded.take_string()?;
    let dev_tools = embedded.take_sized::<bool>();
    let no_cache = embedded.take_sized::<bool>();
    let verbose = embedded.take_sized::<bool>();

    let backend_dir = get_temp_dir(&version, "backend")?;
    let webview_dir = get_temp_dir(&version, "webview")?;
    embedded.take_pack(&backend_dir, !no_cache)?;

    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    command.append(&mut args);
    if verbose {
        println!("Starting: {}", command.join(" "));
    }
    let mut backend = Backend::new(command.clone(), &backend_dir);

    Ok(State {
        verbose,
        dev_tools,
        icon: backend_dir.join("favicon.ico"),
        backend_dir,
        webview_dir,
        backend_in: Mutex::new(backend.get_stdin()),
        backend_out: Mutex::new(backend.get_stdout()),
        backend_err: Mutex::new(backend.get_stderr()),
        backend: Mutex::new(backend),
    })
}
