use std::sync::Arc;

use crate::domain::effects::{ShaderSpec, shader_spec};
use crate::domain::library::catalog::WallpaperKind;

use super::model::Effects;

pub(in crate::frontend::effects) struct PreviewSourceState {
    pub(in crate::frontend::effects) path: String,
    pub(in crate::frontend::effects) thumb: Option<String>,
    pub(in crate::frontend::effects) rendered: Option<String>,
    pub(in crate::frontend::effects) fade_from: Option<String>,
    pub(in crate::frontend::effects) kind: WallpaperKind,
    pub(in crate::frontend::effects) mute: bool,
    pub(in crate::frontend::effects) volume: u32,
    pub(in crate::frontend::effects) rgba: Option<Arc<Vec<u8>>>,
    pub(in crate::frontend::effects) width: u32,
    pub(in crate::frontend::effects) height: u32,
    pub(in crate::frontend::effects) version: u64,
}

impl PreviewSourceState {
    pub(super) fn new(
        path: String,
        thumb: Option<String>,
        kind: WallpaperKind,
        mute: bool,
        volume: u32,
    ) -> Self {
        Self {
            path,
            thumb,
            rendered: None,
            fade_from: None,
            kind,
            mute,
            volume,
            rgba: None,
            width: 0,
            height: 0,
            version: 0,
        }
    }

    pub(super) fn is_audible(&self) -> bool {
        self.kind == WallpaperKind::Video || self.kind == WallpaperKind::We
    }
}

impl Effects {
    pub fn set_source_texture(&mut self, rgba: Vec<u8>, width: u32, height: u32) {
        self.set_source_texture_arc(Arc::new(rgba), width, height);
    }

    pub fn set_source_texture_arc(&mut self, rgba: Arc<Vec<u8>>, width: u32, height: u32) {
        self.preview.rgba = Some(rgba);
        self.preview.width = width;
        self.preview.height = height;
        self.preview.version = next_source_version();
    }

    pub fn shader_effect(&self) -> Option<ShaderSpec> {
        if !self.has_effects_page() {
            return None;
        }
        let effects = self.preview_effects();
        if effects.len() != 1 {
            return None;
        }
        shader_spec(&effects[0].effect, &effects[0].params)
    }

    pub fn shader_live(&self) -> bool {
        self.preview.rgba.is_some() && self.shader_effect().is_some()
    }

    pub fn display_source(&self) -> String {
        self.preview.thumb.clone().unwrap_or_else(|| self.preview.path.clone())
    }

    pub fn is_protected(&self, path: &str) -> bool {
        path == self.preview.path || Some(path) == self.preview.thumb.as_deref()
    }

    pub fn begin_fade(&mut self, new_preview: String) -> Option<String> {
        let current = self.preview.rendered.clone().unwrap_or_else(|| self.display_source());
        let mut dropped = None;
        if !current.is_empty() && current != new_preview {
            dropped = self.preview.fade_from.replace(current);
            self.panel.animation.restart("fade", 0.0);
        }
        self.preview.rendered = Some(new_preview);
        dropped.filter(|path| !self.is_protected(path))
    }

    pub fn has_effects_page(&self) -> bool {
        self.preview.kind == WallpaperKind::Static
    }

    pub fn finish_preview_request(&mut self, output: &str) -> (bool, Option<String>) {
        self.panel.busy = false;
        let stale = if output.is_empty() {
            None
        } else {
            let old = self.begin_fade(output.to_string());
            self.panel.status.clear();
            old
        };
        (self.panel.preview_dirty, stale)
    }

    pub fn finish_commit_request(&mut self, output: &str) -> Option<String> {
        self.panel.busy = false;
        if !self.panel.committing {
            self.preview.rendered = None;
        }
        self.panel.status = if output.is_empty() {
            crate::i18n::tr("effects-save-failed").to_string()
        } else {
            crate::i18n::tr_args!("effects-saved-to", output => output)
        };
        self.preview.fade_from.take().filter(|path| !self.is_protected(path))
    }

    pub fn discardable_previews(self) -> Vec<String> {
        let protected = [self.preview.thumb.as_deref(), Some(self.preview.path.as_str())];
        self.preview
            .rendered
            .into_iter()
            .chain(self.preview.fade_from)
            .filter(|path| !protected.into_iter().flatten().any(|kept| kept == path))
            .collect()
    }
}

fn next_source_version() -> u64 {
    use std::sync::atomic::{AtomicU64, Ordering};
    static VERSION: AtomicU64 = AtomicU64::new(1);
    VERSION.fetch_add(1, Ordering::Relaxed)
}
