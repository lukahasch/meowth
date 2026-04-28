use iced::{Element, widget::container};
use serde::{Deserialize, Serialize};

use crate::state::State;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Setup {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Message {}

impl Setup {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Setup {}
    }

    pub fn update(&mut self, message: Message, state: &State) {}

    pub fn view<'a>(&'a self, state: &'a State) -> Element<'a, Message> {
        container("World").into()
    }

    pub fn title<'a>(&'a self, state: &'a State) -> Element<'a, Message> {
        container("Setup").into()
    }

    pub fn actions<'a>(&'a self, state: &'a State) -> Element<'a, Message> {
        container(" ").into()
    }

    pub fn active<'a>(&'a self, state: &'a State) -> Option<Element<'a, Message>> {
        None
    }
}
