use crate::webview::{enums::webview::info::InfoMessage, std::send_backendin};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum BackendMessage {
    Ready(String),
    Info(InfoMessage),
    Message { from: i32, data: String },
}

impl BackendMessage {
    pub fn send(self) -> Result<()> {
        send_backendin(serde_json::to_string(&self)?)
    }
}
