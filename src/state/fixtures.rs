use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::util::storage::{Id, Storage};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Fixtures {
    pub fixtures: Storage<Fixture>,
    pub models: Storage<Model>,
    pub names: Storage<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Fixture {
    pub name: String,
    pub description: String,
    pub model: Id<Model>,
    pub parameters: Vec<Parameter>,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Model {
    pub name: String,
    pub description: String,
    pub parameters: Vec<Parameter>,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Attribute {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[rustfmt::skip]
pub enum Parameter {
    I8 { name: Id<String>, value: i8 },
    I16 { name: Id<String>, value: i16 },
    I32 { name: Id<String>, value: i32 },
    U8 { name: Id<String>, value: u8 },
    U16 { name: Id<String>, value: u16 },
    U32 { name: Id<String>, value: u32 },
    F32 { name: Id<String>, value: f32 },
    F64 { name: Id<String>, value: f64 },
    Gobo { name: Id<String>, mappings: Vec<(Id<String>, u16)> },
}

impl Fixtures {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Fixtures {
            fixtures: Storage::new(),
            models: Storage::new(),
            names: Storage::new(),
        }
    }
}
