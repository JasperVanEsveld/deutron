use serde::{Deserialize, Serialize};
use tao::event_loop::EventLoopProxy;

use crate::webview::enums::{
    backend::backend_message::BackendMessage,
    webview::{info::RequestInfo, target::Target, webview_action::WebViewAction},
    window::{
        window_action::WindowAction, window_control::WindowControl, window_message::WindowMessage,
    },
};

use anyhow::Result;

#[derive(Debug, Serialize, Deserialize)]
pub enum WindowEvent {
    Loaded,
    Message {
        target: Target,
        data: String,
    },
    Control {
        target: Option<i32>,
        control: WindowControl,
    },
    Window(WindowAction),
    Request(RequestInfo),
}

impl WindowEvent {
    pub fn handle(self, id: i32, proxy: &EventLoopProxy<WebViewAction>) -> Result<()> {
        let source = Target::Window(id);
        match self {
            WindowEvent::Loaded => {
                WebViewAction::Window(source, WindowAction::Loaded).perform(proxy)?;
            }
            WindowEvent::Message {
                target: Target::Backend,
                data,
            } => {
                BackendMessage::Message { from: id, data }.send()?;
            }
            WindowEvent::Message {
                target: Target::Window(target),
                data,
            } => {
                let message = WindowMessage::Message {
                    from: source.clone(),
                    data,
                };
                WebViewAction::Message(source, target, message).perform(proxy)?;
            }
            WindowEvent::Control { target, control } => {
                WebViewAction::Window(
                    source,
                    WindowAction::Control {
                        target: target.unwrap_or(id),
                        control,
                    },
                )
                .perform(proxy)?;
            }
            WindowEvent::Window(action) => {
                WebViewAction::Window(source, action).perform(proxy)?;
            }
            WindowEvent::Request(info) => {
                WebViewAction::Request(source, info).perform(proxy)?;
            }
        };
        Ok(())
    }
}
