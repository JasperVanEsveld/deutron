use crate::{
    unwrap_log,
    webview::{
        enums::{
            backend::backend_message::BackendMessage,
            webview::{target::Target, webview_action::WebViewAction},
            window::window_message::WindowMessage,
        },
        webview_manager::{WebViewExt, WebViewManager, WindowManagerId},
    },
};
use serde::{Deserialize, Serialize};
use tao::event_loop::EventLoopProxy;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum InfoMessage {
    Loaded(i32),
    Created(i32),
    Closed(i32),
    Response(ResponseInfo),
    Error(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WindowInfo {
    id: i32,
    title: String,
    url: String,
    fullscreen: bool,
}

impl WindowInfo {
    fn from_webview(id: i32, webview: &WebViewExt) -> WindowInfo {
        let window = &webview.window;
        WindowInfo {
            id,
            title: window.title(),
            url: webview.view.url().unwrap().to_string(),
            fullscreen: window.fullscreen().map_or(false, |_| true),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ResponseInfo {
    Windows(Vec<WindowInfo>),
    Window(WindowInfo),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RequestInfo {
    Windows,
    Window(Option<i32>),
}

impl RequestInfo {
    pub fn perform(
        self,
        target: Target,
        manager: &WebViewManager,
        proxy: &EventLoopProxy<WebViewAction>,
    ) {
        let _ = proxy;
        match self {
            RequestInfo::Windows => {
                let windows = manager
                    .iter_webviews()
                    .map(|(id, view)| WindowInfo::from_webview(*id, view))
                    .collect();
                let message = InfoMessage::Response(ResponseInfo::Windows(windows));
                unwrap_log!(
                    match target {
                        Target::Window(id) => {
                            WindowMessage::Info(message).send(&WindowManagerId::Number(id), manager)
                        }
                        Target::Backend => BackendMessage::Info(message).send(),
                    },
                    target,
                    proxy
                )
            }
            RequestInfo::Window(id) => {
                let window_id = match id {
                    Some(id) => id,
                    None => match target {
                        Target::Backend => {
                            unwrap_log!(id.ok_or("Backend needs to provide a window id"))
                        }
                        Target::Window(current) => current,
                    },
                };
                let webview = unwrap_log!(manager
                    .get(&WindowManagerId::Number(window_id))
                    .ok_or("Window not found"));
                let info = WindowInfo::from_webview(window_id, webview);
                let message = InfoMessage::Response(ResponseInfo::Window(info));

                let res = match target {
                    Target::Window(id) => {
                        WindowMessage::Info(message).send(&WindowManagerId::Number(id), manager)
                    }
                    Target::Backend => BackendMessage::Info(message).send(),
                };
                unwrap_log!(res, target, proxy)
            }
        }
    }
}
