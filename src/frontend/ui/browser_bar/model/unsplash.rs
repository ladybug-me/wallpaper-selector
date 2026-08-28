use crate::frontend::browser::{
    Browser, PHOTO_ORIENTATIONS, UNSPLASH_COLORS, UNSPLASH_ORDERS, UNSPLASH_SAFETY,
};
use crate::i18n::tr;

use super::super::super::bar::BarItem;
use super::item::{photo_orientation_label_key, push_item};
use super::types::BrowserAct;

fn order_label_key(key: &str) -> &'static str {
    match key {
        "latest" => "browser-latest",
        _ => "browser-relevant",
    }
}

fn safety_label_key(key: &str) -> &'static str {
    match key {
        "high" => "browser-strict",
        _ => "browser-standard",
    }
}

fn color_label_key(key: &str) -> &'static str {
    match key {
        "black_and_white" => "browser-bw",
        "black" => "browser-black",
        "white" => "browser-white",
        "yellow" => "browser-yellow",
        "orange" => "browser-orange",
        "red" => "browser-red",
        "purple" => "browser-purple",
        "magenta" => "browser-magenta",
        "green" => "browser-green",
        "teal" => "browser-teal",
        "blue" => "browser-blue",
        _ => "browser-any-colour",
    }
}

pub(super) fn build_rows(
    browser: &Browser,
    scale: f32,
    mut x: f32,
    output: &mut Vec<(BarItem, BrowserAct)>,
) {
    for (key, _) in UNSPLASH_ORDERS {
        push_item(
            output,
            scale,
            &mut x,
            0.0,
            tr(order_label_key(key)),
            false,
            10.0 * scale,
            browser.request.catalog.unsplash.order_by == key,
            BrowserAct::CatalogFilter("unsplash-order", key.to_string()),
        );
    }
    x += 12.0 * scale;
    for (key, _) in UNSPLASH_SAFETY {
        push_item(
            output,
            scale,
            &mut x,
            0.0,
            tr(safety_label_key(key)),
            false,
            10.0 * scale,
            browser.request.catalog.unsplash.content_filter == key,
            BrowserAct::CatalogFilter("unsplash-safety", key.to_string()),
        );
    }

    let row_height = 30.0 * scale;
    let mut orientation_x = 0.0;
    for (key, _) in PHOTO_ORIENTATIONS {
        let api_key = if key == "square" { "squarish" } else { key };
        push_item(
            output,
            scale,
            &mut orientation_x,
            row_height,
            tr(photo_orientation_label_key(key)),
            false,
            10.0 * scale,
            browser.request.catalog.unsplash.orientation == api_key,
            BrowserAct::CatalogFilter("unsplash-orientation", api_key.to_string()),
        );
    }

    let mut color_x = 0.0;
    for (key, _) in UNSPLASH_COLORS {
        push_item(
            output,
            scale,
            &mut color_x,
            2.0 * row_height,
            tr(color_label_key(key)),
            false,
            9.0 * scale,
            browser.request.catalog.unsplash.color == key,
            BrowserAct::CatalogFilter("unsplash-color", key.to_string()),
        );
    }
}
