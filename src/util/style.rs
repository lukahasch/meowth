use std::ops::Add;

use iced::{
    Background, Border, Color,
    widget::{button, container, rule, svg},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct FrontColor(Color);

pub fn front_color<T>(color: Color) -> T
where
    T: From<FrontColor>,
{
    FrontColor(color).into()
}

impl From<FrontColor> for Color {
    fn from(value: FrontColor) -> Self {
        value.0
    }
}

impl From<Color> for FrontColor {
    fn from(value: Color) -> Self {
        FrontColor(value)
    }
}

impl From<FrontColor> for container::Style {
    fn from(value: FrontColor) -> Self {
        container::Style {
            text_color: Some(value.into()),
            ..Default::default()
        }
    }
}

impl From<FrontColor> for rule::Style {
    fn from(value: FrontColor) -> Self {
        rule::Style {
            color: value.into(),
            radius: 1.0.into(),
            fill_mode: rule::FillMode::Full,
            snap: true,
        }
    }
}

impl From<FrontColor> for svg::Style {
    fn from(value: FrontColor) -> Self {
        svg::Style {
            color: Some(value.into()),
        }
    }
}

impl From<FrontColor> for button::Style {
    fn from(value: FrontColor) -> Self {
        button::Style {
            text_color: value.into(),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct BackColor(Color);

pub fn back_color<T>(color: Color) -> T
where
    T: From<BackColor>,
{
    BackColor(color).into()
}

impl From<BackColor> for Color {
    fn from(value: BackColor) -> Self {
        value.0
    }
}

impl From<Color> for BackColor {
    fn from(value: Color) -> Self {
        BackColor(value)
    }
}

impl From<BackColor> for container::Style {
    fn from(value: BackColor) -> Self {
        container::Style {
            background: Some(iced::Background::Color(value.into())),
            ..Default::default()
        }
    }
}

impl From<BackColor> for button::Style {
    fn from(value: BackColor) -> Self {
        button::Style {
            background: Some(iced::Background::Color(value.into())),
            ..Default::default()
        }
    }
}

pub struct Style {
    pub front: Option<FrontColor>,
    pub back: Option<BackColor>,
    pub border: Option<Border>,
}

impl Add<BackColor> for FrontColor {
    type Output = Style;

    fn add(self, rhs: BackColor) -> Self::Output {
        Style {
            front: Some(self),
            back: Some(rhs),
            border: None,
        }
    }
}

impl Add<FrontColor> for BackColor {
    type Output = Style;

    fn add(self, rhs: FrontColor) -> Self::Output {
        Style {
            front: Some(rhs),
            back: Some(self),
            border: None,
        }
    }
}

impl Add<FrontColor> for Border {
    type Output = Style;

    fn add(self, rhs: FrontColor) -> Self::Output {
        Style {
            front: Some(rhs),
            back: None,
            border: Some(self),
        }
    }
}

impl Add<Border> for FrontColor {
    type Output = Style;

    fn add(self, rhs: Border) -> Self::Output {
        Style {
            front: Some(self),
            back: None,
            border: Some(rhs),
        }
    }
}

impl Add<Border> for Style {
    type Output = Style;

    fn add(self, rhs: Border) -> Self::Output {
        Style {
            front: self.front,
            back: self.back,
            border: Some(rhs),
        }
    }
}

impl Add<Border> for BackColor {
    type Output = Style;

    fn add(self, rhs: Border) -> Self::Output {
        Style {
            front: None,
            back: Some(self),
            border: Some(rhs),
        }
    }
}

impl Add<BackColor> for Border {
    type Output = Style;

    fn add(self, rhs: BackColor) -> Self::Output {
        Style {
            front: None,
            back: Some(rhs),
            border: Some(self),
        }
    }
}

impl Add<Style> for Border {
    type Output = Style;

    fn add(self, rhs: Style) -> Self::Output {
        Style {
            front: rhs.front,
            back: rhs.back,
            border: Some(self),
        }
    }
}

impl Add<BackColor> for Style {
    type Output = Style;

    fn add(self, rhs: BackColor) -> Self::Output {
        Style {
            front: self.front,
            back: Some(rhs),
            border: self.border,
        }
    }
}

impl From<Style> for container::Style {
    fn from(value: Style) -> Self {
        container::Style {
            background: value.back.map(|c| c.into()).map(Background::Color),
            text_color: value.front.map(|c| c.into()),
            border: match value.border {
                Some(border) => border,
                None => container::Style::default().border,
            },
            ..Default::default()
        }
    }
}

impl From<Style> for button::Style {
    fn from(value: Style) -> Self {
        button::Style {
            background: value.back.map(|c| c.into()).map(Background::Color),
            text_color: match value.front {
                Some(front) => front.into(),
                None => button::Style::default().text_color,
            },
            border: match value.border {
                Some(border) => border,
                None => button::Style::default().border,
            },
            ..Default::default()
        }
    }
}
