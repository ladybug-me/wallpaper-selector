use iced::widget::{container, text};
use iced::{Element, Length};

use crate::app::Message;
use crate::frontend::components::with_alpha;
use crate::frontend::theme::Palette;

pub(super) fn thumb_placeholder<'a>(
    width: f32,
    height: f32,
    palette: &Palette,
) -> Element<'a, Message> {
    let background = with_alpha(palette.surface_variant, 0.4);
    container(text(""))
        .width(Length::Fixed(width))
        .height(Length::Fixed(height))
        .style(move |_| crate::frontend::ui::bg_style(background))
        .into()
}
