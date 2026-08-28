use crate::frontend::browser::{Browser, PEXELS_COLORS, PEXELS_SIZES, PHOTO_ORIENTATIONS};
use crate::i18n::tr;

use super::super::super::bar::BarItem;
use super::item::{photo_orientation_label_key, push_item};
use super::types::BrowserAct;

fn size_label(key: &str, label: &'static str) -> &'static str {
    if key.is_empty() { tr("browser-any-size") } else { label }
}

fn color_label_key(key: &str) -> &'static str {
    match key {
        "red" => "browser-red",
        "orange" => "browser-orange",
        "yellow" => "browser-yellow",
        "green" => "browser-green",
        "turquoise" => "browser-turquoise",
        "blue" => "browser-blue",
        "violet" => "browser-violet",
        "pink" => "browser-pink",
        "brown" => "browser-brown",
        "black" => "browser-black",
        "gray" => "browser-gray",
        "white" => "browser-white",
        "#808080" => "browser-neutral",
        _ => "browser-any-colour",
    }
}

pub(super) fn build_rows(
    browser: &Browser,
    scale: f32,
    mut x: f32,
    output: &mut Vec<(BarItem, BrowserAct)>,
) {
    for (key, _) in PHOTO_ORIENTATIONS {
        push_item(
            output,
            scale,
            &mut x,
            0.0,
            tr(photo_orientation_label_key(key)),
            false,
            10.0 * scale,
            browser.request.catalog.pexels.orientation == key,
            BrowserAct::CatalogFilter("pexels-orientation", key.to_string()),
        );
    }

    let row_height = 30.0 * scale;
    let mut size_x = 0.0;
    for (key, label) in PEXELS_SIZES {
        push_item(
            output,
            scale,
            &mut size_x,
            row_height,
            size_label(key, label),
            false,
            10.0 * scale,
            browser.request.catalog.pexels.size == key,
            BrowserAct::CatalogFilter("pexels-size", key.to_string()),
        );
    }

    let mut color_x = 0.0;
    for (key, _) in PEXELS_COLORS {
        push_item(
            output,
            scale,
            &mut color_x,
            2.0 * row_height,
            tr(color_label_key(key)),
            false,
            9.0 * scale,
            browser.request.catalog.pexels.color == key,
            BrowserAct::CatalogFilter("pexels-color", key.to_string()),
        );
    }
}
