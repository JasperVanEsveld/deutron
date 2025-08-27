use crate::webview::enums::{
    webview::{info::RequestInfo, target::Target, webview_action::WebViewAction},
    window::{window_action::WindowAction, window_message::WindowMessage},
};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use tao::event_loop::EventLoopProxy;

#[derive(Debug, Serialize, Deserialize)]
pub enum BackendEvent {
    Message { target: i32, data: String },
    Window(WindowAction),
    Request(RequestInfo),
}

impl BackendEvent {
    pub fn handle(self, proxy: &EventLoopProxy<WebViewAction>) -> Result<()> {
        match self {
            BackendEvent::Message { target, data } => {
                WebViewAction::Message(
                    Target::Backend,
                    target,
                    WindowMessage::Message {
                        from: Target::Backend,
                        data,
                    },
                )
                .perform(proxy)?;
            }
            BackendEvent::Window(action) => {
                WebViewAction::Window(Target::Backend, action).perform(proxy)?;
            }
            BackendEvent::Request(info) => {
                WebViewAction::Request(Target::Backend, info).perform(proxy)?;
            }
        };
        Ok(())
    }
}
