use iced::{
    Border, Element,
    Length::{Fill, Shrink},
    widget::{column, container, rule},
};
use serde::{Deserialize, Serialize};

use crate::{scenes::scene::view::setup::Setup, state::State, util::output::Output};

pub mod setup;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum View {
    Setup(setup::Setup),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Message {
    SetupMessage(setup::Message),
}

impl View {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        View::Setup(Setup::new())
    }

    pub fn update(&mut self, message: Message, state: &State) -> Output<Message> {
        Output::none()
    }

    pub fn view<'a>(&'a self, state: &'a State) -> Element<'a, Message> {
        let view = match self {
            View::Setup(world) => world.view(state).map(Message::SetupMessage),
        };
        let bar = column![
            container(self.title(state))
                .padding(8)
                .width(Fill)
                .height(Shrink),
            rule::horizontal(1).style(|_| rule::Style {
                color: state.theme.palette.purple,
                radius: 1.0.into(),
                fill_mode: rule::FillMode::Full,
                snap: true
            })
        ];
        column![bar, view].into()
    }

    pub fn title<'a>(&'a self, state: &'a State) -> Element<'a, Message> {
        match self {
            View::Setup(world) => world.title(state).map(Message::SetupMessage),
        }
    }

    pub fn actions<'a>(&'a self, state: &'a State) -> Element<'a, Message> {
        match self {
            View::Setup(world) => world.actions(state).map(Message::SetupMessage),
        }
    }

    pub fn active<'a>(&'a self, state: &'a State) -> Option<Element<'a, Message>> {
        match self {
            View::Setup(world) => world.active(state).map(|e| e.map(Message::SetupMessage)),
        }
    }
}
