use crate::{
    cleanup::cleanup_backend,
    unwrap_log,
    webview::{
        enums::{
            backend::backend_message::BackendMessage,
            webview::{info::InfoMessage, target::Target, webview_action::WebViewAction},
            window::{
                window_config::WindowConfig, window_control::WindowControl,
                window_message::WindowMessage,
            },
        },
        webview_manager::{LoopVariables, WebViewManager, WindowManagerId},
    },
};
use serde::{Deserialize, Serialize};
use tao::{event_loop::ControlFlow, window::Fullscreen};

#[derive(Debug, Serialize, Deserialize)]
pub enum WindowAction {
    Loaded,
    Create(WindowConfig),
    Control { target: i32, control: WindowControl },
}

impl WindowAction {
    pub fn perform(self, source: Target, manager: &mut WebViewManager, loop_vars: LoopVariables) {
        match self {
            WindowAction::Loaded => match source {
                Target::Window(id) => {
                    let loaded = InfoMessage::Loaded(id);

                    BackendMessage::Info(loaded.clone()).send().unwrap();
                    let message = WindowMessage::Info(loaded);
                    let window_id = WindowManagerId::Number(id);
                    unwrap_log!(
                        message.send_others(&window_id, manager),
                        source,
                        loop_vars.proxy
                    );

                    manager.get(&window_id).unwrap().window.set_visible(true);
                }
                Target::Backend => {
                    unwrap_log!(Err("ID not provided to Loaded event"));
                }
            },
            WindowAction::Control { target, control } => {
                let id = WindowManagerId::Number(target);
                let get_result = unwrap_log!(
                    manager.get(&id).ok_or("Window not found"),
                    source,
                    loop_vars.proxy
                );
                let window = &get_result.window;
                match control {
                    WindowControl::Fullscreen => {
                        if let Some(_full) = window.fullscreen() {
                            window.set_fullscreen(None);
                        } else {
                            window.set_fullscreen(Some(Fullscreen::Borderless(None)));
                        }
                    }
                    WindowControl::Maximize => {
                        window.set_maximized(!window.is_maximized());
                    }
                    WindowControl::Minimize => {
                        window.set_minimized(true);
                    }
                    WindowControl::Drag => {
                        window.drag_window().unwrap();
                    }
                    WindowControl::Close => {
                        let number_id = id.to_number(manager);
                        manager.close(id.clone());
                        let closed = InfoMessage::Closed(number_id);
                        BackendMessage::Info(closed.clone()).send().unwrap();
                        let message = WindowMessage::Info(closed);
                        unwrap_log!(message.send_others(&id, manager), source, loop_vars.proxy);
                        if manager.is_empty() {
                            cleanup_backend();
                            *loop_vars.control_flow = ControlFlow::Exit;
                        }
                    }
                }
            }
            WindowAction::Create(window_config) => {
                let window_res = manager.create_window(
                    window_config,
                    loop_vars.proxy.clone(),
                    loop_vars.event_loop,
                    loop_vars.context,
                );
                let id = unwrap_log!(window_res, source, loop_vars.proxy);
                let number_id = id.to_number(manager);
                let created = InfoMessage::Created(number_id);
                BackendMessage::Info(created.clone()).send().unwrap();
                let message = WindowMessage::Info(created);
                unwrap_log!(message.send_others(&id, manager), source, loop_vars.proxy);
            }
        }
    }
}
