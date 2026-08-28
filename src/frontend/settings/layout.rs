const PICKER_LAYOUT_STUDIO_WIDTH: f32 = 440.0;

pub fn picker_layout_studio_width(viewport_width: f32, scale: f32) -> f32 {
    let available = (viewport_width - 36.0 * scale).max(320.0);
    (PICKER_LAYOUT_STUDIO_WIDTH * scale.max(0.8))
        .min(available)
        .min((viewport_width * 0.48).max(320.0))
}
