pub mod icon {
    use iced::widget::{Svg, svg::Handle};

    pub const FILE: &[u8] = include_bytes!("file.svg");

    pub fn file<'a>() -> Svg<'a> {
        Svg::new(Handle::from_memory(FILE))
    }

    pub const FOLDER: &[u8] = include_bytes!("folder.svg");

    pub fn folder<'a>() -> Svg<'a> {
        Svg::new(Handle::from_memory(FOLDER))
    }
}
