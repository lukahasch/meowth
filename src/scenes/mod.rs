use std::time::Duration;

use derive_more::From;
use iced::{
    Alignment::Center,
    Background, Color, Element,
    Length::{Fill, Shrink},
    Subscription,
    advanced::widget::Tree,
    border::radius,
    event::listen,
    mouse::Interaction,
    widget::{
        column, container, mouse_area, row, rule, stack, svg,
        text::{self, Wrapping},
    },
};
use serde::{Deserialize, Serialize};

use crate::{
    SetInteraction,
    assets::{self},
    scenes::{scene::Scene, tree::SceneTree},
    state::State,
    util::{
        back_color,
        component::{Comp, Component},
        front_color,
        output::Output,
        resizable::{self, Direction, Resizable},
        storage::{Id, Storage},
    },
};

pub mod scene;
pub mod tree;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Scenes {
    pub scenes: Storage<Scene>,
    pub tree: Comp<Message, Resizable<SceneTree>>,
}

type TreeMessage = resizable::Message<tree::Message>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, From)]
pub enum Message {
    SceneMessage(Id<Scene>, scene::Message),
    SetInteraction(SetInteraction),
    TreeMessage(TreeMessage),
    ToggleHide,
}

pub const ROW_HEIGHT: u32 = 24;
pub const INDENT_WIDTH: u32 = 20;

impl Scenes {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let mut scenes = Storage::new();
        let top = Scene::top(&mut scenes);
        let top = scenes.insert(top);
        Scenes {
            scenes,
            tree: Resizable::new(SceneTree::new(top), 150, Direction::HoriztontalEnd).into(),
        }
    }
}

impl<M> Component<M> for Scenes
where
    M: Send + 'static + From<Message> + From<SetInteraction>,
{
    type Message = Message;
    type Input<'a> = ();

    fn update<'a>(
        &'a mut self,
        message: Self::Message,
        state: &'a State,
        _: Self::Input<'a>,
    ) -> Output<M> {
        match message {
            Message::SceneMessage(id, message) => self
                .scenes
                .get_mut(&id)
                .unwrap()
                .update::<Message>(message, state)
                .map(M::from),
            Message::SetInteraction(inter) => Output::message(M::from(inter)),
            Message::ToggleHide => self
                .tree
                .update(
                    resizable::Message::ToggleHide,
                    state,
                    (state.theme.back.very_dark_accent, &self.scenes),
                )
                .map(M::from),
            Message::TreeMessage(message) => self
                .tree
                .update(
                    message,
                    state,
                    (state.theme.back.very_dark_accent, &self.scenes),
                )
                .map(M::from),
        }
    }

    fn view<'a>(
        &'a self,
        state: &'a State,
        _: Self::Input<'a>,
    ) -> iced::Element<'a, Self::Message> {
        row([
            self.tree
                .view(state, (state.theme.back.very_dark_accent, &self.scenes))
                .map(Message::TreeMessage),
            self.scenes[&self.tree.focused]
                .view(state)
                .map(|message| Message::SceneMessage(self.tree.focused, message)),
        ])
        .into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        self.tree.subscription().map(Message::TreeMessage)
    }
}
