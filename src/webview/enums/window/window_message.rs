use crate::{
    state::STATE,
    webview::{
        enums::webview::{info::InfoMessage, target::Target},
        script::Script,
        webview_manager::{WebViewManager, WindowManagerId},
    },
};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum WindowMessage {
    Info(InfoMessage),
    Message { from: Target, data: String },
}

impl WindowMessage {
    pub fn send(&self, id: &WindowManagerId, manager: &WebViewManager) -> Result<()> {
        let message = serde_json::to_string(&self)?;
        if STATE.verbose {
            let id_number = id.to_number(manager);
            println!("Webview({id_number}) <- {message}");
        }
        let code = format!("window.deutron.triggerEvent({message})");
        code.evaluate(id, manager)?;
        Ok(())
    }
    pub fn send_others(&self, id: &WindowManagerId, manager: &WebViewManager) -> Result<()> {
        for (target, _) in manager.iter_ids() {
            let target = &WindowManagerId::Number(*target);
            if target != id {
                self.send(target, manager)?
            }
        }
        Ok(())
    }
}
