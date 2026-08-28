use log::info;
use serde_json::json;

#[allow(clippy::wildcard_imports)]
use crate::app::*;

impl App {
    pub(super) fn on_effects_preview(
        &mut self,
        result: crate::contracts::daemon::EffectOperationResult,
        source: &str,
    ) {
        let output = result.output;
        if self.panels.effects.as_ref().is_none_or(|effects| effects.source_path() != source) {
            if !output.is_empty() {
                self.daemon.client.call("effects.discard", json!({ "preview": &output }));
            }
            return;
        }
        let (dirty, stale) = if let Some(eff) = self.panels.effects.as_mut() {
            eff.finish_preview_request(&output)
        } else {
            (false, None)
        };
        if let Some(path) = stale {
            self.daemon.client.call("effects.discard", json!({ "preview": path }));
        }
        if dirty {
            effects_do_preview(self);
        }
    }

    pub(super) fn on_effects_commit(
        &mut self,
        result: crate::contracts::daemon::EffectOperationResult,
        apply: bool,
    ) {
        let output = result.output;
        let stale = if let Some(eff) = self.panels.effects.as_mut() {
            eff.finish_commit_request(&output)
        } else {
            None
        };
        if let Some(path) = stale {
            self.daemon.client.call("effects.discard", json!({ "preview": path }));
        }
        if apply && !output.is_empty() {
            self.daemon.client.call(
                "wall.apply",
                json!({ "type": wall_proto::kind::STATIC, "path": output, "no_transition": true }),
            );
        }
    }

    pub(super) fn on_effect_themes(&mut self, result: crate::contracts::daemon::EffectsListResult) {
        if let Some(definitions) = result.definitions {
            self.daemon.effect_definitions = definitions;
        }
        if let Some(options) = result.theme_options {
            self.daemon.effect_themes = options;
        }
        info!("effect themes loaded: {}", self.daemon.effect_themes.len());
    }
}
