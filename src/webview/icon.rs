use crate::webview::enums::{backend::backend_message::BackendMessage, webview::info::InfoMessage};
use image::open;
use tao::window::Icon;

pub fn load_icon(icon: &str) -> Option<Icon> {
    let image_result = open(icon);
    let (icon_rgba, icon_width, icon_height) = {
        let image = match image_result {
            Ok(data) => data.into_rgba8(),
            Err(_err) => {
                BackendMessage::Info(InfoMessage::Error("Could not load icon".to_string()))
                    .send()
                    .unwrap();
                return None;
            }
        };
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    Icon::from_rgba(icon_rgba, icon_width, icon_height).ok()
}
