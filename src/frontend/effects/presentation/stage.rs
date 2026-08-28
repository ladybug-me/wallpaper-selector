use iced::widget::{column, container, image, row, stack, text};
use iced::{Alignment, ContentFit, Element, Length, Padding};

use crate::app::Message;
use crate::frontend::theme::Palette;
use crate::frontend::ui::{UI_FONT, folio_horizontal_rule, label, with_alpha};
use crate::i18n::tr;

use super::super::state::{Effects, EffectsMsg};
use super::super::{PreviewRenderer, PreviewRequest};

impl Effects {
    pub(super) fn editor_overlay<'a>(
        &'a self,
        scale: f32,
        palette: &'a Palette,
    ) -> Element<'a, Message> {
        let definition = self.selected();
        let title = definition.map_or_else(|| self.source_stem(), |effect| effect.label.clone());
        let description = definition.map_or_else(String::new, |effect| effect.description.clone());
        let saved = self.selected_is_saved();

        let actions = row![
            crate::frontend::ui::folio_action(
                tr(if saved { "effects-remove" } else { "effects-save" }),
                saved,
                self.has_effects_page().then_some(Message::Effects(EffectsMsg::ToggleSaved)),
                Length::Fixed(112.0 * scale.max(0.95)),
                scale,
                palette,
            ),
            crate::frontend::ui::folio_action(
                tr("effects-apply"),
                true,
                self.has_saved_effects().then_some(Message::Effects(EffectsMsg::Apply)),
                Length::Fixed(106.0 * scale.max(0.95)),
                scale,
                palette,
            ),
            crate::frontend::ui::folio_action(
                "×",
                false,
                Some(Message::ToggleEffects),
                Length::Shrink,
                scale,
                palette,
            ),
        ]
        .spacing(7.0 * scale)
        .align_y(Alignment::Center);

        let title_block = column![
            text(title)
                .font(UI_FONT)
                .size(52.0 * scale.clamp(0.88, 1.05))
                .line_height(iced::widget::text::LineHeight::Relative(0.96))
                .color(palette.surface_text),
            label(description, 11.0, scale, with_alpha(palette.surface_text, 0.72))
                .line_height(iced::widget::text::LineHeight::Relative(1.35)),
        ]
        .spacing(8.0 * scale)
        .max_width(620.0 * scale.max(0.9));

        container(
            column![
                row![title_block, container(text("")).width(Length::Fill), actions,]
                    .align_y(Alignment::Start),
                container(text("")).height(Length::Fill),
                folio_horizontal_rule(with_alpha(palette.outline, 0.5)),
                self.parameter_sheet(scale, palette),
            ]
            .spacing(14.0 * scale),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(Padding {
            top: 28.0 * scale,
            right: 26.0 * scale,
            bottom: 24.0 * scale,
            left: 28.0 * scale,
        })
        .into()
    }

    pub(super) fn immersive_preview<'a>(
        &'a self,
        palette: &'a Palette,
        preview_renderer: &impl PreviewRenderer<Message, Output = Element<'static, Message>>,
    ) -> Element<'a, Message> {
        if let (Some(shader), Some(rgba)) = (self.shader_effect(), &self.preview.rgba) {
            let widget = preview_renderer.view(PreviewRequest {
                rgba: rgba.clone(),
                width: self.preview.width,
                height: self.preview.height,
                version: self.preview.version,
                shader,
            });
            return container(widget).width(Length::Fill).height(Length::Fill).into();
        }
        let display = if self.preview_effects().is_empty() {
            self.display_source()
        } else {
            self.preview.rendered.clone().unwrap_or_else(|| self.display_source())
        };
        let image_at = |path: &str, opacity: f32| {
            image(image::Handle::from_path(path))
                .content_fit(ContentFit::Cover)
                .width(Length::Fill)
                .height(Length::Fill)
                .opacity(opacity)
        };
        let inner: Element<Message> = if display.is_empty() {
            container(
                text(tr("effects-rendering-preview"))
                    .font(UI_FONT)
                    .size(12.0)
                    .color(with_alpha(palette.surface_text, 0.52)),
            )
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
        } else if self.panel.animation.get("fade") < 0.999
            && self
                .preview
                .fade_from
                .as_deref()
                .is_some_and(|path| !path.is_empty() && path != display)
        {
            let from = self.preview.fade_from.clone().unwrap_or_default();
            stack![image_at(&from, 1.0), image_at(&display, self.fade_ease())].into()
        } else {
            image_at(&display, 1.0).into()
        };
        container(inner).width(Length::Fill).height(Length::Fill).into()
    }

    pub(super) fn source_stem(&self) -> String {
        let path = if self.preview.path.is_empty() {
            self.preview.thumb.as_deref().unwrap_or("")
        } else {
            self.preview.path.as_str()
        };
        std::path::Path::new(path)
            .file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or(tr("effects-wallpaper"))
            .to_string()
    }
}
