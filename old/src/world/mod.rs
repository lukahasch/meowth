use std::time::Instant;

use serde::{Deserialize, Serialize};

use crate::utils::{Id, Map};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct World {
    pub models: Map<Model>,
    pub fixtures: Map<Fixture>,
    pub outputs: Map<Output>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fixture {
    pub model: Id<Model>,
    pub name: String,
    pub channels: Box<[Channel]>,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Model {
    pub name: String,
    pub modes: Vec<Mode>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mode {
    pub name: String,
    pub channels: Box<[Channel]>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Channel {
    pub name: String,
    pub value: u8,
    #[serde(skip)]
    #[serde(default = "Instant::now")]
    pub set_time: Instant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Attribute {
    DMXOutput {
        output: Id<Output>,
        channel_start: usize,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Output {
    DMXDummy,
}

impl World {
    pub fn new() -> Self {
        World {
            models: Map::new(),
            fixtures: Map::new(),
            outputs: Map::new(),
        }
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}
