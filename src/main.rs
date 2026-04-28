#![feature(specialization)]

use std::time::{Duration, SystemTime};

use derive_more::From;
use iced::{
    Border, Element,
    Length::{Fill, Shrink},
    Subscription, Task, Theme,
    mouse::Interaction,
    time,
    widget::{button, button::Status, column, container, mouse_area, row},
    window::close_requests,
};
use serde::{Deserialize, Serialize};
use tap::Pipe;

use crate::{
    scenes::Scenes,
    state::State,
    util::{
        BackColor, back_color,
        component::{Comp, Component},
        front_color,
        output::Output,
    },
};

pub mod assets;
pub mod scenes;
pub mod state;
pub mod util;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Meowth {
    pub state: State,
    pub scenes: Comp<Message, Scenes>,
    #[serde(skip)]
    pub history: Vec<Message>,
    #[serde(skip)]
    pub interaction: Option<Interaction>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, From)]
pub enum SetInteraction {
    #[serde(skip)]
    Some(Interaction),
    None,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, From)]
#[from(transparent)]
pub enum Message {
    ScenesMessage(scenes::Message),
    #[serde(skip)]
    SetInteraction(SetInteraction),
    Timer(Timer),
    Close,
    FileDialog,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Timer {
    DMX512,
}

impl Timer {
    pub fn subscription(&self) -> Subscription<Message> {
        match self {
            Timer::DMX512 => time::every(Duration::from_secs(1))
                .map(|_| Timer::DMX512)
                .map(Message::Timer),
        }
    }
}

impl Meowth {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            scenes: Scenes::new().into(),
            state: State::new(),
            history: Vec::new(),
            interaction: None,
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        mouse_area(column![
            container(row![
                button(self.state.name.as_ref())
                    .on_press(Message::FileDialog)
                    .style(|_, status| match status {
                        Status::Hovered => front_color(self.state.theme.front.primary),
                        _ => front_color(self.state.theme.front.text),
                    },)
            ])
            .width(Fill)
            .height(Shrink)
            .style(
                |_| (back_color::<BackColor>(self.state.theme.back.very_dark)
                    + Border {
                        width: 1.0,
                        radius: 0.0.into(),
                        color: self.state.theme.back.dark_accent,
                    })
                .into()
            ),
            container(
                self.scenes
                    .view(&self.state, ())
                    .map(Message::ScenesMessage)
            )
            .width(Fill)
            .height(Fill),
            container(row![
                button("Scenes")
                    .on_press(Message::ScenesMessage(scenes::Message::ToggleHide))
                    .style(|_, status| match status {
                        Status::Hovered => front_color(self.state.theme.front.primary),
                        _ => front_color(self.state.theme.front.text),
                    },)
            ])
            .width(Fill)
            .height(Shrink)
            .style(
                |_| (back_color::<BackColor>(self.state.theme.back.very_dark)
                    + Border {
                        width: 1.0,
                        radius: 0.0.into(),
                        color: self.state.theme.back.dark_accent,
                    })
                .into()
            )
        ])
        .pipe(|e| {
            if let Some(interaction) = self.interaction {
                e.interaction(interaction)
            } else {
                e
            }
        })
        .into()
    }

    pub fn update(&mut self, message: Message) -> Output<Message> {
        match message {
            Message::ScenesMessage(message) => self.scenes.update(message, &self.state, ()),
            Message::Timer(timer) => match timer {
                Timer::DMX512 => Output::none(), // TODO: Implement DMX512 timer
            },
            Message::Close => {
                let serialised = ron::to_string(self).expect("Failed to serialize Meowth state");
                let name = format!(
                    "{} — Closed {}.meowth",
                    self.state.name,
                    humantime::format_rfc3339(SystemTime::now())
                );
                //std::fs::write(name, serialised).expect("Failed to write Meowth state to file");
                Output::task(iced::exit())
            }
            Message::SetInteraction(interaction) => {
                match interaction {
                    SetInteraction::Some(interaction) => self.interaction = Some(interaction),
                    SetInteraction::None => self.interaction = None,
                }
                Output::none()
            }
            Message::FileDialog => {
                let file = native_dialog::DialogBuilder::file()
                    .add_filter("Meowth", [".meowth", ".ron"])
                    .set_title("What file do you want to open?")
                    .open_single_file()
                    .show()
                    .unwrap()
                    .unwrap();
                let contents = std::fs::read_to_string(file).unwrap();
                let meowth = ron::from_str(&contents).unwrap();
                *self = meowth;
                Output::none()
            }
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(vec![
            Timer::DMX512.subscription(),
            self.scenes.subscription().map(Message::ScenesMessage),
            close_requests().map(|_| Message::Close),
        ])
    }
}

fn update(meowth: &mut Meowth, msg: Message) -> Task<Message> {
    meowth
        .update(msg)
        .realise(&mut |msg| meowth.update(msg))
        .unwrap_or(Task::none())
}

fn main() -> iced::Result {
    dioxus_devtools::connect_subsecond();
    crate::util::args::test();
    return Ok(());
    /*iced::application(Meowth::new, update, Meowth::view)
    .theme(|meowth: &Meowth| {
        Theme::custom(
            "Meowth",
            iced::theme::Palette {
                background: meowth.state.theme.back.normal,
                text: meowth.state.theme.front.text,
                primary: meowth.state.theme.front.primary,
                success: meowth.state.theme.front.success,
                warning: meowth.state.theme.front.warning,
                danger: meowth.state.theme.front.error,
            },
        )
    })
    .title(|meowth: &Meowth| meowth.state.name.clone())
    .subscription(|meowth| meowth.subscription())
    .exit_on_close_request(false)
    .run()*/
}
