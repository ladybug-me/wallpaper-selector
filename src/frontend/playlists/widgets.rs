use iced::{Element, Length};

use crate::app::Message;
use crate::frontend::theme::Palette;

pub(super) fn hint_line<'a>(hint: &'a str, scale: f32, palette: &Palette) -> Element<'a, Message> {
    iced::widget::text(hint)
        .font(crate::frontend::ui::UI_FONT)
        .size(10.0 * scale.max(0.95))
        .color(crate::frontend::ui::with_alpha(palette.surface_text, 0.44))
        .into()
}

pub(super) fn field<'a>(
    value: &'a str,
    placeholder: &'a str,
    on_input: fn(String) -> Message,
    on_submit: Message,
    width: Length,
    scale: f32,
    palette: &'a Palette,
) -> Element<'a, Message> {
    crate::frontend::ui::field_input(value, placeholder, on_input, on_submit, width, scale, palette)
}

pub(super) fn row_chip(
    label: String,
    active: bool,
    message: Message,
    scale: f32,
    palette: &Palette,
) -> Element<'_, Message> {
    crate::frontend::ui::folio_action(label, active, Some(message), Length::Fill, scale, palette)
}
