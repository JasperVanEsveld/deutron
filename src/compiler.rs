use std::sync::Mutex;
use std::{fs::remove_file, path::PathBuf};

use crate::backend::Backend;
use crate::base::{get_out_path, Base};
use crate::cleanup::cleanup_onexit;
use crate::shared::embed_error::{EmbedError, UnwrapPrintln};
use crate::state::State;
use crate::temp::get_temp_dir;
use clap::{command, Parser};
use serde::{Deserialize, Serialize};

#[macro_use]
mod macros;
mod backend;
mod base;
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
    #[clap(
        long,
        short,
        help = "Directly runs command instead of compiling to an executable"
    )]
    pub debug: bool,
    #[clap(long, short, help = "The output file of the executable")]
    pub out: Option<String>,
    #[clap(
        long,
        short,
        default_value = "1.0",
        help = "Set the version of the output binary"
    )]
    pub set_version: String,
    #[clap(
        long,
        short,
        help = "Opens a terminal on windows when the application is double clicked, useful for debugging"
    )]
    pub terminal: bool,
    #[clap(
        long,
        short,
        default_value = "./",
        help = "The directory that is packed into the binary"
    )]
    pub include: PathBuf,
    #[clap(long, short, help = "Forces deutron to unpack on every boot")]
    pub no_cache: bool,
    #[clap(
        long,
        help = "Enables the use of webview developer console (F12 on windows)"
    )]
    pub dev_tools: bool,
    #[clap(long, short, help = "Logs all info and messages send")]
    pub verbose: bool,
    #[clap(long, help = "Compile for a given target: windows, linux or macos")]
    pub target: Option<String>,
    #[arg(
        trailing_var_arg = true,
        allow_hyphen_values = true,
        help = "The command used to run your backend"
    )]
    pub command: Vec<String>,
}

fn main() {
    let config = Config::parse();
    if config.debug {
        cleanup_onexit();
        let manager = webview::webview_manager::WebViewManager::new();
        manager.start(Some(vec![]));
    } else {
        embed(config).unwrap_or_println();
    }
}

pub fn get_target(config: &Config) -> &str {
    match config.target.as_ref() {
        Some(target) => target,
        None => std::env::consts::OS,
    }
}

fn embed(config: Config) -> Result<(), EmbedError> {
    let target = get_target(&config);
    let mut base = Base::load(target).unwrap_or_println();

    if !config.terminal && target == "windows" {
        base.disable_terminal()?;
    }

    let target_path = get_out_path(&config)?;
    if target_path.exists() {
        remove_file(&target_path).map_err(|_| {
            EmbedError(format!("Can't write binary, make sure the application is closed").into())
        })?;
    }
    let icon = "favicon.ico";
    if target == "windows" {
        base.set_icon(icon).ok();
    }
    base.add_pack(&config.include)?;
    base.add_sized::<bool>(&config.verbose);
    base.add_sized::<bool>(&config.no_cache);
    base.add_sized::<bool>(&config.dev_tools);
    base.add_string(&config.set_version);
    base.add_strings(config.command.clone());
    println!("Compiled to: {:?}", base.write(target_path)?);
    Ok(())
}

fn start() -> Result<State, EmbedError> {
    let config = Config::parse();
    let mut backend = Backend::new(config.command.clone(), &PathBuf::from(&config.include));
    Ok(State {
        verbose: config.verbose,
        dev_tools: config.dev_tools,
        icon: config.include.join("./favicon.ico"),
        backend_dir: config.include,
        webview_dir: get_temp_dir(&config.set_version, "webview").unwrap_or_println(),
        backend_in: Mutex::new(backend.get_stdin()),
        backend_out: Mutex::new(backend.get_stdout()),
        backend_err: Mutex::new(backend.get_stderr()),
        backend: Mutex::new(backend),
    })
}
