use crate::webview::webview_manager::{WebViewManager, WindowManagerId};
use anyhow::{Context, Result};

pub trait Script {
    fn evaluate(self, id: &WindowManagerId, manager: &WebViewManager) -> Result<()>;
}
impl Script for String {
    fn evaluate(self, id: &WindowManagerId, manager: &WebViewManager) -> Result<()> {
        let webview = manager.get(id).context("Target window not found")?;
        webview.view.evaluate_script(&self)?;
        Ok(())
    }
}
