use iced::Element;

use crate::frontend::theme::Palette;

use super::style::field_input_style;

pub fn inert_backdrop<'a, Message: Clone + 'a>(
    content: impl Into<Element<'a, Message>>,
    capture: Message,
) -> Element<'a, Message> {
    iced::widget::mouse_area(content).on_press(capture).into()
}

pub fn field_input<'a, Message: Clone + 'a>(
    value: &str,
    placeholder: &'a str,
    on_input: impl Fn(String) -> Message + 'a,
    on_submit: Message,
    width: iced::Length,
    scale: f32,
    pal: &'a Palette,
) -> Element<'a, Message> {
    iced::widget::text_input(placeholder, value)
        .on_input(on_input)
        .on_submit(on_submit)
        .padding(5.0 * scale)
        .size(12.0 * scale)
        .width(width)
        .style(move |_theme, _status| field_input_style(pal))
        .into()
}
