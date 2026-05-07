use egui::{Separator, Vec2};
use egui_flex::{Flex, FlexAlign, FlexJustify, item};
use egui_tiles::UiResponse;
use serde::{Deserialize, Serialize};

use crate::Context;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fixtures;

impl Fixtures {
    pub fn tab_tile_for_pane(&self, _: &mut Context) -> egui::WidgetText {
        "Fixtures".into()
    }

    pub fn pane_ui(
        &mut self,
        ui: &mut egui::Ui,
        tile_id: egui_tiles::TileId,
        ctx: &mut Context,
    ) -> egui_tiles::UiResponse {
        let extreme_bg = ui.visuals().extreme_bg_color;
        let mut drag = false;

        egui::Frame::new().fill(extreme_bg).show(ui, |ui| {
            ui.label("Fixtures");
            Flex::vertical()
                .h_full()
                .w_full()
                .align_items(FlexAlign::Center)
                .justify(FlexJustify::Center)
                .gap(Vec2::new(0.0, 10.0))
                .show(ui, |flex| {
                    if flex.add(item(), egui::Button::new("Fixtures")).clicked() {
                        drag = true;
                    }
                });
        });
        if drag {
            UiResponse::DragStarted
        } else {
            UiResponse::None
        }
    }
}
