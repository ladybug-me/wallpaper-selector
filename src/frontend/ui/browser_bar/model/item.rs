use super::super::super::bar::BarItem;
use super::super::super::misc::text_width;
use super::types::BrowserAct;

pub(super) fn photo_orientation_label_key(key: &str) -> &'static str {
    match key {
        "landscape" => "browser-orient-landscape",
        "portrait" => "browser-orient-portrait",
        "square" => "browser-square",
        _ => "browser-any",
    }
}

pub(super) fn push_item(
    output: &mut Vec<(BarItem, BrowserAct)>,
    scale: f32,
    x: &mut f32,
    y: f32,
    label: &str,
    nerd_font: bool,
    size: f32,
    active: bool,
    action: BrowserAct,
) {
    let height = 24.0 * scale;
    let skew = 8.0 * scale;
    let width = text_width(label, size, nerd_font) + 24.0 * scale + skew;
    output.push((
        BarItem {
            x: *x,
            y,
            w: width,
            h: height,
            skew,
            label: label.to_string(),
            nerd: nerd_font,
            text_size: size,
            swatch: None,
            notice: None,
            active,
            action: None,
            z: if active { 10 } else { 1 },
        },
        action,
    ));
    *x += width - 6.0 * scale;
}
