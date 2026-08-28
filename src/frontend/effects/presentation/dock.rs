use iced::widget::{button, column, container, row, scrollable, text};
use iced::{Alignment, Background, Element, Length, Padding};

use crate::app::Message;
use crate::domain::effects::EffectParamKind;
use crate::frontend::theme::Palette;
use crate::frontend::ui::{UI_FONT, folio_rule, label, legible_type_scale, with_alpha};
use crate::i18n::tr;

use super::super::state::{Effects, EffectsMsg, NAV_CATEGORY, NAV_CONFIG_BASE, NAV_EFFECT};
use super::controls::{param_color, param_dropdown_reel, param_slider};

impl Effects {
    pub(super) fn effect_index<'a>(
        &'a self,
        scale: f32,
        palette: &'a Palette,
    ) -> Element<'a, Message> {
        let current_category =
            self.selected().map(|effect| effect.category.as_str()).unwrap_or_default();
        let mut tree = column![].spacing(1.0 * scale);
        if self.has_effects_page() {
            for category in self.categories() {
                let active = category == current_category;
                let focused = active && self.editor.nav_focus == NAV_CATEGORY;
                let first = self.first_effect_of(&category);
                tree = tree.push(
                    button(
                        row![
                            text(if focused {
                                "◆"
                            } else if active {
                                "▾"
                            } else {
                                "▸"
                            })
                            .font(UI_FONT)
                            .size(9.0 * legible_type_scale(scale)),
                            text(category.to_ascii_lowercase())
                                .font(UI_FONT)
                                .size(12.5 * legible_type_scale(scale)),
                            container(text("")).width(Length::Fill),
                        ]
                        .spacing(9.0 * scale)
                        .align_y(Alignment::Center),
                    )
                    .width(Length::Fill)
                    .padding([8.0 * scale, 8.0 * scale])
                    .on_press(Message::Effects(EffectsMsg::Select(first)))
                    .style(move |_theme, status| {
                        crate::frontend::ui::folio_line_button_style(
                            active, false, palette, 1.0, status,
                        )
                    }),
                );
                if active {
                    let mut branch = column![].spacing(1.0 * scale);
                    for effect in
                        self.editor.definitions.iter().filter(|effect| effect.category == category)
                    {
                        let selected = effect.id == self.editor.selected_id;
                        let saved = self.is_saved(&effect.id);
                        let focused = selected && self.editor.nav_focus == NAV_EFFECT;
                        let id = effect.id.clone();
                        branch = branch.push(
                            button(
                                row![
                                    label(
                                        if focused {
                                            "◆"
                                        } else if selected {
                                            "●"
                                        } else {
                                            "◇"
                                        },
                                        8.0,
                                        scale,
                                        with_alpha(
                                            palette.primary,
                                            if selected { 1.0 } else { 0.44 },
                                        ),
                                    ),
                                    text(&effect.label)
                                        .font(UI_FONT)
                                        .size(11.5 * legible_type_scale(scale)),
                                    container(text("")).width(Length::Fill),
                                    label(
                                        if saved { "✓" } else { "" },
                                        10.0,
                                        scale,
                                        palette.primary,
                                    ),
                                ]
                                .spacing(9.0 * scale)
                                .align_y(Alignment::Center),
                            )
                            .width(Length::Fill)
                            .padding([7.0 * scale, 7.0 * scale])
                            .on_press(Message::Effects(EffectsMsg::Select(id)))
                            .style(move |_theme, status| {
                                crate::frontend::ui::folio_line_button_style(
                                    selected, false, palette, 1.0, status,
                                )
                            }),
                        );
                    }
                    let guide = container(text(""))
                        .width(Length::Fixed(1.0))
                        .height(Length::Fill)
                        .style(move |_| {
                            crate::frontend::ui::bg_style(Background::Color(with_alpha(
                                palette.primary,
                                0.42,
                            )))
                        });
                    tree = tree.push(
                        container(row![guide, branch].spacing(9.0 * scale))
                            .padding(Padding {
                                top: 2.0 * scale,
                                right: 0.0,
                                bottom: 5.0 * scale,
                                left: 17.0 * scale,
                            })
                            .width(Length::Fill),
                    );
                }
            }
        } else {
            tree = tree.push(
                container(
                    column![
                        label(tr("effects-static-required"), 13.0, scale, palette.surface_text),
                        label(
                            tr("effects-static-required-desc"),
                            9.5,
                            scale,
                            with_alpha(palette.surface_text, 0.48),
                        )
                        .line_height(iced::widget::text::LineHeight::Relative(1.4)),
                    ]
                    .spacing(6.0 * scale),
                )
                .padding(12.0 * scale),
            );
        }

        crate::frontend::ui::folio_index_shell_tinted(
            tr("effects-index-title"),
            tr("effects-index-desc"),
            vec![
                scrollable(tree)
                    .direction(iced::widget::scrollable::Direction::Vertical(
                        iced::widget::scrollable::Scrollbar::hidden(),
                    ))
                    .height(Length::Fill)
                    .into(),
            ],
            17.0,
            (0.78, 0.58),
            scale,
            palette,
        )
    }

    pub(super) fn parameter_sheet<'a>(
        &'a self,
        scale: f32,
        palette: &'a Palette,
    ) -> Element<'a, Message> {
        let Some(definition) = self.selected() else {
            return container(
                column![
                    folio_rule(palette),
                    label(
                        tr("effects-no-parameters"),
                        11.0,
                        scale,
                        with_alpha(palette.surface_text, 0.56),
                    ),
                ]
                .spacing(12.0 * scale),
            )
            .width(Length::Fill)
            .padding(22.0 * scale)
            .into();
        };

        let mut fields = row![].spacing(26.0 * scale).align_y(Alignment::Start);
        for (parameter_index, parameter) in definition.params.iter().enumerate() {
            let id = parameter.id.clone();
            let current = self.editor.values.get(&id);
            let focused = self.editor.nav_focus == NAV_CONFIG_BASE + parameter_index;
            let field: Element<'a, Message> = match &parameter.kind {
                EffectParamKind::Integer | EffectParamKind::Number => param_slider(
                    parameter,
                    &id,
                    parameter.label.as_str(),
                    current,
                    focused,
                    scale,
                    palette,
                ),
                EffectParamKind::Dropdown => param_dropdown_reel(
                    parameter,
                    &id,
                    current,
                    parameter.label.as_str(),
                    focused,
                    scale,
                    palette,
                ),
                EffectParamKind::Color => {
                    param_color(id, parameter.label.clone(), current, focused, scale, palette)
                }
                EffectParamKind::Other(_) => continue,
            };
            fields = fields.push(container(field).width(Length::FillPortion(1)));
        }

        let controls: Element<'a, Message> = if definition.params.is_empty() {
            label(
                tr("effects-no-controls"),
                crate::frontend::ui::TYPE_SMALL,
                scale,
                with_alpha(palette.surface_text, 0.52),
            )
            .into()
        } else {
            fields.into()
        };
        container(
            column![label(tr("effects-parameters"), 14.0, scale, palette.surface_text), controls,]
                .spacing(14.0 * scale),
        )
        .width(Length::Fill)
        .padding(Padding {
            top: 18.0 * scale,
            right: 24.0 * scale,
            bottom: 22.0 * scale,
            left: 24.0 * scale,
        })
        .style(move |_| {
            crate::frontend::ui::box_style(
                with_alpha(palette.surface, 0.84),
                with_alpha(palette.outline, 0.68),
            )
        })
        .into()
    }
}
