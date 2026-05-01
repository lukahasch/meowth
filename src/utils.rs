use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

use serde::{Deserialize, Serialize};

pub struct Id<T> {
    id: u64,
    _marker: std::marker::PhantomData<T>,
}

impl<T> std::fmt::Debug for Id<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Id")
            .field("id", &self.id)
            .field("_marker", &self._marker)
            .finish()
    }
}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Id<T> {}

impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T> Eq for Id<T> {}

impl<T> std::hash::Hash for Id<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<T> Serialize for Id<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.id.serialize(serializer)
    }
}

impl<'de, T> Deserialize<'de> for Id<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let id = u64::deserialize(deserializer)?;
        Ok(Self {
            id,
            _marker: std::marker::PhantomData,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Map<V> {
    inner: HashMap<Id<V>, V>,
    cid: u64,
}

impl<V> Map<V> {
    pub fn new() -> Self {
        Self {
            cid: 0,
            inner: HashMap::new(),
        }
    }

    pub fn create(&mut self, v: V) -> Id<V> {
        self.cid += 1;
        let id = Id {
            id: self.cid,
            _marker: std::marker::PhantomData,
        };
        self.inner.insert(id, v);
        id
    }
}

impl<V> Default for Map<V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<V> Deref for Map<V> {
    type Target = HashMap<Id<V>, V>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<V> DerefMut for Map<V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

use egui::{Color32, Stroke, Visuals, style::WidgetVisuals};

pub fn mocha_visuals() -> Visuals {
    let base = Color32::from_rgb(30, 30, 46);
    let surface0 = Color32::from_rgb(49, 50, 68);
    let surface1 = Color32::from_rgb(69, 71, 90);
    let surface2 = Color32::from_rgb(88, 91, 112);
    let overlay1 = Color32::from_rgb(127, 132, 156);
    let text = Color32::from_rgb(205, 214, 244);
    let lavender = Color32::from_rgb(180, 190, 254);
    let mauve = Color32::from_rgb(203, 166, 247);

    let widget_inactive = WidgetVisuals {
        bg_fill: surface0,
        weak_bg_fill: surface0,
        bg_stroke: Stroke::new(1.0, surface2),
        fg_stroke: Stroke::new(1.0, text),
        corner_radius: 4.0.into(),
        expansion: 0.0,
    };
    let widget_hovered = WidgetVisuals {
        bg_fill: surface1,
        weak_bg_fill: surface1,
        bg_stroke: Stroke::new(1.0, lavender),
        fg_stroke: Stroke::new(1.5, text),
        corner_radius: 4.0.into(),
        expansion: 1.0,
    };
    let widget_active = WidgetVisuals {
        bg_fill: surface2,
        weak_bg_fill: surface2,
        bg_stroke: Stroke::new(1.0, mauve),
        fg_stroke: Stroke::new(2.0, text),
        corner_radius: 4.0.into(),
        expansion: 1.0,
    };

    let mut v = Visuals::dark();
    v.panel_fill = base;
    v.window_fill = base;
    v.faint_bg_color = surface0;
    v.extreme_bg_color = Color32::from_rgb(24, 24, 37); // crust
    v.code_bg_color = surface0;
    v.selection.bg_fill = mauve.linear_multiply(0.4);
    v.selection.stroke = Stroke::new(1.0, mauve);
    v.hyperlink_color = lavender;
    v.override_text_color = Some(text);
    v.widgets.noninteractive = WidgetVisuals {
        bg_fill: base,
        weak_bg_fill: base,
        bg_stroke: Stroke::new(1.0, surface1),
        fg_stroke: Stroke::new(1.0, overlay1),
        corner_radius: 4.0.into(),
        expansion: 0.0,
    };
    v.widgets.inactive = widget_inactive;
    v.widgets.hovered = widget_hovered;
    v.widgets.active = widget_active;
    v.widgets.open = widget_hovered; // for dropdowns
    v
}

pub fn macchiato_visuals() -> Visuals {
    let base = Color32::from_rgb(36, 39, 58);
    let surface0 = Color32::from_rgb(54, 58, 79);
    let surface1 = Color32::from_rgb(73, 77, 100);
    let surface2 = Color32::from_rgb(91, 96, 120);
    let overlay1 = Color32::from_rgb(128, 135, 162);
    let text = Color32::from_rgb(202, 211, 245);
    let lavender = Color32::from_rgb(183, 189, 248);
    let mauve = Color32::from_rgb(198, 160, 246);

    let widget_inactive = WidgetVisuals {
        bg_fill: surface0,
        weak_bg_fill: surface0,
        bg_stroke: Stroke::new(1.0, surface2),
        fg_stroke: Stroke::new(1.0, text),
        corner_radius: 4.0.into(),
        expansion: 0.0,
    };
    let widget_hovered = WidgetVisuals {
        bg_fill: surface1,
        weak_bg_fill: surface1,
        bg_stroke: Stroke::new(1.0, lavender),
        fg_stroke: Stroke::new(1.5, text),
        corner_radius: 4.0.into(),
        expansion: 1.0,
    };
    let widget_active = WidgetVisuals {
        bg_fill: surface2,
        weak_bg_fill: surface2,
        bg_stroke: Stroke::new(1.0, mauve),
        fg_stroke: Stroke::new(2.0, text),
        corner_radius: 4.0.into(),
        expansion: 1.0,
    };

    let mut v = Visuals::dark();
    v.panel_fill = base;
    v.window_fill = base;
    v.faint_bg_color = surface0;
    v.extreme_bg_color = Color32::from_rgb(24, 25, 38); // crust
    v.code_bg_color = surface0;
    v.selection.bg_fill = mauve.linear_multiply(0.4);
    v.selection.stroke = Stroke::new(1.0, mauve);
    v.hyperlink_color = lavender;
    v.override_text_color = Some(text);
    v.widgets.noninteractive = WidgetVisuals {
        bg_fill: base,
        weak_bg_fill: base,
        bg_stroke: Stroke::new(1.0, surface1),
        fg_stroke: Stroke::new(1.0, overlay1),
        corner_radius: 4.0.into(),
        expansion: 0.0,
    };
    v.widgets.inactive = widget_inactive;
    v.widgets.hovered = widget_hovered;
    v.widgets.active = widget_active;
    v.widgets.open = widget_hovered;
    v
}
