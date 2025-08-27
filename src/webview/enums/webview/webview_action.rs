use crate::webview::enums::{
    webview::{info::RequestInfo, target::Target},
    window::{window_action::WindowAction, window_message::WindowMessage},
};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use tao::event_loop::EventLoopProxy;

#[derive(Debug, Serialize, Deserialize)]
pub enum WebViewAction {
    Message(Target, i32, WindowMessage),
    Window(Target, WindowAction),
    Request(Target, RequestInfo),
}

impl WebViewAction {
    pub fn perform(self, proxy: &EventLoopProxy<WebViewAction>) -> Result<()> {
        proxy
            .send_event(self)
            .context("The eventloop was closed, so the action can no longer be performed")?;
        Ok(())
    }
}
