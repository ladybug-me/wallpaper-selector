use iced::widget::canvas::Text;
use iced::{Alignment, Color, Font, Point, alignment};

pub const NERD_FONT: Font = Font::with_name("Symbols Nerd Font");

pub const UI_FONT: Font = Font {
    family: iced::font::Family::Name("Roboto Condensed"),
    weight: iced::font::Weight::Bold,
    stretch: iced::font::Stretch::Normal,
    style: iced::font::Style::Normal,
};

pub const TYPE_SMALL: f32 = 11.0;

pub fn legible_type_scale(scale: f32) -> f32 {
    scale.max(0.95)
}

pub fn label<'a>(
    content: impl iced::widget::text::IntoFragment<'a>,
    size: f32,
    scale: f32,
    color: Color,
) -> iced::widget::Text<'a> {
    iced::widget::text(content).font(UI_FONT).size(size * legible_type_scale(scale)).color(color)
}

pub fn sentence_case(label: &str) -> String {
    let mut value = label.to_ascii_lowercase();
    if let Some(first) = value.get_mut(0..1) {
        first.make_ascii_uppercase();
    }
    value
}

pub(crate) fn glyph_width(size: f32) -> f32 {
    size * 1.1
}

pub(crate) fn text_width(label: &str, size: f32, nerd: bool) -> f32 {
    let count = label.chars().count() as f32;
    if nerd { count * glyph_width(size) } else { count * size * 0.64 }
}

pub(crate) fn ellipsize_text(label: &str, size: f32, max_width: f32) -> String {
    if text_width(label, size, false) <= max_width {
        return label.to_owned();
    }
    let ellipsis = '…';
    let char_width = size * 0.64;
    let keep = ((max_width / char_width).floor() as usize).saturating_sub(1);
    let mut fitted: String = label.chars().take(keep).collect();
    fitted.push(ellipsis);
    fitted
}

pub fn mid_text(
    content: impl Into<String>,
    position: Point,
    color: Color,
    size: f32,
    font: Font,
    align_x: Alignment,
) -> Text {
    Text {
        content: content.into(),
        position,
        color,
        size: size.into(),
        font,
        align_x: align_x.into(),
        align_y: alignment::Vertical::Center,
        ..Text::default()
    }
}
