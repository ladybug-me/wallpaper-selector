use iced::widget::{
    button, column, container, mouse_area, pick_list, row, scrollable, text, text_input,
};
use iced::{Alignment, Background, Color, Element, Length};

use crate::app::Message;
use crate::domain::theme::Candidate;
use crate::frontend::theme::{Palette, ROLES};
use crate::frontend::theme_designer::{ThemeDesigner, ThemeMsg};
use crate::frontend::ui::{
    FOLIO_INDEX_WIDTH, UI_FONT, folio_horizontal_rule, folio_rule, label, legible_type_scale,
    with_alpha,
};
use crate::i18n::{tr, tr_args};

const COLOUR_FIELD_SIZE: f32 = 184.0;
const READING_SECTION_SPACING: f32 = 14.0;

#[derive(Clone, PartialEq, Eq)]
struct PresetOption {
    key: String,
    label: String,
}

impl std::fmt::Display for PresetOption {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.label)
    }
}

fn swatch<'a>(hex: &str, width: f32, height: f32, msg: Option<Message>) -> Element<'a, Message> {
    let color = crate::frontend::theme::parse_hex(hex).unwrap_or(Color::from_rgb(0.5, 0.5, 0.5));
    let cell = container(text(""))
        .width(Length::Fixed(width))
        .height(Length::Fixed(height))
        .style(move |_| crate::frontend::ui::box_style(color, with_alpha(Color::BLACK, 0.38)));
    match msg {
        Some(message) => {
            mouse_area(cell).on_press(message).interaction(iced::mouse::Interaction::Pointer).into()
        }
        None => cell.into(),
    }
}

fn folio_button<'a>(
    label: impl text::IntoFragment<'a>,
    active: bool,
    destructive: bool,
    message: Option<Message>,
    scale: f32,
    pal: &'a Palette,
) -> Element<'a, Message> {
    let control = button(text(label).font(UI_FONT).size(11.0 * legible_type_scale(scale)))
        .padding([7.0 * scale, 12.0 * scale])
        .style(move |_theme, status| {
            crate::frontend::ui::folio_button_style(active, destructive, pal, status)
        });
    match message {
        Some(message) => control.on_press(message).into(),
        None => control.into(),
    }
}

fn field<'a>(
    value: &str,
    placeholder: &'a str,
    on_input: impl Fn(String) -> Message + 'a,
    on_submit: Message,
    width: Length,
    scale: f32,
    pal: &'a Palette,
) -> Element<'a, Message> {
    text_input(placeholder, value)
        .font(UI_FONT)
        .on_input(on_input)
        .on_submit(on_submit)
        .padding([7.0 * scale, 9.0 * scale])
        .size(12.0 * legible_type_scale(scale))
        .width(width)
        .style(move |_theme, _status| {
            crate::frontend::ui::workbench_input_style(pal.surface_text, pal.primary)
        })
        .into()
}

impl ThemeDesigner {
    fn masthead<'a>(&self, scale: f32, pal: &'a Palette) -> Element<'a, Message> {
        let role = tr(ROLES[self.selected].name);
        crate::frontend::ui::folio_masthead(
            tr_args!("theme-designer-masthead", role => role),
            Message::Theme(ThemeMsg::DesignClose),
            scale,
            pal,
        )
    }

    fn palette_index<'a>(&'a self, scale: f32, pal: &'a Palette) -> Element<'a, Message> {
        let mut roles = column![].spacing(1.0 * scale);
        for role in ROLES {
            let index = role.role.index();
            let active = index == self.selected;
            let hex = &self.candidate.colors[index];
            let entry = button(
                row![
                    label(
                        if active { "◆" } else { "◇" },
                        8.0,
                        scale,
                        with_alpha(pal.primary, if active { 1.0 } else { 0.44 }),
                    ),
                    label(
                        format!("{:02}", index + 1),
                        9.0,
                        scale,
                        with_alpha(pal.primary, if active { 0.9 } else { 0.48 }),
                    ),
                    swatch(hex, 24.0 * scale, 16.0 * scale, None),
                    text(tr(role.name)).font(UI_FONT).size(11.5 * legible_type_scale(scale)),
                    container(text("")).width(Length::Fill),
                    label(hex, 9.0, scale, with_alpha(pal.surface_text, 0.46)),
                ]
                .spacing(8.0 * scale)
                .align_y(Alignment::Center),
            )
            .width(Length::Fill)
            .padding([8.0 * scale, 7.0 * scale])
            .on_press(Message::Theme(ThemeMsg::RoleSelect(index as u8)))
            .style(move |_theme, status| {
                crate::frontend::ui::folio_line_button_style(active, false, pal, 1.0, status)
            });
            roles = roles.push(entry);
        }

        let selected = &ROLES[self.selected];
        let footer = column![
            folio_horizontal_rule(with_alpha(pal.outline, 0.5)),
            label(
                format!("{:02}  {}", self.selected + 1, tr(selected.name)),
                10.0,
                scale,
                pal.primary
            ),
            label(
                tr(selected.description),
                crate::frontend::ui::TYPE_SMALL,
                scale,
                with_alpha(pal.surface_text, 0.5)
            )
            .line_height(iced::widget::text::LineHeight::Relative(1.4)),
        ]
        .spacing(8.0 * scale);

        crate::frontend::ui::folio_index_shell(
            tr("theme-designer-index-title"),
            crate::i18n::theme_designer_index_subtitle(ROLES.len()),
            vec![roles.into(), container(text("")).height(Length::Fill).into(), footer.into()],
            18.0,
            scale,
            pal,
        )
    }

    fn wheel_editor<'a>(&'a self, scale: f32, pal: &'a Palette) -> Element<'a, Message> {
        let wheel = iced::widget::canvas(crate::frontend::theme_designer::wheel::ColorWheel {
            hsv: self.hsv,
        })
        .width(Length::Fixed(COLOUR_FIELD_SIZE * scale))
        .height(Length::Fixed(COLOUR_FIELD_SIZE * scale));

        let mut recents = row![].spacing(6.0 * scale).align_y(Alignment::Center);
        if self.recent.is_empty() {
            recents = recents.push(label(
                tr("theme-designer-recent-empty"),
                9.5,
                scale,
                with_alpha(pal.surface_text, 0.42),
            ));
        } else {
            for hex in self.recent.iter().rev() {
                recents = recents.push(swatch(
                    hex,
                    28.0 * scale,
                    20.0 * scale,
                    Some(Message::Theme(ThemeMsg::Recent(hex.clone()))),
                ));
            }
        }

        let editor = column![
            row![
                label("01", 9.0, scale, pal.primary),
                label(tr("theme-designer-colour-field-title"), 14.0, scale, pal.surface_text),
            ]
            .spacing(9.0 * scale),
            label(
                tr("theme-designer-colour-field-desc"),
                10.5,
                scale,
                with_alpha(pal.surface_text, 0.56),
            ),
            container(wheel).width(Length::Fill).center_x(Length::Fill),
            container(
                row![
                    swatch(&self.candidate.colors[self.selected], 42.0 * scale, 30.0 * scale, None),
                    field(
                        &self.hex_buf,
                        tr("theme-designer-colour-placeholder"),
                        |value| Message::Theme(ThemeMsg::HexInput(value)),
                        Message::Theme(ThemeMsg::HexSubmit),
                        Length::Fixed(138.0 * scale),
                        scale,
                        pal,
                    ),
                    folio_button(
                        tr("theme-designer-set-colour"),
                        true,
                        false,
                        Some(Message::Theme(ThemeMsg::HexSubmit)),
                        scale,
                        pal,
                    ),
                ]
                .spacing(8.0 * scale)
                .align_y(Alignment::Center),
            )
            .width(Length::Fill)
            .center_x(Length::Fill),
            folio_horizontal_rule(with_alpha(pal.outline, 0.36)),
            label(tr("theme-designer-recent"), 9.0, scale, pal.primary),
            recents,
        ]
        .spacing(10.0 * scale);

        container(editor).width(Length::FillPortion(5)).padding([0.0, 12.0 * scale]).into()
    }

    fn folio_preview(&self, scale: f32) -> Element<'_, Message> {
        let candidate = Palette::from_candidate(&self.candidate);
        let selected_role = ROLES[self.selected];

        let active_chip = container(label(
            tr("theme-designer-proof-selected"),
            8.0,
            scale,
            candidate.primary_text,
        ))
        .padding([5.0 * scale, 12.0 * scale])
        .style(move |_| {
            crate::frontend::ui::box_style(candidate.primary, with_alpha(candidate.primary, 0.8))
        });
        let secondary_chip =
            container(label(tr("theme-designer-proof-secondary"), 8.0, scale, candidate.tertiary))
                .padding([4.0 * scale, 9.0 * scale])
                .style(move |_| {
                    crate::frontend::ui::box_style(
                        candidate.surface_container,
                        with_alpha(candidate.tertiary, 0.76),
                    )
                });

        let search = container(
            row![
                label("⌕", 8.0, scale, candidate.primary),
                label(
                    tr("theme-designer-proof-search"),
                    7.5,
                    scale,
                    with_alpha(candidate.surface_text, 0.58)
                ),
            ]
            .spacing(6.0 * scale)
            .align_y(Alignment::Center),
        )
        .width(Length::Fill)
        .padding([5.0 * scale, 6.0 * scale])
        .style(move |_| {
            crate::frontend::ui::box_style(
                candidate.surface_container,
                with_alpha(candidate.outline, 0.68),
            )
        });
        let active_nav = container(
            row![
                label("◆", 6.5, scale, candidate.primary),
                label(tr("theme-designer-proof-nav-active"), 8.0, scale, candidate.surface_text),
            ]
            .spacing(6.0 * scale)
            .align_y(Alignment::Center),
        )
        .width(Length::Fill)
        .padding([5.0 * scale, 6.0 * scale])
        .style(move |_| {
            crate::frontend::ui::box_style(
                candidate.surface_variant,
                with_alpha(candidate.primary, 0.74),
            )
        });
        let navigator = container(
            column![
                label(tr("theme-designer-proof-nav-title"), 10.0, scale, candidate.surface_text),
                search,
                active_nav,
                label(
                    tr("theme-designer-proof-nav-motion"),
                    7.5,
                    scale,
                    with_alpha(candidate.surface_text, 0.68)
                ),
                label(
                    tr("theme-designer-proof-nav-playback"),
                    7.5,
                    scale,
                    with_alpha(candidate.surface_text, 0.68)
                ),
                label(
                    tr("theme-designer-proof-nav-theme"),
                    7.5,
                    scale,
                    with_alpha(candidate.surface_text, 0.68)
                ),
                container(text("")).height(Length::Fill),
                label(
                    tr("theme-designer-proof-nav-footer"),
                    6.5,
                    scale,
                    with_alpha(candidate.surface_text, 0.4)
                ),
            ]
            .spacing(8.0 * scale),
        )
        .width(Length::Fixed(126.0 * scale))
        .height(Length::Fill)
        .padding(9.0 * scale)
        .style(move |_| {
            crate::frontend::ui::box_style(
                candidate.background,
                with_alpha(candidate.outline, 0.58),
            )
        });

        let raised_card = container(
            column![
                row![
                    label(
                        tr("theme-designer-proof-primary-title"),
                        9.5,
                        scale,
                        candidate.surface_text
                    ),
                    container(text("")).width(Length::Fill),
                    active_chip,
                ]
                .align_y(Alignment::Center),
                label(
                    tr("theme-designer-proof-primary-desc"),
                    7.5,
                    scale,
                    with_alpha(candidate.surface_text, 0.58),
                ),
            ]
            .spacing(6.0 * scale),
        )
        .padding(9.0 * scale)
        .width(Length::FillPortion(1))
        .style(move |_| {
            crate::frontend::ui::box_style(
                candidate.surface_container,
                with_alpha(candidate.outline, 0.66),
            )
        });
        let alternate_card = container(
            column![
                row![
                    label(
                        tr("theme-designer-proof-alternate-title"),
                        9.5,
                        scale,
                        candidate.surface_text
                    ),
                    container(text("")).width(Length::Fill),
                    secondary_chip,
                ]
                .align_y(Alignment::Center),
                label(
                    tr("theme-designer-proof-alternate-desc"),
                    7.5,
                    scale,
                    with_alpha(candidate.surface_text, 0.58),
                ),
            ]
            .spacing(6.0 * scale),
        )
        .padding(9.0 * scale)
        .width(Length::FillPortion(1))
        .style(move |_| {
            crate::frontend::ui::box_style(
                candidate.surface_variant,
                with_alpha(candidate.outline, 0.66),
            )
        });
        let filter_field = container(
            row![
                label(
                    tr("theme-designer-proof-filter"),
                    8.0,
                    scale,
                    with_alpha(candidate.surface_text, 0.58)
                ),
                container(text("")).width(Length::Fill),
                label(tr("settings-nav-search-shortcut"), 7.0, scale, candidate.tertiary),
            ]
            .align_y(Alignment::Center),
        )
        .padding([6.0 * scale, 8.0 * scale])
        .width(Length::Fill)
        .style(move |_| {
            crate::frontend::ui::box_style(candidate.surface, with_alpha(candidate.outline, 0.72))
        });
        let status_bar = container(
            row![
                label("◆", 6.5, scale, candidate.tertiary),
                label(tr("theme-designer-proof-status"), 7.5, scale, candidate.surface_text,),
                container(text("")).width(Length::Fill),
                label(
                    tr("theme-designer-proof-roles"),
                    7.0,
                    scale,
                    with_alpha(candidate.surface_text, 0.5)
                ),
            ]
            .spacing(6.0 * scale)
            .align_y(Alignment::Center),
        )
        .padding([6.0 * scale, 8.0 * scale])
        .width(Length::Fill)
        .style(move |_| {
            crate::frontend::ui::box_style(
                candidate.surface_container,
                with_alpha(candidate.tertiary, 0.72),
            )
        });
        let reading = container(
            column![
                row![
                    column![
                        label(tr("theme-designer-proof-eyebrow"), 7.0, scale, candidate.primary),
                        text(tr("theme-designer-proof-title"))
                            .font(UI_FONT)
                            .size(15.0 * scale.clamp(0.9, 1.05))
                            .color(candidate.surface_text),
                    ]
                    .spacing(3.0 * scale),
                    container(text("")).width(Length::Fill),
                    label(
                        tr_args!(
                            "theme-designer-editing-role",
                            role => tr(selected_role.name).to_ascii_lowercase(),
                        ),
                        7.0,
                        scale,
                        with_alpha(candidate.surface_text, 0.48),
                    ),
                ]
                .align_y(Alignment::End),
                folio_rule(&candidate),
                row![raised_card, alternate_card].spacing(7.0 * scale),
                filter_field,
                status_bar,
            ]
            .spacing(7.0 * scale),
        )
        .height(Length::Fill)
        .width(Length::Fill)
        .padding(11.0 * scale)
        .style(move |_| crate::frontend::ui::bg_style(Background::Color(candidate.surface)));
        let miniature = container(column![
            container(
                row![
                    label("skwd-wall", 8.5, scale, candidate.surface_text),
                    container(text("")).width(Length::Fill),
                    label(
                        tr("theme-designer-proof-crumb"),
                        7.0,
                        scale,
                        with_alpha(candidate.surface_text, 0.48)
                    ),
                ]
                .spacing(7.0 * scale)
                .align_y(Alignment::Center),
            )
            .padding([6.0 * scale, 9.0 * scale])
            .style(move |_| {
                crate::frontend::ui::box_style(
                    candidate.background,
                    with_alpha(candidate.outline, 0.58),
                )
            }),
            row![navigator, reading].height(Length::Fixed(224.0 * scale)),
        ])
        .width(Length::Fill)
        .style(move |_| {
            crate::frontend::ui::box_style(candidate.surface, with_alpha(candidate.outline, 0.76))
        });

        container(
            column![
                row![
                    label("02", 9.0, scale, candidate.primary),
                    label(
                        tr("theme-designer-proof-section-title"),
                        14.0,
                        scale,
                        candidate.surface_text
                    ),
                ]
                .spacing(9.0 * scale),
                label(
                    tr("theme-designer-proof-section-desc"),
                    10.5,
                    scale,
                    with_alpha(candidate.surface_text, 0.56),
                ),
                miniature,
            ]
            .spacing(9.0 * scale),
        )
        .width(Length::FillPortion(7))
        .padding([0.0, 12.0 * scale])
        .into()
    }

    fn palette_controls<'a>(&'a self, scale: f32, pal: &'a Palette) -> Element<'a, Message> {
        let options: Vec<_> = skwd_palette::PRESETS
            .iter()
            .map(|(key, label)| PresetOption {
                key: (*key).to_string(),
                label: (*label).to_string(),
            })
            .collect();
        let selected = self
            .selected_preset()
            .and_then(|key| options.iter().find(|option| option.key == key))
            .cloned();
        let picker =
            pick_list(options, selected, |preset| Message::Theme(ThemeMsg::Preset(preset.key)))
                .placeholder(tr("theme-designer-preset-placeholder"))
                .font(UI_FONT)
                .text_size(11.0 * legible_type_scale(scale))
                .padding([7.0 * scale, 10.0 * scale])
                .width(Length::Fill)
                .style(move |_theme, _status| iced::widget::pick_list::Style {
                    text_color: pal.surface_text,
                    placeholder_color: with_alpha(pal.surface_text, 0.58),
                    handle_color: pal.primary,
                    background: Background::Color(pal.surface_container),
                    border: iced::Border {
                        color: with_alpha(pal.primary, 0.48),
                        width: 1.0,
                        radius: 0.0.into(),
                    },
                });
        let has_name = !self.name_buf.trim().is_empty();
        let actions = row![
            field(
                &self.name_buf,
                tr("theme-designer-name-placeholder"),
                |value| Message::Theme(ThemeMsg::NameInput(value)),
                Message::Theme(ThemeMsg::SaveTheme),
                Length::Fill,
                scale,
                pal,
            ),
            folio_button(
                tr("theme-designer-save"),
                has_name,
                false,
                has_name.then(|| Message::Theme(ThemeMsg::SaveTheme)),
                scale,
                pal,
            ),
            folio_button(
                tr("theme-designer-save-apply"),
                has_name,
                false,
                has_name.then(|| Message::Theme(ThemeMsg::SaveApply)),
                scale,
                pal,
            ),
        ]
        .spacing(7.0 * scale)
        .align_y(Alignment::Center);
        let mut reset = row![].spacing(7.0 * scale);
        if self.dirty() {
            reset = reset.push(folio_button(
                tr("theme-designer-reset"),
                false,
                true,
                Some(Message::Theme(ThemeMsg::Reset)),
                scale,
                pal,
            ));
        }

        let content = column![
            row![
                label("03", 9.0, scale, pal.primary),
                label(tr("theme-designer-start-title"), 14.0, scale, pal.surface_text),
            ]
            .spacing(9.0 * scale),
            row![
                picker,
                folio_button(
                    tr("theme-designer-derive"),
                    false,
                    false,
                    Some(Message::Theme(ThemeMsg::SeedGen)),
                    scale,
                    pal,
                ),
            ]
            .spacing(7.0 * scale)
            .align_y(Alignment::Center),
            folio_horizontal_rule(with_alpha(pal.outline, 0.42)),
            row![
                label("04", 9.0, scale, pal.primary),
                label(tr("theme-designer-publish-title"), 14.0, scale, pal.surface_text),
            ]
            .spacing(9.0 * scale),
            label(
                tr("theme-designer-publish-desc"),
                9.5,
                scale,
                with_alpha(pal.surface_text, 0.56),
            ),
            actions,
            reset,
        ]
        .spacing(8.0 * scale);

        container(content)
            .width(Length::FillPortion(5))
            .height(Length::Fill)
            .padding(14.0 * scale)
            .style(move |_| {
                crate::frontend::ui::box_style(
                    with_alpha(pal.surface_container, 0.84),
                    with_alpha(pal.outline, 0.48),
                )
            })
            .into()
    }

    fn saved_library<'a>(
        &'a self,
        saved: Vec<(String, Candidate)>,
        scale: f32,
        pal: &'a Palette,
    ) -> Element<'a, Message> {
        let saved_count = saved.len();
        let mut library = column![].spacing(1.0 * scale);

        if saved.is_empty() {
            library = library.push(
                container(label(
                    tr("theme-designer-saved-empty"),
                    10.0,
                    scale,
                    with_alpha(pal.surface_text, 0.48),
                ))
                .padding([16.0 * scale, 10.0 * scale]),
            );
        }

        for (index, (name, candidate)) in saved.into_iter().enumerate() {
            let active = name == self.name_buf;
            let confirming_delete = self.armed_delete() == Some(name.as_str());
            let mut colors = row![].spacing(2.0 * scale).align_y(Alignment::Center);
            for hex in &candidate.colors {
                colors = colors.push(swatch(hex, 10.0 * scale, 15.0 * scale, None));
            }

            library = library.push(
                row![
                    button(
                        row![
                            label(if active { "◆" } else { "◇" }, 8.0, scale, pal.primary),
                            label(
                                format!("{:02}", index + 1),
                                8.5,
                                scale,
                                with_alpha(pal.primary, 0.68),
                            ),
                            text(name.clone()).font(UI_FONT).size(10.5 * legible_type_scale(scale)),
                            container(text("")).width(Length::Fill),
                            colors,
                        ]
                        .spacing(8.0 * scale)
                        .align_y(Alignment::Center),
                    )
                    .width(Length::Fill)
                    .padding([6.0 * scale, 8.0 * scale])
                    .on_press(Message::Theme(ThemeMsg::LoadSaved(name.clone())))
                    .style(move |_theme, status| {
                        crate::frontend::ui::folio_button_style(active, false, pal, status)
                    }),
                    folio_button(
                        if confirming_delete { tr("theme-designer-confirm") } else { "×" },
                        confirming_delete,
                        true,
                        Some(Message::Theme(ThemeMsg::DeleteSaved(name))),
                        scale,
                        pal,
                    ),
                ]
                .spacing(5.0 * scale),
            );
        }

        let list = scrollable(library)
            .direction(iced::widget::scrollable::Direction::Vertical(
                iced::widget::scrollable::Scrollbar::new().width(3.0).scroller_width(3.0),
            ))
            .style(crate::frontend::ui::scroll_style(pal.primary))
            .height(Length::Fill);
        container(
            column![
                row![
                    row![
                        label("05", 9.0, scale, pal.primary),
                        label(tr("theme-designer-saved-title"), 14.0, scale, pal.surface_text),
                    ]
                    .spacing(9.0 * scale),
                    container(text("")).width(Length::Fill),
                    label(
                        crate::i18n::theme_designer_custom_count(saved_count),
                        9.0,
                        scale,
                        with_alpha(pal.surface_text, 0.44),
                    ),
                ]
                .align_y(Alignment::Center),
                label(
                    tr("theme-designer-saved-desc"),
                    9.5,
                    scale,
                    with_alpha(pal.surface_text, 0.56),
                ),
                list,
            ]
            .spacing(8.0 * scale),
        )
        .width(Length::FillPortion(7))
        .height(Length::Fill)
        .padding(14.0 * scale)
        .style(move |_| {
            crate::frontend::ui::box_style(
                with_alpha(pal.surface_container, 0.84),
                with_alpha(pal.outline, 0.48),
            )
        })
        .into()
    }

    fn reading_surface<'a>(
        &'a self,
        saved: Vec<(String, Candidate)>,
        scale: f32,
        pal: &'a Palette,
    ) -> Element<'a, Message> {
        let role = &ROLES[self.selected];
        let saved_count = saved.len();
        let header = column![
            row![
                column![
                    label(tr("theme-designer-eyebrow"), 9.0, scale, with_alpha(pal.primary, 0.84)),
                    text(tr_args!(
                        "theme-designer-paint-role",
                        role => tr(role.name).to_ascii_lowercase(),
                    ))
                    .font(UI_FONT)
                    .size(34.0 * scale.clamp(0.88, 1.08))
                    .line_height(iced::widget::text::LineHeight::Relative(1.0))
                    .color(pal.surface_text),
                ]
                .spacing(6.0 * scale),
                container(text("")).width(Length::Fill),
                column![
                    label(
                        tr(role.description),
                        crate::frontend::ui::TYPE_SMALL,
                        scale,
                        with_alpha(pal.surface_text, 0.56)
                    ),
                    label(
                        crate::i18n::theme_designer_stats(ROLES.len(), saved_count),
                        9.0,
                        scale,
                        with_alpha(pal.surface_text, 0.42),
                    ),
                ]
                .spacing(6.0 * scale)
                .align_x(Alignment::End),
            ]
            .align_y(Alignment::Center),
            folio_rule(pal),
        ]
        .spacing(10.0 * scale);

        let workspace = row![self.wheel_editor(scale, pal), self.folio_preview(scale)]
            .spacing(10.0 * scale)
            .align_y(Alignment::Start);
        let management =
            row![self.palette_controls(scale, pal), self.saved_library(saved, scale, pal),]
                .spacing(10.0 * scale)
                .height(Length::Fixed(330.0 * scale));
        let content =
            column![header, workspace, management].spacing(READING_SECTION_SPACING * scale);

        scrollable(
            container(content)
                .width(Length::Fill)
                .padding(crate::frontend::ui::folio_scroll_padding(23.0, 27.0, scale)),
        )
        .id(iced::widget::Id::new("theme-designer-reading"))
        .direction(iced::widget::scrollable::Direction::Vertical(
            iced::widget::scrollable::Scrollbar::new().width(3.0).scroller_width(3.0),
        ))
        .style(crate::frontend::ui::scroll_style(pal.primary))
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    pub fn view<'a>(
        &'a self,
        saved: Vec<(String, Candidate)>,
        viewport: (f32, f32),
        scale: f32,
        pal: &'a Palette,
    ) -> Element<'a, Message> {
        crate::frontend::ui::folio_sheet(
            self.masthead(scale, pal),
            self.palette_index(scale, pal),
            self.reading_surface(saved, scale, pal),
            Message::Noop,
            viewport,
            scale,
            self.ease(),
            FOLIO_INDEX_WIDTH,
            pal,
        )
    }
}
