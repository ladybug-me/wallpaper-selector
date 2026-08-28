use crate::domain::effects::{EffectDefinition, EffectStep, EffectValue, EffectValues};

use super::model::{Effects, NAV_EFFECT};

pub(in crate::frontend::effects) struct EffectEditorState {
    pub(in crate::frontend::effects) definitions: Vec<EffectDefinition>,
    pub(in crate::frontend::effects) selected_id: String,
    pub(in crate::frontend::effects) values: EffectValues,
    pub(in crate::frontend::effects) saved: Vec<EffectStep>,
    pub(in crate::frontend::effects) audition_selected: bool,
    pub(in crate::frontend::effects) nav_focus: usize,
}

impl EffectEditorState {
    pub(super) fn new(definitions: Vec<EffectDefinition>) -> Self {
        let selected_id =
            definitions.first().map(|definition| definition.id.clone()).unwrap_or_default();
        Self {
            definitions,
            selected_id,
            values: EffectValues::new(),
            saved: Vec::new(),
            audition_selected: true,
            nav_focus: NAV_EFFECT,
        }
    }

    pub(super) fn selected(&self) -> Option<&EffectDefinition> {
        self.definitions.iter().find(|definition| definition.id == self.selected_id)
    }

    pub(super) fn category_of(&self, effect_id: &str) -> String {
        self.definitions
            .iter()
            .find(|definition| definition.id == effect_id)
            .map(|definition| definition.category.clone())
            .unwrap_or_default()
    }
}

impl Effects {
    pub fn selected(&self) -> Option<&EffectDefinition> {
        self.editor.selected()
    }

    pub fn categories(&self) -> Vec<String> {
        let mut seen = Vec::new();
        for definition in &self.editor.definitions {
            if !seen.iter().any(|category| category == &definition.category) {
                seen.push(definition.category.clone());
            }
        }
        seen
    }

    pub fn reset_params(&mut self) {
        if let Some(saved) =
            self.editor.saved.iter().find(|saved| saved.effect == self.editor.selected_id)
        {
            self.editor.values.clone_from(&saved.params);
            return;
        }
        self.editor.values.clear();
        let defaults: Vec<(String, EffectValue)> = self
            .selected()
            .map(|definition| {
                definition
                    .params
                    .iter()
                    .filter_map(|param| Some((param.id.clone(), param.default.as_ref()?.clone())))
                    .collect()
            })
            .unwrap_or_default();
        self.editor.values.extend(defaults);
    }

    #[cfg(test)]
    pub fn parameter_values(&self) -> &EffectValues {
        &self.editor.values
    }

    pub fn set_num(&mut self, id: &str, value: f64) {
        self.editor.audition_selected = true;
        self.editor.values.insert(id.to_string(), EffectValue::Number(value));
        self.sync_saved_params();
    }

    pub fn set_str(&mut self, id: &str, value: String) {
        self.editor.audition_selected = true;
        self.editor.values.insert(id.to_string(), EffectValue::Text(value));
        self.sync_saved_params();
    }

    pub fn toggle_saved(&mut self) {
        if let Some(index) =
            self.editor.saved.iter().position(|saved| saved.effect == self.editor.selected_id)
        {
            self.editor.saved.remove(index);
            self.editor.audition_selected = false;
        } else if !self.editor.selected_id.is_empty() {
            self.editor.saved.push(EffectStep {
                effect: self.editor.selected_id.clone(),
                params: self.editor.values.clone(),
            });
            self.editor.audition_selected = true;
        }
    }

    pub fn is_saved(&self, effect_id: &str) -> bool {
        self.editor.saved.iter().any(|saved| saved.effect == effect_id)
    }

    pub fn selected_is_saved(&self) -> bool {
        self.is_saved(&self.editor.selected_id)
    }

    pub fn has_saved_effects(&self) -> bool {
        !self.editor.saved.is_empty()
    }

    pub fn saved_effects(&self) -> &[EffectStep] {
        &self.editor.saved
    }

    pub fn preview_effects(&self) -> Vec<EffectStep> {
        let mut effects = self.editor.saved.clone();
        if !self.selected_is_saved()
            && self.editor.audition_selected
            && !self.editor.selected_id.is_empty()
        {
            effects.push(EffectStep {
                effect: self.editor.selected_id.clone(),
                params: self.editor.values.clone(),
            });
        }
        effects
    }

    fn sync_saved_params(&mut self) {
        if let Some(saved) =
            self.editor.saved.iter_mut().find(|saved| saved.effect == self.editor.selected_id)
        {
            saved.params.clone_from(&self.editor.values);
        }
    }

    pub fn select_effect(&mut self, id: String) {
        if self.editor.selected_id != id {
            self.panel.animation.restart("params", 0.0);
            if self.editor.category_of(&self.editor.selected_id) != self.editor.category_of(&id) {
                self.panel.animation.restart("list", 0.0);
            }
        }
        self.editor.selected_id = id;
        self.editor.audition_selected = true;
        self.reset_params();
        let stops = self.nav_stops();
        if stops > 0 {
            self.editor.nav_focus = self.editor.nav_focus.min(stops - 1);
        }
    }

    pub(in crate::frontend::effects) fn first_effect_of(&self, category: &str) -> String {
        self.editor
            .definitions
            .iter()
            .find(|definition| definition.category == category)
            .map(|definition| definition.id.clone())
            .unwrap_or_default()
    }
}
