use iced::widget::canvas::{self, Stroke};
use iced::{Alignment, Color, Point, Rectangle, Size, Vector};

use crate::frontend::components::cut_rect;
use crate::frontend::theme::Palette;
use crate::i18n::tr;

use super::super::color::swatch_color;
use super::super::misc::{FadeFrame, NERD_FONT, UI_FONT, glyph_width, mid_text};
use super::super::{parallelogram, with_alpha};
use super::canvas::FilterBar;
use super::catalog::{DROP_ARROW, DROP_ARROW_UP, ICON_FOLDER, MENU_MAX_ROWS};
use super::menu::{MenuKind, folder_depth, folder_leaf};
use super::model::{BarItem, BarNotice, BarVisualStyle};

pub(super) fn draw_filter_bar(
    bar: &FilterBar<'_>,
    renderer: &iced::Renderer,
    bounds: Rectangle,
) -> canvas::Geometry {
    let palette = bar.pal;
    bar.cache.draw(renderer, bounds.size(), |frame| {
        let frame = &mut FadeFrame::new(frame, bar.fade.clamp(0.0, 1.0));
        let mut order: Vec<usize> = (0..bar.model.items.len()).collect();
        order.sort_by_key(|&index| {
            let depth = bar.model.items[index].z;
            if Some(index) == bar.hover && depth < 5 { 5 } else { depth }
        });
        for index in order {
            draw_item(
                frame,
                &bar.model.items[index],
                Some(index) == bar.hover,
                palette,
                bar.visual_style,
            );
        }
        if let Some((x, y)) = bar.model.swatch_at
            && !bar.theme_swatch.is_empty()
        {
            draw_swatch_strip(frame, bar, x, y);
        }
        if let Some(rectangle) = bar.menu_rect() {
            draw_menu(frame, rectangle, bar, palette);
        }
    })
}

fn draw_item(
    frame: &mut FadeFrame<'_>,
    item: &BarItem,
    hovered: bool,
    palette: &Palette,
    style: BarVisualStyle,
) {
    if let Some(swatch_index) = item.swatch {
        draw_swatch_styled(frame, item, swatch_index, hovered, palette, style);
    } else if let Some(notice) = item.notice.as_ref() {
        draw_notice(frame, item, notice, palette, style);
    } else if item.action.is_some() {
        draw_button_styled(frame, item, hovered, palette, style);
    } else {
        let (visual_x, visual_w) = item_visual_bounds(item, style);
        frame.fill_text(mid_text(
            item.label.clone(),
            Point::new(visual_x + visual_w / 2.0, item.y + item.h / 2.0),
            with_alpha(palette.surface_text, 0.7),
            item.text_size,
            UI_FONT,
            Alignment::Center,
        ));
    }
}

fn draw_notice(
    frame: &mut FadeFrame<'_>,
    item: &BarItem,
    notice: &BarNotice,
    palette: &Palette,
    style: BarVisualStyle,
) {
    let accent = match &notice.state {
        crate::contracts::daemon::TaskState::Running
        | crate::contracts::daemon::TaskState::Completed => palette.primary,
        crate::contracts::daemon::TaskState::Paused
        | crate::contracts::daemon::TaskState::Failed => palette.tertiary,
        crate::contracts::daemon::TaskState::Cancelled
        | crate::contracts::daemon::TaskState::Other(_) => palette.outline,
    };
    let (fill, border, text_alpha, marker_alpha) = match &notice.state {
        crate::contracts::daemon::TaskState::Running => (
            control_background(palette, false, false),
            control_border(palette, false, false),
            0.9,
            0.9,
        ),
        crate::contracts::daemon::TaskState::Paused
        | crate::contracts::daemon::TaskState::Other(_) => (
            control_background(palette, false, false),
            control_border(palette, false, false),
            0.84,
            0.82,
        ),
        crate::contracts::daemon::TaskState::Completed => (
            with_alpha(palette.surface_container, 0.74),
            with_alpha(palette.outline, 0.28),
            0.68,
            0.58,
        ),
        crate::contracts::daemon::TaskState::Failed => (
            control_background(palette, false, false),
            with_alpha(palette.tertiary, 0.58),
            0.9,
            1.0,
        ),
        crate::contracts::daemon::TaskState::Cancelled => (
            with_alpha(palette.surface_container, 0.68),
            with_alpha(palette.outline, 0.24),
            0.62,
            0.48,
        ),
    };
    let path = item_path(item, style);
    frame.fill(&path, fill);
    frame.stroke(&path, Stroke::default().with_color(border).with_width(1.0));

    let scale = item.h / 24.0;
    let (visual_x, visual_w) = item_visual_bounds(item, style);
    let leading_edge = match style {
        BarVisualStyle::Slices => visual_x + item.skew * 0.5,
        BarVisualStyle::Hex => visual_x + item.h * 0.14,
        BarVisualStyle::Wall => visual_x,
    };
    let marker_x = leading_edge + 8.0 * scale;
    let marker = canvas::Path::circle(Point::new(marker_x, item.y + item.h / 2.0), 2.5 * scale);
    frame.fill(&marker, with_alpha(accent, marker_alpha));
    frame.fill_text(mid_text(
        item.label.clone(),
        Point::new(marker_x + 8.0 * scale, item.y + item.h / 2.0),
        with_alpha(palette.surface_text, text_alpha),
        item.text_size,
        UI_FONT,
        Alignment::Start,
    ));

    let Some(progress) = notice.progress else {
        return;
    };
    let track_inset = match style {
        BarVisualStyle::Hex => item.h * 0.3,
        BarVisualStyle::Slices | BarVisualStyle::Wall => 2.0 * scale,
    };
    let trailing_inset =
        if style == BarVisualStyle::Slices { item.skew + 2.0 * scale } else { track_inset };
    let track_width = (visual_w - track_inset - trailing_inset).max(1.0);
    let track_y = item.y + item.h - 2.5 * scale;
    let track = canvas::Path::rectangle(
        Point::new(visual_x + track_inset, track_y),
        Size::new(track_width, 1.5 * scale),
    );
    frame.fill(&track, with_alpha(palette.outline, 0.24));
    let fill = canvas::Path::rectangle(
        Point::new(visual_x + track_inset, track_y),
        Size::new(track_width * progress.clamp(0.0, 1.0), 1.5 * scale),
    );
    frame.fill(&fill, with_alpha(accent, marker_alpha));
}

fn draw_swatch_strip(frame: &mut FadeFrame<'_>, bar: &FilterBar<'_>, x: f32, y: f32) {
    let cell = 14.0 * bar.scale;
    for (index, color) in bar.theme_swatch.iter().take(6).enumerate() {
        let shape =
            style_path(x + index as f32 * cell, y, cell, cell, 3.0 * bar.scale, bar.visual_style);
        frame.fill(&shape, *color);
        frame.stroke(
            &shape,
            Stroke::default().with_color(with_alpha(Color::BLACK, 0.25)).with_width(1.0),
        );
    }
}

fn item_background(
    frame: &mut FadeFrame<'_>,
    item: &BarItem,
    hovered: bool,
    palette: &Palette,
    style: BarVisualStyle,
) {
    let fill = control_background(palette, item.active, hovered);
    let stroke = control_border(palette, item.active, hovered);
    let path = item_path(item, style);
    frame.fill(&path, fill);
    frame.stroke(&path, Stroke::default().with_color(stroke).with_width(1.0));
}

pub(crate) fn control_background(palette: &Palette, active: bool, hovered: bool) -> Color {
    if active {
        palette.primary
    } else if hovered {
        with_alpha(palette.surface_variant, 0.82)
    } else {
        with_alpha(palette.surface_container, 0.92)
    }
}

pub(crate) fn control_text(palette: &Palette, active: bool) -> Color {
    if active { palette.primary_text } else { palette.surface_text }
}

pub(crate) fn control_border(palette: &Palette, active: bool, hovered: bool) -> Color {
    with_alpha(
        palette.outline,
        if active {
            0.72
        } else if hovered {
            0.58
        } else {
            0.4
        },
    )
}

fn item_visual_bounds(item: &BarItem, style: BarVisualStyle) -> (f32, f32) {
    if style == BarVisualStyle::Slices {
        (item.x, item.w)
    } else {
        (item.x, (item.w - item.skew).max(1.0))
    }
}

fn item_path(item: &BarItem, style: BarVisualStyle) -> canvas::Path {
    let (x, width) = item_visual_bounds(item, style);
    style_path(x, item.y, width, item.h, item.skew, style)
}

fn style_path(
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    accent: f32,
    style: BarVisualStyle,
) -> canvas::Path {
    match style {
        BarVisualStyle::Slices => parallelogram(x, y, width, height, accent.min(width * 0.35)),
        BarVisualStyle::Hex => {
            let cut = (height * 0.28).min(width * 0.18).max(2.0);
            canvas::Path::new(|builder| {
                builder.move_to(Point::new(x + cut, y));
                builder.line_to(Point::new(x + width - cut, y));
                builder.line_to(Point::new(x + width, y + height / 2.0));
                builder.line_to(Point::new(x + width - cut, y + height));
                builder.line_to(Point::new(x + cut, y + height));
                builder.line_to(Point::new(x, y + height / 2.0));
                builder.close();
            })
        }
        BarVisualStyle::Wall => canvas::Path::rectangle(
            Point::new(x + 0.5, y + 0.5),
            Size::new((width - 1.0).max(1.0), (height - 1.0).max(1.0)),
        ),
    }
}

fn panel_path(rectangle: Rectangle, style: BarVisualStyle) -> canvas::Path {
    match style {
        BarVisualStyle::Slices => {
            cut_rect(rectangle.x, rectangle.y, rectangle.width, rectangle.height, 10.0)
        }
        BarVisualStyle::Hex => {
            style_path(rectangle.x, rectangle.y, rectangle.width, rectangle.height, 10.0, style)
        }
        BarVisualStyle::Wall => canvas::Path::rectangle(rectangle.position(), rectangle.size()),
    }
}

pub fn split_icon_label(label: &str) -> (&str, &str, &str) {
    let Some((icon, rest)) = label.split_once(' ') else {
        return (label, "", "");
    };
    for arrow in [DROP_ARROW, DROP_ARROW_UP] {
        if let Some(name) = rest.strip_suffix(arrow) {
            return (icon, name.trim_end(), arrow);
        }
    }
    (icon, rest, "")
}

pub(crate) fn draw_button(
    frame: &mut FadeFrame<'_>,
    item: &BarItem,
    hovered: bool,
    palette: &Palette,
) {
    draw_button_styled(frame, item, hovered, palette, BarVisualStyle::Slices);
}

fn draw_button_styled(
    frame: &mut FadeFrame<'_>,
    item: &BarItem,
    hovered: bool,
    palette: &Palette,
    style: BarVisualStyle,
) {
    item_background(frame, item, hovered, palette, style);
    let text_color = control_text(palette, item.active);
    if item.nerd && item.label.contains(' ') {
        draw_icon_label(frame, item, text_color, style);
        return;
    }
    let (visual_x, visual_w) = item_visual_bounds(item, style);
    frame.fill_text(mid_text(
        item.label.clone(),
        Point::new(visual_x + visual_w / 2.0, item.y + item.h / 2.0),
        text_color,
        item.text_size,
        if item.nerd { NERD_FONT } else { UI_FONT },
        Alignment::Center,
    ));
}

fn draw_icon_label(
    frame: &mut FadeFrame<'_>,
    item: &BarItem,
    text_color: Color,
    style: BarVisualStyle,
) {
    let (icon, name, arrow) = split_icon_label(&item.label);
    let center_y = item.y + item.h / 2.0;
    let (visual_x, visual_w) = item_visual_bounds(item, style);
    let icon_x =
        visual_x + if style == BarVisualStyle::Slices { item.skew.max(6.0) + 4.0 } else { 10.0 };
    frame.fill_text(mid_text(
        icon.to_string(),
        Point::new(icon_x, center_y),
        text_color,
        item.text_size,
        NERD_FONT,
        Alignment::Start,
    ));
    let name_x = icon_x + glyph_width(item.text_size) + 5.0;
    frame.fill_text(mid_text(
        name.to_string(),
        Point::new(name_x, center_y),
        text_color,
        item.text_size,
        UI_FONT,
        Alignment::Start,
    ));
    if arrow.is_empty() {
        return;
    }
    frame.fill_text(mid_text(
        arrow.to_string(),
        Point::new(visual_x + visual_w - 7.0, center_y),
        text_color,
        item.text_size,
        UI_FONT,
        Alignment::End,
    ));
}

pub(crate) fn draw_swatch_with_style(
    frame: &mut FadeFrame<'_>,
    item: &BarItem,
    index: usize,
    hovered: bool,
    palette: &Palette,
    style: BarVisualStyle,
) {
    draw_swatch_styled(frame, item, index, hovered, palette, style);
}

fn draw_swatch_styled(
    frame: &mut FadeFrame<'_>,
    item: &BarItem,
    index: usize,
    hovered: bool,
    palette: &Palette,
    style: BarVisualStyle,
) {
    let scale = if item.active { 1.15 } else { 1.0 };
    let (visual_x, visual_w) = item_visual_bounds(item, style);
    let center_x = visual_x + visual_w / 2.0;
    let center_y = item.y + item.h / 2.0;
    frame.with_save(|frame| {
        frame.translate(Vector::new(center_x, center_y));
        frame.scale(scale);
        frame.translate(Vector::new(-center_x, -center_y));
        let outer = item_path(item, style);
        frame.fill(&outer, with_alpha(palette.background, 0.9));
        let mut fill = swatch_color(index, item.active || hovered);
        if hovered && !item.active {
            fill = Color {
                r: (fill.r * 1.2).min(1.0),
                g: (fill.g * 1.2).min(1.0),
                b: (fill.b * 1.2).min(1.0),
                a: 1.0,
            };
        }
        let inset = 1.0;
        let inner = style_path(
            visual_x + inset,
            item.y + inset,
            visual_w - 2.0 * inset,
            item.h - 2.0 * inset,
            item.skew * (item.h - 2.0 * inset) / item.h,
            style,
        );
        frame.fill(&inner, fill);
        if item.active {
            frame.stroke(&outer, Stroke::default().with_color(palette.primary).with_width(1.5));
        }
    });
}

fn draw_menu(
    frame: &mut FadeFrame<'_>,
    rectangle: Rectangle,
    bar: &FilterBar<'_>,
    palette: &Palette,
) {
    let background = panel_path(rectangle, bar.visual_style);
    frame.fill(&background, with_alpha(palette.surface, 0.97));
    frame.stroke(
        &background,
        Stroke::default().with_color(with_alpha(palette.primary, 0.4)).with_width(1.0),
    );
    let row_height = bar.menu_row_h();
    let backends = bar.active_menu() == Some(MenuKind::Backends);
    let rows = if backends { bar.backend_options.len() } else { bar.folder_options.len() };
    frame.with_clip(rectangle, |frame| {
        for index in 0..rows {
            let y = rectangle.y + 4.0 + index as f32 * row_height - bar.menu_scroll;
            if y + row_height < rectangle.y || y > rectangle.y + rectangle.height {
                continue;
            }
            if backends {
                draw_backend_row(frame, rectangle, bar, palette, index, y);
            } else {
                draw_folder_row(frame, rectangle, bar, palette, index, y);
            }
        }
    });
    if !backends {
        draw_scrollbar(frame, rectangle, bar, palette);
    }
}

fn draw_backend_row(
    frame: &mut FadeFrame<'_>,
    rectangle: Rectangle,
    bar: &FilterBar<'_>,
    palette: &Palette,
    index: usize,
    y: f32,
) {
    let (key, label_key) = bar.backend_options[index];
    let label = tr(label_key);
    let row_height = bar.menu_row_h();
    let selected = bar.backend == key;
    if selected || bar.menu_hover == Some(index) {
        let row = panel_path(
            Rectangle::new(
                Point::new(rectangle.x + 4.0, y),
                Size::new(rectangle.width - 8.0, row_height - 2.0),
            ),
            bar.visual_style,
        );
        frame.fill(&row, with_alpha(palette.primary, if selected { 0.16 } else { 0.08 }));
    }
    frame.fill_text(mid_text(
        label.to_string(),
        Point::new(rectangle.x + 12.0, y + row_height / 2.0),
        if selected { palette.primary } else { palette.surface_text },
        10.0 * bar.scale,
        UI_FONT,
        Alignment::Start,
    ));
}

fn draw_folder_row(
    frame: &mut FadeFrame<'_>,
    rectangle: Rectangle,
    bar: &FilterBar<'_>,
    palette: &Palette,
    index: usize,
    y: f32,
) {
    let option = &bar.folder_options[index];
    let row_height = bar.menu_row_h();
    let size = 10.0 * bar.scale;
    let selected = option == bar.selected_folder;
    if selected || bar.menu_hover == Some(index) {
        let row = panel_path(
            Rectangle::new(
                Point::new(rectangle.x + 4.0, y),
                Size::new(rectangle.width - 8.0, row_height - 2.0),
            ),
            bar.visual_style,
        );
        frame.fill(&row, with_alpha(palette.primary, if selected { 0.16 } else { 0.08 }));
    }
    let center_y = y + row_height / 2.0;
    if let "" | "*" = option.as_str() {
        let shown = match option.as_str() {
            "*" => tr("filter-bar-folder-all"),
            _ => tr("filter-bar-folder-main"),
        };
        frame.fill_text(mid_text(
            shown.to_string(),
            Point::new(rectangle.x + 12.0, center_y),
            if selected { palette.primary } else { palette.surface_text },
            size,
            UI_FONT,
            Alignment::Start,
        ));
        return;
    }
    let base_x = rectangle.x + 12.0 + (folder_depth(option).min(6) as f32) * 16.0 * bar.scale;
    frame.fill_text(mid_text(
        ICON_FOLDER.to_string(),
        Point::new(base_x, center_y),
        if selected { palette.primary } else { with_alpha(palette.surface_text, 0.7) },
        size,
        NERD_FONT,
        Alignment::Start,
    ));
    frame.fill_text(mid_text(
        folder_leaf(option).to_string(),
        Point::new(base_x + glyph_width(size) + 5.0, center_y),
        if selected { palette.primary } else { palette.surface_text },
        size,
        UI_FONT,
        Alignment::Start,
    ));
}

fn draw_scrollbar(
    frame: &mut FadeFrame<'_>,
    rectangle: Rectangle,
    bar: &FilterBar<'_>,
    palette: &Palette,
) {
    let max_scroll = bar.menu_max_scroll();
    if max_scroll <= 0.0 {
        return;
    }
    let count = bar.menu_len() as f32;
    let visible = bar.menu_len().min(MENU_MAX_ROWS) as f32;
    let track_height = rectangle.height - 8.0;
    let thumb_height = (visible / count * track_height).max(12.0);
    let fraction = (bar.menu_scroll / max_scroll).clamp(0.0, 1.0);
    let thumb_y = rectangle.y + 4.0 + fraction * (track_height - thumb_height);
    let scrollbar = cut_rect(rectangle.x + rectangle.width - 5.0, thumb_y, 3.0, thumb_height, 1.0);
    frame.fill(&scrollbar, with_alpha(palette.primary, 0.5));
}
