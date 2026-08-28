use serde_json::Value;

use crate::domain::library::filter::Filters;
use crate::frontend::theme::Palette;

pub(crate) struct DemoSession {
    pub(crate) config: Value,
    pub(crate) filters: Filters,
    pub(crate) query: String,
    pub(crate) selection_key: String,
    pub(crate) filter_bar_visible: bool,
    pub(crate) palette: Palette,
    pub(crate) base_palette: Palette,
    pub(crate) motion_scale: f32,
    pub(crate) type_badges_suppressed: bool,
    pub(crate) opening_blur: Option<std::path::PathBuf>,
    pub(crate) opening_blur_source: Option<String>,
    pub(crate) opening_blur_resolved: bool,
    pub(crate) apply_source: Option<String>,
    pub(crate) batch_id: Option<String>,
    pub(crate) batch_commands: Vec<String>,
}
