use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct WindowConfig {
    pub title: String,
    pub url: String,
    pub icon: Option<String>,
    pub no_decorations: bool,
    pub dev_tools: bool,
    pub transparent: bool,
    pub width: i16,
    pub height: i16,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: "WebView".to_owned(),
            url: "index.html".to_owned(),
            icon: None,
            dev_tools: false,
            no_decorations: false,
            transparent: false,
            width: 680,
            height: 480,
        }
    }
}
