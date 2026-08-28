use iced::widget::{column, container, row, scrollable, text};
use iced::{Alignment, Color, Element, Length};

use super::state::{ScenePropMsg, SceneProperties};
use crate::app::Message;
use crate::domain::scene_properties::{SceneProperty, ScenePropertyKind, format_number};
use crate::frontend::theme::Palette;
use crate::frontend::ui::{
    FOLIO_INDEX_WIDTH, TYPE_SMALL, folio_action, folio_action_wrap, folio_field, folio_ghost_field,
    folio_index_shell, folio_masthead, folio_scroll_padding, folio_sheet, folio_slider, label,
    with_alpha,
};
use crate::i18n::{tr, tr_args};

const SECTION_SPACING: f32 = 14.0;
const CONTROL_WIDTH: f32 = 420.0;

fn wrap(message: ScenePropMsg) -> Message {
    Message::SceneProps(message)
}

fn swatch<'a>(property: &SceneProperty, scale: f32, palette: &Palette) -> Element<'a, Message> {
    let colour = property
        .value
        .colour()
        .map_or(palette.surface, |[red, green, blue]| Color::from_rgb(red, green, blue));
    container(text(""))
        .width(Length::Fixed(38.0 * scale))
        .height(Length::Fixed(20.0 * scale))
        .style(move |_| crate::frontend::ui::box_style(colour, with_alpha(Color::BLACK, 0.38)))
        .into()
}

fn control<'a>(
    property: &'a SceneProperty,
    panel: &'a SceneProperties,
    scale: f32,
    palette: &'a Palette,
) -> Element<'a, Message> {
    let name = property.name.clone();
    match property.kind {
        ScenePropertyKind::Flag => {
            let on = property.value.flag();
            let caption = if on { tr("scene-props-on") } else { tr("scene-props-off") };
            folio_action(
                caption,
                on,
                Some(wrap(ScenePropMsg::Toggle(name))),
                Length::Shrink,
                scale,
                palette,
            )
        }
        ScenePropertyKind::Range => {
            let (min, max, step) = property.range();
            let value = property.value.number().clamp(min, max);
            let slide_name = name.clone();
            row![
                container(folio_slider(
                    min,
                    max,
                    value,
                    step,
                    move |next| wrap(ScenePropMsg::Slide(slide_name.clone(), next)),
                    wrap(ScenePropMsg::Commit(name)),
                    palette,
                ))
                .width(Length::Fill),
                label(format_number(value), 11.0, scale, palette.surface_text),
            ]
            .spacing(10.0 * scale)
            .align_y(Alignment::Center)
            .into()
        }
        ScenePropertyKind::Choice => {
            let selected = property.value.number();
            let actions = property
                .choices
                .iter()
                .map(|choice| {
                    let caption = if choice.label.is_empty() {
                        format_number(choice.value)
                    } else {
                        choice.label.clone()
                    };
                    let active = (choice.value - selected).abs() < f64::EPSILON;
                    (caption, active, wrap(ScenePropMsg::Choose(name.clone(), choice.value)))
                })
                .collect();
            folio_action_wrap(actions, CONTROL_WIDTH * scale, scale, palette)
        }
        ScenePropertyKind::Colour => {
            let draft = panel.colour_draft(&property.name).unwrap_or_default();
            let input_name = name.clone();
            row![
                swatch(property, scale, palette),
                folio_ghost_field(
                    draft,
                    "1.000 1.000 1.000",
                    move |next| wrap(ScenePropMsg::ColourInput(input_name.clone(), next)),
                    wrap(ScenePropMsg::ColourCommit(name)),
                    Length::Fixed(190.0 * scale),
                    scale,
                    palette,
                ),
            ]
            .spacing(10.0 * scale)
            .align_y(Alignment::Center)
            .into()
        }
        ScenePropertyKind::Group | ScenePropertyKind::Unsupported => label(
            tr("scene-props-unsupported"),
            TYPE_SMALL,
            scale,
            with_alpha(palette.surface_text, 0.48),
        )
        .into(),
    }
}

fn notice(message: &str, colour: Color, scale: f32) -> Element<'_, Message> {
    container(label(message, 12.0, scale, colour)).padding(24.0 * scale).into()
}

fn reading<'a>(
    panel: &'a SceneProperties,
    scale: f32,
    palette: &'a Palette,
) -> Element<'a, Message> {
    if panel.loading {
        return notice(tr("scene-props-loading"), palette.surface_text, scale);
    }
    if let Some(error) = &panel.error {
        return notice(error.as_str(), palette.primary, scale);
    }
    if panel.rows.is_empty() {
        return notice(tr("scene-props-empty"), palette.surface_text, scale);
    }

    let mut body = column![].spacing(SECTION_SPACING * scale);
    let mut index = 0_usize;
    for property in &panel.rows {
        if property.is_group() {
            body = body.push(
                container(label(property.label.as_str(), 12.0, scale, palette.primary))
                    .padding([10.0 * scale, 8.0 * scale]),
            );
            continue;
        }
        index += 1;
        let marker = if property.overridden {
            tr("scene-props-changed").to_string()
        } else {
            format!("{index:02}")
        };
        let description = if property.overridden {
            tr_args!("scene-props-default", value => property.default.display())
        } else {
            property.name.clone()
        };
        body = body.push(folio_field(
            marker,
            property.label.as_str(),
            description,
            control(property, panel, scale, palette),
            scale,
            palette,
        ));
    }

    container(scrollable(body.padding(folio_scroll_padding(23.0, 27.0, scale))))
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn index_column<'a>(
    panel: &'a SceneProperties,
    scale: f32,
    palette: &'a Palette,
) -> Element<'a, Message> {
    let note = tr_args!(
        "scene-props-count",
        editable => panel.editable_count(),
        changed => panel.overridden_count()
    );
    let reset = folio_action(
        tr("scene-props-reset"),
        false,
        (panel.overridden_count() > 0).then(|| wrap(ScenePropMsg::Reset)),
        Length::Fill,
        scale,
        palette,
    );
    folio_index_shell(panel.title.as_str(), note, vec![reset], 12.0 * scale, scale, palette)
}

pub fn view<'a>(
    panel: &'a SceneProperties,
    viewport: (f32, f32),
    scale: f32,
    palette: &'a Palette,
) -> Element<'a, Message> {
    folio_sheet(
        folio_masthead(
            tr("scene-props-crumb").to_string(),
            wrap(ScenePropMsg::Close),
            scale,
            palette,
        ),
        index_column(panel, scale, palette),
        reading(panel, scale, palette),
        Message::Noop,
        viewport,
        scale,
        1.0,
        FOLIO_INDEX_WIDTH,
        palette,
    )
}
