use std::sync::Arc;

use iced::widget::canvas::{self, Frame};
use iced::{Rectangle, mouse};

use crate::frontend::scene::{Chrome, RenderSnapshot};
use crate::frontend::theme::Palette;

use super::back::draw_back;
use super::badges::draw_chrome_item;

pub struct ChromeCanvas<'a> {
    pub render: Arc<RenderSnapshot>,
    pub pal: &'a Palette,
    pub cache: &'a canvas::Cache,
    pub overview_set: bool,
    pub show_type_badges: bool,
    pub fade: f32,
}

impl<Message> canvas::Program<Message> for ChromeCanvas<'_> {
    type State = ();

    fn draw(
        &self,
        _state: &(),
        renderer: &iced::Renderer,
        _theme: &iced::Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let palette = self.pal;
        let fade = self.fade.clamp(0.0, 1.0);
        let geometry = self.cache.draw(renderer, bounds.size(), |frame: &mut Frame| {
            for chrome in &self.render.chrome {
                if chrome_hidden(chrome, &self.render) {
                    continue;
                }
                let faded = &Chrome { opacity: chrome.opacity * fade, ..*chrome };
                draw_chrome_item(frame, palette, faded, self.show_type_badges);
            }
            if let Some(panel) = &self.render.back {
                draw_back(frame, panel, palette, self.overview_set, fade);
            }
        });
        vec![geometry]
    }
}

fn chrome_hidden(chrome: &Chrome, render: &RenderSnapshot) -> bool {
    if chrome.opacity <= 0.02 {
        return true;
    }
    if let Some(clip) = render.clip
        && (chrome.cy + chrome.hh > clip[3] + 0.5 || chrome.cy - chrome.hh < clip[1] - 0.5)
    {
        return true;
    }
    if render.overlay
        && let Some(panel) = &render.back
        && chrome.cx >= panel.cx - panel.hw
        && chrome.cx <= panel.cx + panel.hw
        && chrome.cy >= panel.cy - panel.hh
        && chrome.cy <= panel.cy + panel.hh
    {
        return true;
    }
    false
}
