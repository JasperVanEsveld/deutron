use rand::rngs::ThreadRng;
use rand::Rng;
use std::{borrow::Cow, collections::HashMap};

use tao::{
    event::{Event, StartCause, WindowEvent as WryWindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder, EventLoopProxy, EventLoopWindowTarget},
    window::{Window, WindowBuilder, WindowId},
};
use wry::{dpi::LogicalSize, http::Request, WebContext, WebView, WebViewBuilder};

use crate::{
    cleanup::cleanup_backend,
    state::STATE,
    unwrap_log,
    webview::{
        enums::{
            backend::backend_message::BackendMessage,
            webview::{info::InfoMessage, target::Target, webview_action::WebViewAction},
            window::{
                window_config::WindowConfig, window_event::WindowEvent,
                window_message::WindowMessage,
            },
        },
        icon::load_icon,
        serve::{serve, ResolverInfo},
        std::{listen_backenderr, listen_backendout},
    },
};

#[derive(Debug, PartialEq, Clone)]
pub enum WindowManagerId {
    Number(i32),
    Tao(WindowId),
}

impl WindowManagerId {
    pub fn to_number(&self, manager: &WebViewManager) -> i32 {
        match self {
            WindowManagerId::Number(id) => *id,
            WindowManagerId::Tao(id) => *manager.ids.get_by_right(id).unwrap(),
        }
    }
}

pub struct WebViewExt {
    pub view: WebView,
    pub window: Window,
}

pub struct WebViewManager {
    rng: ThreadRng,
    ids: bimap::BiMap<i32, WindowId>,
    webviews: HashMap<i32, WebViewExt>,
}

pub struct LoopVariables<'a> {
    pub proxy: &'a EventLoopProxy<WebViewAction>,
    pub event_loop: &'a EventLoopWindowTarget<WebViewAction>,
    pub context: &'a mut WebContext,
    pub control_flow: &'a mut ControlFlow,
}

impl WebViewManager {
    pub fn new() -> WebViewManager {
        WebViewManager {
            rng: rand::rng(),
            ids: bimap::BiMap::<i32, WindowId>::new(),
            webviews: HashMap::<i32, WebViewExt>::new(),
        }
    }

    pub fn gen_id(&mut self) -> i32 {
        loop {
            let id = self.rng.random::<i32>();
            if !self.ids.contains_left(&id) {
                return id;
            }
        }
    }
    pub fn get(&self, id: &WindowManagerId) -> Option<&WebViewExt> {
        let id = id.to_number(self);
        self.webviews.get(&id)
    }

    pub fn set(&mut self, id: i32, window_id: WindowId, webview: WebViewExt) -> i32 {
        self.ids.insert(id, window_id);
        self.webviews.insert(id, webview);
        id
    }

    pub fn close(&mut self, id: WindowManagerId) {
        self.get(&id).unwrap().window.set_visible(false);
        let id = id.to_number(self);
        self.webviews.remove(&id);
        self.ids.remove_by_left(&id);
    }

    pub fn is_empty(&self) -> bool {
        self.webviews.is_empty()
    }

    pub fn iter_ids(&self) -> bimap::hash::Iter<'_, i32, WindowId> {
        self.ids.iter()
    }

    pub fn iter_webviews(&self) -> std::collections::hash_map::Iter<'_, i32, WebViewExt> {
        self.webviews.iter()
    }
}

/**
 * Wry functions
 */
impl WebViewManager {
    pub fn create_window(
        &mut self,
        window_config: WindowConfig,
        proxy: EventLoopProxy<WebViewAction>,
        event_loop: &EventLoopWindowTarget<WebViewAction>,
        context: &mut WebContext,
    ) -> wry::Result<WindowManagerId> {
        let resolve = ResolverInfo {
            root: STATE.backend_dir.clone(),
        };
        let id = self.gen_id();
        let handler = move |req: Request<String>| {
            let req_str = req.body().as_str();
            if STATE.verbose {
                println!("Webview({id}) -> {req_str}");
            }
            let req_result = serde_json::from_str(&req_str);

            let event: WindowEvent = unwrap_log!(req_result, Target::Window(id), proxy);
            unwrap_log!(event.handle(id, &proxy));
        };

        let script = include_str!("deutron.js");
        let webview_icon = if let Some(icon_path) = &window_config.icon {
            icon_path
        } else {
            &STATE.icon.clone().into_os_string().into_string().unwrap()
        };
        let window_icon = load_icon(webview_icon);
        let transparent = window_config.transparent;
        #[cfg(windows)]
        let window = WindowBuilder::new()
            .with_visible(false)
            .with_window_icon(window_icon)
            .with_decorations(!window_config.no_decorations)
            .with_title(window_config.title)
            .with_transparent(transparent)
            .with_inner_size(LogicalSize::<i16>::new(
                window_config.width,
                window_config.height,
            ))
            .build(event_loop)
            .unwrap();

        #[cfg(not(windows))]
        let window = WindowBuilder::new()
            .with_window_icon(window_icon)
            .with_decorations(!window_config.no_decorations)
            .with_title(window_config.title)
            .with_transparent(transparent)
            .with_inner_size(LogicalSize::<i16>::new(
                window_config.width,
                window_config.height,
            ))
            .build(event_loop)
            .unwrap();

        let window_id = window.id();
        let webview = WebViewBuilder::new_with_web_context(context)
            .with_custom_protocol("local".to_string(), move |_id, request| {
                let response = match serve(&request, &resolve) {
                    Ok(response) => response,
                    Err(e) => {
                        BackendMessage::Info(InfoMessage::Error(e.to_string()))
                            .send()
                            .unwrap();
                        wry::http::Response::builder()
                            .status(404)
                            .body(Cow::from(vec![]))
                            .unwrap()
                    }
                };
                response
            })
            .with_navigation_handler(|url| url.starts_with("http://local.files/"))
            .with_new_window_req_handler(|url| url.starts_with("http://local.files/"))
            .with_url(&format!("local://files/{}", window_config.url))
            .with_ipc_handler(handler)
            .with_initialization_script(script)
            .with_transparent(transparent)
            .with_devtools(STATE.dev_tools || window_config.dev_tools)
            .build(&window)
            .unwrap();
        self.set(
            id,
            window_id,
            WebViewExt {
                view: webview,
                window,
            },
        );
        Ok(WindowManagerId::Number(id))
    }

    pub fn start(mut self, actions: Option<Vec<WebViewAction>>) {
        let event_loop = EventLoopBuilder::<WebViewAction>::with_user_event().build();
        let proxy = event_loop.create_proxy();
        let mut context = WebContext::new(Some(STATE.webview_dir.clone()));
        listen_backendout(proxy.clone());
        listen_backenderr(proxy.clone());
        for action in actions.unwrap_or(vec![]) {
            unwrap_log!(action.perform(&proxy));
        }
        event_loop.run(move |event, event_loop, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::NewEvents(StartCause::Init) => {
                    BackendMessage::Ready(
                        STATE
                            .backend_dir
                            .clone()
                            .into_os_string()
                            .into_string()
                            .unwrap(),
                    )
                    .send()
                    .unwrap();
                }
                Event::WindowEvent {
                    event, window_id, ..
                } => {
                    if event == WryWindowEvent::CloseRequested {
                        let id = WindowManagerId::Tao(window_id);
                        let number_id = id.to_number(&self);
                        self.close(id.clone());

                        let closed = InfoMessage::Closed(number_id);
                        BackendMessage::Info(closed.clone()).send().unwrap();
                        let message = WindowMessage::Info(closed);
                        unwrap_log!(
                            message.send_others(&id, &self),
                            Target::Window(number_id),
                            proxy
                        );
                        if self.is_empty() {
                            cleanup_backend();
                            *control_flow = ControlFlow::Exit;
                        }
                    }
                }
                Event::UserEvent(user_event) => {
                    let loop_vars = LoopVariables {
                        proxy: &proxy,
                        event_loop,
                        context: &mut context,
                        control_flow,
                    };
                    self.handle_user_events(user_event, loop_vars);
                }
                _ => {}
            }
        });
    }

    fn handle_user_events(&mut self, user_event: WebViewAction, loop_vars: LoopVariables) {
        match user_event {
            WebViewAction::Message(source, id, message) => {
                let id = WindowManagerId::Number(id);
                unwrap_log!(message.send(&id, self), source, loop_vars.proxy);
            }
            WebViewAction::Window(source, action) => {
                action.perform(source, self, loop_vars);
            }
            WebViewAction::Request(source, request) => {
                request.perform(source, self, loop_vars.proxy)
            }
        }
    }
}
