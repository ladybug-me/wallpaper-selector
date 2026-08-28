use iced::widget::canvas::{Frame, Path, Stroke};
use iced::{Alignment, Color, Font, Point};

use crate::frontend::scene::Chrome;
use crate::frontend::theme::Palette;
use crate::i18n::tr;

use super::super::bar::ICON_FAV;
use super::super::misc::{NERD_FONT, UI_FONT, mid_text};
use super::super::{parallelogram, with_alpha};

fn badge_label(kind: u8) -> &'static str {
    match kind % 3 {
        1 => tr("filter-bar-type-vid"),
        2 => tr("filter-bar-type-we"),
        _ => tr("filter-bar-type-pic"),
    }
}

fn badge_text(
    frame: &mut Frame,
    content: &str,
    center_x: f32,
    center_y: f32,
    size: f32,
    color: Color,
) {
    frame.fill_text(mid_text(
        content.to_string(),
        Point::new(center_x, center_y),
        color,
        size,
        Font { weight: iced::font::Weight::Bold, ..UI_FONT },
        Alignment::Center,
    ));
}

fn video_indicator(
    frame: &mut Frame,
    palette: &Palette,
    center_x: f32,
    center_y: f32,
    diameter: f32,
    opacity: f32,
) {
    let circle = Path::circle(Point::new(center_x, center_y), diameter / 2.0);
    frame.fill(&circle, with_alpha(palette.surface, 0.92 * opacity));
    frame.stroke(
        &circle,
        Stroke::default().with_color(with_alpha(palette.primary, 0.6 * opacity)).with_width(1.0),
    );
    badge_text(
        frame,
        "\u{25b6}",
        center_x + 1.0,
        center_y,
        diameter * 0.41,
        with_alpha(palette.primary, opacity),
    );
}

fn type_badge(
    frame: &mut Frame,
    palette: &Palette,
    kind: u8,
    x: f32,
    y: f32,
    height: f32,
    skew: f32,
    text_size: f32,
    background_alpha: f32,
    opacity: f32,
) {
    let label = badge_label(kind);
    let width = label.chars().count() as f32 * text_size * 0.7 + height;
    let background = parallelogram(x, y, width, height, skew.min(height * 0.4));
    frame.fill(
        &background,
        with_alpha(palette.surface_container, background_alpha.max(0.82) * opacity),
    );
    frame.stroke(
        &background,
        Stroke::default().with_color(with_alpha(palette.outline, 0.58 * opacity)).with_width(1.0),
    );
    badge_text(
        frame,
        label,
        x + width / 2.0,
        y + height / 2.0,
        text_size,
        with_alpha(palette.surface_text, opacity),
    );
}

pub(super) fn draw_chrome_item(
    frame: &mut Frame,
    palette: &Palette,
    chrome: &Chrome,
    show_type_badge: bool,
) {
    match chrome.view {
        0 => draw_slice(frame, palette, chrome, show_type_badge),
        1 => draw_grid(frame, palette, chrome, show_type_badge),
        _ => draw_hex(frame, palette, chrome, show_type_badge),
    }
}

fn draw_slice(frame: &mut Frame, palette: &Palette, chrome: &Chrome, show_type_badge: bool) {
    if chrome.has_video {
        let x = if chrome.skew >= 0.0 {
            chrome.cx + chrome.hw - 21.0
        } else {
            chrome.cx - chrome.hw + 21.0
        };
        video_indicator(frame, palette, x, chrome.cy - chrome.hh + 21.0, 22.0, chrome.opacity);
    }
    if show_type_badge {
        let skew = chrome.skew.abs();
        let height = 16.0;
        let label = badge_label(chrome.kind);
        let width = label.chars().count() as f32 * 9.0 * 0.7 + height;
        let x = if chrome.skew >= 0.0 {
            chrome.cx + chrome.hw - width - skew - 8.0
        } else {
            chrome.cx - chrome.hw + skew + 8.0
        };
        type_badge(
            frame,
            palette,
            chrome.kind,
            x,
            chrome.cy + chrome.hh - height - 8.0,
            height,
            (height / 2.0).min(chrome.radius * 0.5).max(2.0),
            9.0,
            0.75,
            chrome.opacity,
        );
    }
}

fn draw_grid(frame: &mut Frame, palette: &Palette, chrome: &Chrome, show_type_badge: bool) {
    if show_type_badge {
        type_badge(
            frame,
            palette,
            chrome.kind,
            chrome.cx - chrome.hw + 4.0,
            chrome.cy + chrome.hh - 18.0,
            14.0,
            3.0,
            8.0,
            0.6,
            chrome.opacity,
        );
    }
    if chrome.has_video {
        video_indicator(
            frame,
            palette,
            chrome.cx - chrome.hw + 13.0,
            chrome.cy - chrome.hh + 13.0,
            18.0,
            chrome.opacity,
        );
    }
    if chrome.favourite {
        frame.fill_text(mid_text(
            ICON_FAV.to_string(),
            Point::new(chrome.cx + chrome.hw - 11.0, chrome.cy - chrome.hh + 11.0),
            with_alpha(palette.primary, chrome.opacity),
            14.0,
            NERD_FONT,
            Alignment::Center,
        ));
    }
}

fn draw_hex(frame: &mut Frame, palette: &Palette, chrome: &Chrome, show_type_badge: bool) {
    if show_type_badge {
        let height = 18.0;
        let label = badge_label(chrome.kind);
        let width = label.chars().count() as f32 * 9.0 * 0.7 + 14.0;
        type_badge(
            frame,
            palette,
            chrome.kind,
            chrome.cx - width / 2.0,
            chrome.cy + chrome.hh - height - chrome.hw * 0.18,
            height,
            9.0,
            9.0,
            0.75,
            chrome.opacity,
        );
    }
    if chrome.has_video {
        video_indicator(
            frame,
            palette,
            chrome.cx + chrome.hw * 0.5 - 14.0,
            chrome.cy - chrome.hh * 0.866 + 18.0,
            20.0,
            chrome.opacity,
        );
    }
}
