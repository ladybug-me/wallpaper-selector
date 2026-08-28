use iced::Color;

use crate::contracts::picker::PaletteSpec;
use crate::domain::theme::{Candidate, ThemeRole};

#[derive(Debug, Clone, Copy)]
pub struct Palette {
    pub primary: Color,
    pub primary_text: Color,
    pub surface: Color,
    pub surface_text: Color,
    pub surface_variant: Color,
    pub surface_container: Color,
    pub background: Color,
    pub outline: Color,
    pub tertiary: Color,
}

pub fn parse_hex(raw: &str) -> Option<Color> {
    hex(raw)
}

fn hex(raw: &str) -> Option<Color> {
    let digits = raw.trim().trim_start_matches('#');
    let parse = |idx: usize| u8::from_str_radix(digits.get(idx..idx + 2)?, 16).ok();
    match digits.len() {
        6 => Some(Color::from_rgb8(parse(0)?, parse(2)?, parse(4)?)),
        8 => Some(Color::from_rgba8(
            parse(2)?,
            parse(4)?,
            parse(6)?,
            u8::from_str_radix(&digits[0..2], 16).ok()? as f32 / 255.0,
        )),
        _ => None,
    }
}

impl Default for Palette {
    fn default() -> Self {
        Self {
            primary: hex("#ffb4ab").unwrap(),
            primary_text: hex("#1d100e").unwrap(),
            surface: hex("#1d100e").unwrap(),
            surface_text: hex("#f1dedb").unwrap(),
            surface_variant: hex("#534341").unwrap(),
            surface_container: hex("#271815").unwrap(),
            background: hex("#1d100e").unwrap(),
            outline: hex("#a08c89").unwrap(),
            tertiary: hex("#dfc38c").unwrap(),
        }
    }
}

use crate::frontend::components::lerp_color;

impl Palette {
    pub fn destructive(&self) -> Color {
        self.tertiary
    }

    pub fn lerp(&self, other: &Palette, t: f32) -> Palette {
        let t = t.clamp(0.0, 1.0);
        Palette {
            primary: lerp_color(self.primary, other.primary, t),
            primary_text: lerp_color(self.primary_text, other.primary_text, t),
            surface: lerp_color(self.surface, other.surface, t),
            surface_text: lerp_color(self.surface_text, other.surface_text, t),
            surface_variant: lerp_color(self.surface_variant, other.surface_variant, t),
            surface_container: lerp_color(self.surface_container, other.surface_container, t),
            background: lerp_color(self.background, other.background, t),
            outline: lerp_color(self.outline, other.outline, t),
            tertiary: lerp_color(self.tertiary, other.tertiary, t),
        }
    }

    pub fn from_spec(spec: &PaletteSpec) -> Self {
        let mut palette = Self::default();
        if let Some(col) = spec.primary.as_deref().and_then(hex) {
            palette.primary = col;
        }
        if let Some(col) = spec.primary_text.as_deref().and_then(hex) {
            palette.primary_text = col;
        }
        if let Some(col) = spec.surface.as_deref().and_then(hex) {
            palette.surface = col;
        }
        if let Some(col) = spec.surface_text.as_deref().and_then(hex) {
            palette.surface_text = col;
        }
        if let Some(col) = spec.surface_variant.as_deref().and_then(hex) {
            palette.surface_variant = col;
        }
        if let Some(col) = spec.surface_container.as_deref().and_then(hex) {
            palette.surface_container = col;
        }
        if let Some(col) = spec.background.as_deref().and_then(hex) {
            palette.background = col;
        }
        if let Some(col) = spec.outline.as_deref().and_then(hex) {
            palette.outline = col;
        }
        if let Some(col) = spec.tertiary.as_deref().and_then(hex) {
            palette.tertiary = col;
        }
        palette
    }

    pub fn from_candidate(candidate: &Candidate) -> Self {
        let mut palette = Self::default();
        let color = |role: ThemeRole| candidate.colors[role.index()].as_str();
        if let Some(value) = hex(color(ThemeRole::Primary)) {
            palette.primary = value;
        }
        if let Some(value) = hex(color(ThemeRole::PrimaryText)) {
            palette.primary_text = value;
        }
        if let Some(value) = hex(color(ThemeRole::Surface)) {
            palette.surface = value;
        }
        if let Some(value) = hex(color(ThemeRole::SurfaceText)) {
            palette.surface_text = value;
        }
        if let Some(value) = hex(color(ThemeRole::SurfaceVariant)) {
            palette.surface_variant = value;
        }
        if let Some(value) = hex(color(ThemeRole::SurfaceContainer)) {
            palette.surface_container = value;
        }
        if let Some(value) = hex(color(ThemeRole::Background)) {
            palette.background = value;
        }
        if let Some(value) = hex(color(ThemeRole::Outline)) {
            palette.outline = value;
        }
        if let Some(value) = hex(color(ThemeRole::Tertiary)) {
            palette.tertiary = value;
        }
        palette
    }
}
