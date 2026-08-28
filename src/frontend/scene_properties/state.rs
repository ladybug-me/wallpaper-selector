use crate::domain::scene_properties::{SceneProperty, ScenePropertyKind, ScenePropertyValue};

#[derive(Clone, Debug, PartialEq)]
pub enum ScenePropMsg {
    Toggle(String),
    Slide(String, f64),
    Commit(String),
    Choose(String, f64),
    ColourInput(String, String),
    ColourCommit(String),
    Reset,
    Close,
}

pub struct SceneProperties {
    pub we_id: String,
    pub title: String,
    pub rows: Vec<SceneProperty>,
    pub loading: bool,
    pub error: Option<String>,
    pub colour_drafts: std::collections::BTreeMap<String, String>,
}

impl SceneProperties {
    #[must_use]
    pub fn opening(we_id: &str, title: &str) -> Self {
        Self {
            we_id: we_id.to_string(),
            title: title.to_string(),
            rows: Vec::new(),
            loading: true,
            error: None,
            colour_drafts: std::collections::BTreeMap::new(),
        }
    }

    pub fn accept(&mut self, we_id: &str, rows: Vec<SceneProperty>) {
        if we_id != self.we_id {
            return;
        }
        self.colour_drafts.clear();
        for property in rows.iter().filter(|row| row.kind == ScenePropertyKind::Colour) {
            self.colour_drafts.insert(property.name.clone(), property.value.display());
        }
        self.rows = rows;
        self.loading = false;
        self.error = None;
    }

    pub fn fail(&mut self, error: &str) {
        self.loading = false;
        self.error = Some(error.to_string());
    }

    #[must_use]
    pub fn row(&self, name: &str) -> Option<&SceneProperty> {
        self.rows.iter().find(|row| row.name == name)
    }

    #[must_use]
    pub fn editable_count(&self) -> usize {
        self.rows.iter().filter(|row| row.editable()).count()
    }

    #[must_use]
    pub fn overridden_count(&self) -> usize {
        self.rows.iter().filter(|row| row.overridden).count()
    }

    #[must_use]
    pub fn colour_draft(&self, name: &str) -> Option<&str> {
        self.colour_drafts.get(name).map(String::as_str)
    }

    pub fn set_colour_draft(&mut self, name: &str, text: &str) {
        self.colour_drafts.insert(name.to_string(), text.to_string());
    }

    pub fn set_local(&mut self, name: &str, value: ScenePropertyValue) {
        if let Some(row) = self.rows.iter_mut().find(|row| row.name == name) {
            row.overridden = value != row.default;
            row.value = value;
        }
    }
}
