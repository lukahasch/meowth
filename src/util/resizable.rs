use std::ops::{Deref, DerefMut};

use derive_more::From;
use iced::{
    Alignment, Color, Element,
    Length::Fill,
    Subscription,
    event::listen,
    mouse::Interaction,
    widget::{Container, container, mouse_area, rule, stack},
};
use serde::{Deserialize, Serialize};
use tap::Pipe;

use crate::{
    SetInteraction,
    state::{State, catpuccin_machiato},
    util::{Output, back_color, component::Component, front_color},
};

#[allow(clippy::type_complexity)]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Resizable<T> {
    pub inner: T,
    resizing: bool,
    starting_position: Option<u32>,
    pub size: u32,
    pub default_size: u32,
    hidden: bool,
    pub direction: Direction,
    grab_size: u32,
}

impl<T> Deref for Resizable<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for Resizable<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, From)]
pub enum Message<M>
where
    M: Clone + Send,
{
    Inner(M),
    EndResize,
    StartResize,
    #[from(skip)]
    ResizeTo(u32),
    ToggleHide,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Hash)]
pub enum Direction {
    HoritonalStart,
    HoriztontalEnd,
    VerticalStart,
    VerticalEnd,
}

impl<T> Resizable<T> {
    pub fn new(inner: T, size: u32, direction: Direction) -> Self {
        Resizable {
            inner,
            resizing: false,
            starting_position: None,
            size,
            hidden: false,
            direction,
            grab_size: 10,
            default_size: size,
        }
    }
}

impl<T, O> Component<O> for Resizable<T>
where
    T: Component<O>,
    O: Send + 'static,
    O: From<SetInteraction>,
    T::Message: Clone + Send,
{
    type Message = Message<T::Message>;
    type Input<'a> = (Color, T::Input<'a>);

    fn update(
        &mut self,
        message: Self::Message,
        state: &State,
        (_, input): Self::Input<'_>,
    ) -> Output<O> {
        match message {
            Message::Inner(inner) => return self.inner.update(inner, state, input),
            Message::StartResize => {
                self.resizing = true;
                self.starting_position = None;
                return Output::message(O::from(SetInteraction::Some(match self.direction {
                    Direction::HoritonalStart | Direction::HoriztontalEnd => {
                        Interaction::ResizingHorizontally
                    }
                    Direction::VerticalStart | Direction::VerticalEnd => {
                        Interaction::ResizingVertically
                    }
                })));
            }
            Message::EndResize => {
                self.resizing = false;
                if self.size < 7 {
                    self.hidden = true;
                }
                return Output::message(O::from(SetInteraction::None));
            }
            Message::ResizeTo(pos) => {
                if let Some(start) = self.starting_position {
                    let delta = start as i32 - pos as i32;
                    self.size = ((self.size as i32) - delta).unsigned_abs();
                }
                self.starting_position = Some(pos);
            }
            Message::ToggleHide => {
                self.hidden = !self.hidden;
                if !self.hidden && self.size < 7 {
                    self.size = self.default_size;
                }
            }
        }
        Output::none()
    }

    fn view<'a>(
        &'a self,
        state: &'a State,
        (color, input): Self::Input<'a>,
    ) -> iced::Element<'a, Self::Message> {
        let size = if self.hidden { 0 } else { self.size };
        stack([
            container(self.inner.view(state, input))
                .pipe(|e: Container<_>| match self.direction {
                    Direction::HoritonalStart | Direction::HoriztontalEnd => e.width(size),
                    Direction::VerticalStart | Direction::VerticalEnd => e.height(size),
                })
                .pipe(Element::from)
                .map(Message::Inner),
            container(
                mouse_area(
                    container(
                        match self.direction {
                            Direction::HoritonalStart | Direction::HoriztontalEnd => {
                                rule::vertical(1)
                            }
                            Direction::VerticalStart | Direction::VerticalEnd => {
                                rule::horizontal(1)
                            }
                        }
                        .style(move |_| front_color(color)),
                    )
                    .width(match self.direction {
                        Direction::HoritonalStart | Direction::HoriztontalEnd => {
                            self.grab_size.into()
                        }
                        Direction::VerticalStart | Direction::VerticalEnd => Fill,
                    })
                    .height(match self.direction {
                        Direction::HoritonalStart | Direction::HoriztontalEnd => Fill,
                        Direction::VerticalStart | Direction::VerticalEnd => self.grab_size.into(),
                    })
                    .align_y(match self.direction {
                        Direction::VerticalStart => Alignment::Start,
                        Direction::VerticalEnd => Alignment::End,
                        _ => Alignment::Center,
                    })
                    .align_x(match self.direction {
                        Direction::HoritonalStart => Alignment::Start,
                        Direction::HoriztontalEnd => Alignment::End,
                        _ => Alignment::Center,
                    }),
                )
                .on_press(Message::StartResize)
                .interaction(match self.direction {
                    Direction::HoritonalStart | Direction::HoriztontalEnd => {
                        Interaction::ResizingHorizontally
                    }
                    Direction::VerticalStart | Direction::VerticalEnd => {
                        Interaction::ResizingVertically
                    }
                }),
            )
            .align_y(match self.direction {
                Direction::VerticalStart => Alignment::Start,
                Direction::VerticalEnd => Alignment::End,
                _ => Alignment::Center,
            })
            .align_x(match self.direction {
                Direction::HoritonalStart => Alignment::Start,
                Direction::HoriztontalEnd => Alignment::End,
                _ => Alignment::Center,
            })
            .width(Fill)
            .width(Fill)
            .into(),
        ])
        .pipe(|e| match self.direction {
            Direction::HoritonalStart | Direction::HoriztontalEnd => e.width(size),
            Direction::VerticalStart | Direction::VerticalEnd => e.height(size),
        })
        .clip(true)
        .into()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        if self.resizing {
            listen()
                .with(self.direction.clone())
                .filter_map(move |(dir, event)| match event {
                    iced::Event::Mouse(iced::mouse::Event::CursorMoved { position }) => {
                        Some(Message::ResizeTo(match dir {
                            Direction::HoritonalStart | Direction::HoriztontalEnd => {
                                position.x as u32
                            }
                            Direction::VerticalStart | Direction::VerticalEnd => position.y as u32,
                        }))
                    }
                    iced::Event::Mouse(iced::mouse::Event::ButtonReleased(_)) => {
                        Some(Message::EndResize)
                    }
                    _ => None,
                })
        } else {
            self.inner.subscription().map(Message::Inner)
        }
    }
}
