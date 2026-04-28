use iced::{Element, Task};
use serde::{Deserialize, Serialize};

use crate::{
    scenes::scene::view::View,
    state::State,
    util::{
        output::Output,
        storage::{Id, Storage},
    },
};

pub mod view;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Scene {
    pub name: String,
    pub children: Vec<Id<Scene>>,
    pub view: View,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Message {
    ViewMessage(view::Message),
}

impl Scene {
    #[allow(clippy::new_without_default)]
    pub fn new(name: String) -> Self {
        Scene {
            name,
            children: Vec::new(),
            view: View::new(),
        }
    }

    pub fn top(scenes: &mut Storage<Scene>) -> Self {
        let layered = Scene {
            name: String::from("Folder 1"),
            children: vec![
                scenes.insert(Scene::new(String::from("Scene 2"))),
                scenes.insert(Scene::new(String::from("Scene 3"))),
            ],
            view: View::new(),
        };
        let mut children = vec![
            scenes.insert(Scene::new(String::from("Scene 1"))),
            scenes.insert(layered),
        ];
        for i in 4..=20 {
            children.push(scenes.insert(Scene::new(format!("Scene {}", i))));
        }
        Scene {
            name: String::from("Top"),
            children,
            view: View::new(),
        }
    }

    pub fn update<T: From<(Id<Self>, Message)> + 'static + Send>(
        &mut self,
        message: Message,
        state: &State,
    ) -> Output<T> {
        Output::none()
    }

    pub fn view<'a>(&'a self, state: &'a State) -> Element<'a, Message> {
        self.view.view(state).map(Message::ViewMessage)
    }

    pub fn is_folder(&self) -> bool {
        !self.children.is_empty()
    }
}
