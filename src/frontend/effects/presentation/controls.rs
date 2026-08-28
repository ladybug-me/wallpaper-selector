use iced::widget::{column, container, row, scrollable, text, text_input};
use iced::{Alignment, Element, Length};

use crate::app::Message;
use crate::domain::effects::{EffectParam, EffectParamKind, EffectValue};
use crate::frontend::theme::{Palette, parse_hex};
use crate::frontend::ui::{UI_FONT, folio_horizontal_rule, legible_type_scale, with_alpha};

use super::super::state::EffectsMsg;

pub(super) fn param_slider<'a>(
    parameter: &'a EffectParam,
    id: &str,
    label: &'a str,
    current: Option<&EffectValue>,
    focused: bool,
    scale: f32,
    palette: &'a Palette,
) -> Element<'a, Message> {
    let value = current.and_then(EffectValue::as_f64).unwrap_or(parameter.min);
    let decimals = if matches!(&parameter.kind, EffectParamKind::Integer) { 0 } else { 2 };
    let slider_id = id.to_string();

    column![
        folio_horizontal_rule(with_alpha(palette.outline, 0.38)),
        row![
            crate::frontend::ui::label(
                if focused { format!("◆  {label}") } else { label.to_string() },
                11.0,
                scale,
                if focused { palette.primary } else { palette.surface_text },
            ),
            container(text("")).width(Length::Fill),
            crate::frontend::ui::label(format!("{value:.decimals$}"), 12.0, scale, palette.primary),
        ]
        .align_y(Alignment::Center),
        crate::frontend::ui::folio_slider(
            parameter.min,
            parameter.max,
            value,
            parameter.step,
            move |value| Message::Effects(EffectsMsg::SetNum(slider_id.clone(), value)),
            Message::Effects(EffectsMsg::Preview),
            palette,
        ),
        crate::frontend::ui::label(
            format!("{:.decimals$} - {:.decimals$}", parameter.min, parameter.max),
            8.5,
            scale,
            with_alpha(palette.surface_text, 0.38),
        ),
    ]
    .spacing(8.0 * scale)
    .into()
}

pub(super) fn param_dropdown_reel<'a>(
    parameter: &'a EffectParam,
    id: &str,
    current: Option<&EffectValue>,
    label: &'a str,
    focused: bool,
    scale: f32,
    palette: &'a Palette,
) -> Element<'a, Message> {
    let current = current.and_then(EffectValue::as_str).unwrap_or_default();
    let mut options = row![].spacing(6.0 * scale).align_y(Alignment::Center);
    for option in &parameter.options {
        let active = option.mode == current;
        let mode = option.mode.clone();
        let message = Message::Effects(EffectsMsg::SetChoice(id.to_string(), mode));
        let width = (option.label.chars().count() as f32 * 7.4 + 30.0).clamp(74.0, 160.0);
        options = options.push(crate::frontend::ui::folio_action(
            option.label.as_str(),
            active,
            Some(message),
            Length::Fixed(width * scale.max(0.95)),
            scale * 0.92,
            palette,
        ));
    }

    let reel = crate::frontend::ui::smooth_pane(
        "fx.params",
        scrollable(options.padding(iced::Padding { bottom: 4.0, ..iced::Padding::ZERO }))
            .direction(crate::frontend::ui::thin_hbar())
            .style(crate::frontend::ui::scroll_style(palette.primary)),
        Length::Fill,
        Length::Fixed(39.0 * scale.max(1.0)),
        Message::PaneWheel,
        Message::PaneScrolled,
    );

    column![
        folio_horizontal_rule(with_alpha(palette.outline, 0.38)),
        crate::frontend::ui::label(
            if focused { format!("◆  {label}") } else { label.to_string() },
            11.0,
            scale,
            if focused { palette.primary } else { palette.surface_text },
        ),
        reel,
    ]
    .spacing(8.0 * scale)
    .into()
}

pub(super) fn param_color<'a>(
    id: String,
    label: String,
    current: Option<&EffectValue>,
    focused: bool,
    scale: f32,
    palette: &'a Palette,
) -> Element<'a, Message> {
    let value = current.and_then(EffectValue::as_str).unwrap_or("#000000").to_string();
    let swatch = parse_hex(&value).unwrap_or(palette.surface_variant);
    let input_palette = *palette;
    let input = text_input("#rrggbb", &value)
        .on_input(move |value| Message::Effects(EffectsMsg::SetStr(id.clone(), value)))
        .on_submit(Message::Effects(EffectsMsg::Preview))
        .padding([7.0 * scale, 10.0 * scale])
        .size(11.5 * legible_type_scale(scale))
        .font(UI_FONT)
        .style(move |_theme, _status| crate::frontend::ui::ghost_input_style(&input_palette));

    column![
        folio_horizontal_rule(with_alpha(palette.outline, 0.38)),
        crate::frontend::ui::label(
            if focused { format!("◆  {label}") } else { label },
            11.0,
            scale,
            if focused { palette.primary } else { palette.surface_text },
        ),
        row![
            container(text(""))
                .width(Length::Fixed(38.0 * scale))
                .height(Length::Fixed(31.0 * scale.max(1.0)))
                .style(move |_| crate::frontend::ui::box_style(
                    swatch,
                    with_alpha(palette.outline, 0.58),
                )),
            container(input).width(Length::Fill).style(move |_| crate::frontend::ui::box_style(
                with_alpha(palette.background, 0.76),
                with_alpha(palette.outline, 0.42),
            )),
        ]
        .spacing(7.0 * scale)
        .align_y(Alignment::Center),
    ]
    .spacing(8.0 * scale)
    .into()
}
