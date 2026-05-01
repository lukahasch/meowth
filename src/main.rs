use std::sync::Arc;

use eframe::{App, CreationContext};
use egui::Stroke;
use egui_tiles::{Behavior, Tree};
use serde::{Deserialize, Serialize};
use undo::History;

use crate::logic::Logic;
use crate::utils::macchiato_visuals;
use crate::world::World;

pub mod logic;
pub mod panes;
pub mod utils;
pub mod world;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meowth {
    pub project_name: String,
    pub world: World,
    pub logic: Logic,
    pub tree: Tree<Pane>,
    pub history: History<Command>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Pane {
    New(panes::new::New),
    Fixtures(panes::fixtures::Fixtures),
    SceneTree,
    Settings,
    Canvas,
    Outputs,
    Visualisation,
    Cues,
    UndoTree,
}

#[derive(Debug)]
pub struct Context<'a> {
    pub world: &'a mut World,
    pub logic: &'a mut Logic,
    pub history: &'a mut History<Command>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Command {}

impl<'a> Behavior<Pane> for Context<'a> {
    fn pane_ui(
        &mut self,
        ui: &mut egui::Ui,
        tile_id: egui_tiles::TileId,
        pane: &mut Pane,
    ) -> egui_tiles::UiResponse {
        match pane {
            Pane::New(new) => new.pane_ui(ui, tile_id, self),
            Pane::Fixtures(fixtures) => fixtures.pane_ui(ui, tile_id, self),
            _ => todo!(),
        }
    }

    fn tab_title_for_pane(&mut self, pane: &Pane) -> egui::WidgetText {
        match pane {
            Pane::New(new) => new.tab_tile_for_pane(self),
            Pane::Fixtures(fixtures) => fixtures.tab_tile_for_pane(self),
            _ => todo!(),
        }
    }

    fn is_tab_closable(
        &self,
        _tiles: &egui_tiles::Tiles<Pane>,
        _tile_id: egui_tiles::TileId,
    ) -> bool {
        true
    }

    fn resize_stroke(&self, style: &egui::Style, _: egui_tiles::ResizeState) -> egui::Stroke {
        Stroke::new(1.0_f32, style.visuals.faint_bg_color)
    }
}

impl App for Meowth {
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        subsecond::call(|| {
            self.tree.ui(
                &mut Context {
                    world: &mut self.world,
                    logic: &mut self.logic,
                    history: &mut self.history,
                },
                ui,
            );
        })
    }
}

impl Meowth {
    fn new(cc: &CreationContext) -> Self {
        cc.egui_ctx.set_visuals(macchiato_visuals());
        let c = cc.egui_ctx.clone();
        subsecond::register_handler(Arc::new(move || c.request_repaint()));
        Self {
            project_name: "Untitled".into(),
            world: World::new(),
            logic: Logic::new(),
            tree: Tree::new_grid("tree", vec![Pane::Fixtures(panes::fixtures::Fixtures)]),
            history: History::new(),
        }
    }
}

fn main() {
    dioxus_devtools::connect_subsecond();
    let native_options = eframe::NativeOptions::default();
    _ = eframe::run_native(
        "Meowth",
        native_options,
        Box::new(|cc| Ok(Box::new(Meowth::new(cc)))),
    );
}
