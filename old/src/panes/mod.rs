use serde::{Deserialize, Serialize};

pub mod fixtures;
pub mod new;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleBar {}

pub enum TitleBarResponse {
    Close,
    Drag,
}
