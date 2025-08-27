use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum WindowControl {
    Fullscreen,
    Maximize,
    Minimize,
    Drag,
    Close,
}
