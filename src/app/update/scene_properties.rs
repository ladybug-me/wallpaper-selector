use serde_json::json;

use crate::app::overlay::Overlay;
#[allow(clippy::wildcard_imports)]
use crate::app::*;
use crate::domain::scene_properties::{ScenePropertyValue, parse_vector};
use crate::frontend::scene_properties::{ScenePropMsg, SceneProperties};

impl App {
    pub(in crate::app) fn open_scene_properties(&mut self) {
        let Some((we_id, title)) = self.focused_scene() else {
            return;
        };
        self.panels.scene_properties = Some(SceneProperties::opening(&we_id, &title));
        self.request_scene_properties(&we_id);
        self.retick();
    }

    fn focused_scene(&self) -> Option<(String, String)> {
        let index = self.scene.flipped().unwrap_or(self.scene.current);
        let selected = *self.library_session.filtered.get(index)?;
        let item = self.library_session.library.catalog().items.get(selected as usize)?;
        if item.effective_kind() != crate::domain::library::catalog::WallpaperKind::We
            || item.we_id.is_empty()
        {
            return None;
        }
        Some((item.we_id.clone(), item.name.clone()))
    }

    fn request_scene_properties(&mut self, we_id: &str) {
        self.call_tracked(
            wall_proto::rpc::WALL_WE_PROPERTIES,
            json!({ "we_id": we_id }),
            Pending::SceneProperties { we_id: we_id.to_string() },
        );
    }

    fn write_scene_property(&mut self, name: &str, value: ScenePropertyValue) {
        let Some(panel) = self.panels.scene_properties.as_mut() else {
            return;
        };
        let we_id = panel.we_id.clone();
        let encoded = crate::infrastructure::scene_properties::encode(&value);
        panel.set_local(name, value);
        self.call_tracked(
            wall_proto::rpc::WALL_SET_WE_PROPERTY,
            json!({ "we_id": we_id, "name": name, "value": encoded }),
            Pending::SceneProperties { we_id },
        );
        self.retick();
    }

    pub(in crate::app) fn on_scene_properties(
        &mut self,
        result: crate::contracts::daemon::ScenePropertiesResult,
    ) {
        if let Some(panel) = self.panels.scene_properties.as_mut() {
            panel.accept(&result.we_id, result.rows);
        }
        self.retick();
    }

    pub(in crate::app) fn on_scene_properties_error(&mut self, error: &str) {
        if let Some(panel) = self.panels.scene_properties.as_mut() {
            panel.fail(error);
        }
        self.retick();
    }
}

pub(super) fn update(app: &mut App, message: ScenePropMsg) {
    match message {
        ScenePropMsg::Close => app.close_overlay(Overlay::SceneProperties),
        ScenePropMsg::Toggle(name) => {
            let Some(current) = app
                .panels
                .scene_properties
                .as_ref()
                .and_then(|panel| panel.row(&name).map(|row| row.value.flag()))
            else {
                return;
            };
            app.write_scene_property(&name, ScenePropertyValue::Flag(!current));
        }
        ScenePropMsg::Choose(name, value) => {
            app.write_scene_property(&name, ScenePropertyValue::Number(value));
        }
        ScenePropMsg::Slide(name, value) => {
            if let Some(panel) = app.panels.scene_properties.as_mut() {
                panel.set_local(&name, ScenePropertyValue::Number(value));
            }
            app.retick();
        }
        ScenePropMsg::Commit(name) => {
            let Some(value) = app
                .panels
                .scene_properties
                .as_ref()
                .and_then(|panel| panel.row(&name).map(|row| row.value.number()))
            else {
                return;
            };
            app.write_scene_property(&name, ScenePropertyValue::Number(value));
        }
        ScenePropMsg::ColourInput(name, text) => {
            if let Some(panel) = app.panels.scene_properties.as_mut() {
                panel.set_colour_draft(&name, &text);
            }
            app.retick();
        }
        ScenePropMsg::ColourCommit(name) => {
            let Some(text) = app
                .panels
                .scene_properties
                .as_ref()
                .and_then(|panel| panel.colour_draft(&name).map(str::to_string))
            else {
                return;
            };
            let Some(parts) = parse_vector(&text).filter(|parts| parts.len() == 3) else {
                return;
            };
            app.write_scene_property(&name, ScenePropertyValue::Vector(parts));
        }
        ScenePropMsg::Reset => {
            let Some(we_id) = app.panels.scene_properties.as_ref().map(|panel| panel.we_id.clone())
            else {
                return;
            };
            app.call_tracked(
                wall_proto::rpc::WALL_SET_WE_PROPERTY,
                json!({ "we_id": we_id, "reset": true }),
                Pending::SceneProperties { we_id },
            );
            app.retick();
        }
    }
}
