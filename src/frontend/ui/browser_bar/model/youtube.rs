use crate::frontend::browser::{Browser, DURATIONS};
use crate::i18n::tr;

use super::super::super::bar::BarItem;
use super::item::push_item;
use super::types::BrowserAct;

fn duration_label_key(key: &str) -> &'static str {
    match key {
        "300" => "browser-max-5m",
        "600" => "browser-max-10m",
        "1800" => "browser-max-30m",
        "3600" => "browser-max-1h",
        _ => "browser-any",
    }
}

pub(super) fn build_row(
    browser: &Browser,
    scale: f32,
    mut x: f32,
    output: &mut Vec<(BarItem, BrowserAct)>,
) {
    for (key, _) in DURATIONS {
        let active = if key.is_empty() {
            browser.request.catalog.max_duration.is_empty()
        } else {
            browser.request.catalog.max_duration == key
        };
        push_item(
            output,
            scale,
            &mut x,
            0.0,
            tr(duration_label_key(key)),
            false,
            10.0 * scale,
            active,
            BrowserAct::MaxDuration(key),
        );
    }
}
