pub mod fixtures;
pub mod theme;

pub use fixtures::*;
use serde::{Deserialize, Serialize};
pub use theme::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct State {
    pub name: String,
    pub fixtures: Fixtures,
    pub theme: Theme,
    pub text_size: u16,
}

impl State {
    #[allow(clippy::new_without_default)]
    #[must_use]
    pub fn new() -> Self {
        State {
            name: String::from("Unnamed"),
            fixtures: Fixtures::new(),
            theme: theme::catpuccin_machiato(),
            text_size: 16,
        }
    }
}
