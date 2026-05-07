use std::time::Instant;

use serde::{Deserialize, Serialize};

use crate::{
    utils::{Id, Map},
    world::Fixture,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Logic {
    pub cues: Map<Cue>,
    pub canvases: Map<Canvas>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cue {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub canvas: Id<Canvas>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub kind: Kind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Kind {
    Number,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Canvas {
    pub nodes: Map<Node>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub position: (f32, f32),
    pub inputs: Box<[Port]>,
    pub outputs: Box<[(Port, Vec<Connection>)]>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Port {
    pub name: String,
    pub kind: Kind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connection {
    pub node: Id<Node>,
    pub port: usize,
    pub kind: ConnectionKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConnectionKind {
    Direct,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Message {
    #[serde(skip)]
    Time(Instant),
    SetChannel(Id<Fixture>, usize),
    Number(f64),
}

impl Logic {
    pub fn new() -> Self {
        Logic {
            cues: Map::new(),
            canvases: Map::new(),
        }
    }
}

impl Default for Logic {
    fn default() -> Self {
        Self::new()
    }
}
