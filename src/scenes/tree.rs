use std::time::Duration;

use iced::{
    Alignment::Center,
    Color, Element,
    Length::{Fill, Shrink},
    Task,
    widget::{column, container, mouse_area, row, rule, stack, svg, text, text::Wrapping},
};
use serde::{Deserialize, Serialize};

use crate::{
    assets,
    scenes::scene::Scene,
    state::State,
    util::{Id, Output, Storage, back_color, component::Component, front_color, resizable},
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SceneTree {
    pub top: Id<Scene>,
    pub focused: Id<Scene>,
    pub playing: Option<Id<Scene>>,
    pub hovered: Option<Id<Scene>>,
    pub pressed: Option<Id<Scene>>,
    pub modal: Option<Id<Scene>>,
}

pub const ROW_HEIGHT: u32 = 24;
pub const INDENT_WIDTH: u32 = 20;

impl SceneTree {
    #[allow(clippy::new_without_default)]
    pub fn new(top: Id<Scene>) -> Self {
        SceneTree {
            top,
            focused: top,
            playing: None,
            hovered: None,
            pressed: None,
            modal: None,
        }
    }

    fn names<'a>(
        &'a self,
        scenes: &'a Storage<Scene>,
        id: Id<Scene>,
        state: &'a State,
    ) -> Element<'a, Message> {
        let scene = &scenes[&id];
        let icon = Self::icon(state, scene);
        let name = container(
            row![icon, " ", text(&scene.name).wrapping(Wrapping::None)]
                .align_y(Center)
                .height(ROW_HEIGHT),
        )
        .style(move |_| {
            if self.playing(id) {
                front_color(state.theme.palette.red)
            } else {
                front_color(state.theme.front.text)
            }
        });
        if !scene.is_folder() {
            return name.into();
        }
        let rule =
            container(rule::vertical(1).style(|_| front_color(state.theme.back.very_dark_accent)))
                .width(INDENT_WIDTH)
                .height(Fill)
                .align_x(Center);
        let children = column(
            scene
                .children
                .iter()
                .map(|child| self.names(scenes, *child, state)),
        );
        column![name, row![rule, children]].height(Shrink).into()
    }

    fn containers<'a>(
        &'a self,
        scenes: &'a Storage<Scene>,
        id: Id<Scene>,
        state: &'a State,
    ) -> Box<dyn Iterator<Item = Element<'a, Message>> + 'a> {
        Box::new(
            std::iter::once(
                mouse_area(
                    container("")
                        .width(Fill)
                        .height(ROW_HEIGHT)
                        .style(move |_| {
                            if self.pressed(id) {
                                back_color(state.theme.back.very_dark)
                            } else if self.hovered(id) {
                                back_color(state.theme.back.very_dark_accent)
                            } else if self.focused(id) {
                                back_color(state.theme.back.very_dark_accent_understated)
                            } else {
                                back_color(Color::TRANSPARENT)
                            }
                        }),
                )
                .on_double_click(Message::Play(id))
                .on_press(Message::Focus(id))
                .on_right_press(Message::Modal(id))
                .on_enter(Message::Hover(id))
                .on_exit(Message::Unhover(id))
                .into(),
            )
            .chain(
                scenes[&id]
                    .children
                    .iter()
                    .flat_map(|child| self.containers(scenes, *child, state)),
            ),
        )
    }

    fn icon<'a>(state: &'a State, scene: &'a Scene) -> svg::Svg<'a> {
        if scene.is_folder() {
            assets::icon::folder()
        } else {
            assets::icon::file()
        }
        .width(INDENT_WIDTH)
        .style(|_, _| front_color(state.theme.front.text))
    }

    pub fn focused(&self, id: Id<Scene>) -> bool {
        self.focused == id
    }

    pub fn playing(&self, id: Id<Scene>) -> bool {
        self.playing == Some(id)
    }

    pub fn hovered(&self, id: Id<Scene>) -> bool {
        self.hovered == Some(id)
    }

    pub fn pressed(&self, id: Id<Scene>) -> bool {
        self.pressed == Some(id)
    }

    fn unpress<M>(&mut self, id: Id<Scene>) -> Output<M>
    where
        M: Send + 'static + From<resizable::Message<Message>>,
    {
        Output::task(Task::future(async move {
            smol::Timer::after(Duration::from_millis(75)).await;
            M::from(resizable::Message::Inner(Message::Unpress(id)))
        }))
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Message {
    Focus(Id<Scene>),
    Play(Id<Scene>),
    Hover(Id<Scene>),
    Unhover(Id<Scene>),
    Unpress(Id<Scene>),
    Modal(Id<Scene>),
    CloseModal,
}

impl<M> Component<M> for SceneTree
where
    M: Send + 'static + From<resizable::Message<Message>>,
{
    type Message = Message;
    type Input<'a> = &'a Storage<Scene>;

    fn update<'a>(
        &'a mut self,
        message: Self::Message,
        _state: &'a State,
        _data: Self::Input<'a>,
    ) -> Output<M> {
        match message {
            Message::Focus(id) => {
                self.focused = id;
                self.pressed = Some(id);
                return self.unpress(id);
            }
            Message::Play(id) => {
                self.playing = Some(id);
                self.pressed = Some(id);
                return self.unpress(id);
            }
            Message::Hover(id) => {
                self.hovered = Some(id);
            }
            Message::Unhover(id) => {
                if self.hovered(id) {
                    self.hovered = None;
                }
            }
            Message::Unpress(id) => {
                if self.pressed(id) {
                    self.pressed = None;
                }
            }
            Message::Modal(id) => {
                self.modal = Some(id);
            }
            Message::CloseModal => {
                self.modal = None;
            }
        }
        Output::none()
    }

    fn view<'a>(
        &'a self,
        state: &'a State,
        data: Self::Input<'a>,
    ) -> iced::Element<'a, Self::Message> {
        container(stack([
            column(self.containers(data, self.top, state))
                .width(Fill)
                .height(Fill)
                .into(),
            container(self.names(data, self.top, state)).into(),
        ]))
        .style(|_| back_color(state.theme.back.dark))
        .into()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::Subscription::none()
    }
}
