use iced::Color;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub back: Background,
    pub front: Foreground,
    pub palette: Palette,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Background {
    pub normal: Color,
    pub normal_accent: Color,
    pub dark: Color,
    pub dark_accent: Color,
    pub dark_accent_understated: Color,
    pub very_dark: Color,
    pub very_dark_accent: Color,
    pub very_dark_accent_understated: Color,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Foreground {
    pub text: Color,
    pub success: Color,
    pub warning: Color,
    pub error: Color,
    pub primary: Color,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Palette {
    pub purple: Color,
    pub blue: Color,
    pub green: Color,
    pub yellow: Color,
    pub orange: Color,
    pub red: Color,
}

pub fn catpuccin_machiato() -> Theme {
    Theme {
        name: String::from("Catpuccin Machiato"),
        back: Background {
            normal: Color::from_rgb8(36, 38, 58),
            normal_accent: Color::from_rgb8(60, 63, 86),
            dark: Color::from_rgb8(31, 31, 47),
            dark_accent: Color::from_rgb8(44, 48, 67),
            dark_accent_understated: Color::from_rgb8(37, 40, 57),
            very_dark: Color::from_rgb8(25, 24, 38),
            very_dark_accent: Color::from_rgb8(55, 58, 79),
            very_dark_accent_understated: Color::from_rgb8(37, 40, 57),
        },
        front: Foreground {
            text: Color::from_rgb8(202, 211, 245),
            success: Color::from_rgb8(154, 202, 140),
            warning: Color::from_rgb8(175, 158, 125),
            error: Color::from_rgb8(227, 130, 146),
            primary: Color::from_rgb8(198, 160, 246),
        },
        palette: Palette {
            purple: Color::from_rgb8(198, 160, 246),
            blue: Color::from_rgb8(127, 159, 224),
            green: Color::from_rgb8(166, 218, 149),
            yellow: Color::from_rgb8(238, 212, 159),
            orange: Color::from_rgb8(245, 169, 127),
            red: Color::from_rgb8(232, 133, 148),
        },
    }
}
