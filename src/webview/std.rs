use crate::{
    state::STATE,
    unwrap_log,
    webview::enums::{
        backend::backend_event::BackendEvent, backend::backend_message::BackendMessage,
        webview::info::InfoMessage, webview::webview_action::WebViewAction,
    },
};
use anyhow::Result;
use std::{
    io::{BufRead, Read, Write},
    thread,
};
use tao::event_loop::EventLoopProxy;

static PREFIX: &str = "DEUTRON_IPC:";

pub fn listen_backendout(proxy: EventLoopProxy<WebViewAction>) {
    thread::spawn(move || {
        let mut backend_out = STATE.backend_out.lock().unwrap();
        for line_result in backend_out.by_ref().lines() {
            let mut line = unwrap_log!(line_result);
            if !line.starts_with(PREFIX) {
                println!("{}", line);
                continue;
            }
            line.replace_range(0..12, "");
            if STATE.verbose {
                println!("Backend -> {}", line);
            }
            let json_result = serde_json::from_str::<BackendEvent>(&line);
            let event = unwrap_log!(json_result);
            unwrap_log!(event.handle(&proxy));
        }
    });
}

pub fn listen_backenderr(proxy: EventLoopProxy<WebViewAction>) {
    thread::spawn(move || {
        let mut backend_out = STATE.backend_err.lock().unwrap();
        for line_result in backend_out.by_ref().lines() {
            let mut line = unwrap_log!(line_result);
            if !line.starts_with(PREFIX) {
                println!("{}", line);
                continue;
            }
            line.replace_range(0..12, "");
            if STATE.verbose {
                println!("Backend -> {}", line);
            }
            let json_result = serde_json::from_str::<BackendEvent>(&line);
            let event = unwrap_log!(json_result);
            unwrap_log!(event.handle(&proxy));
        }
    });
}

pub fn send_backendin(message: String) -> Result<()> {
    let mut backend_in = STATE.backend_in.lock().unwrap();

    if STATE.verbose {
        println!("Backend <- {}", message);
    }
    backend_in.write_all(message.as_bytes())?;
    backend_in.write_all(b"\n")?;
    backend_in.flush()?;
    Ok(())
}
