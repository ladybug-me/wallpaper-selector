use iced::widget::canvas::{self, Frame, Stroke};
use iced::{Rectangle, mouse};

use crate::frontend::theme::Palette;

use super::super::{parallelogram, with_alpha};

pub const PANEL_SKEW: f32 = 14.0;

pub struct ChamferPanel {
    pub pal: Palette,
}

impl<Message> canvas::Program<Message> for ChamferPanel {
    type State = ();

    fn draw(
        &self,
        _state: &(),
        renderer: &iced::Renderer,
        _theme: &iced::Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        let card = parallelogram(0.0, 0.0, bounds.width, bounds.height, PANEL_SKEW);
        for (ring, alpha) in [(3.0, 0.05), (2.0, 0.08), (1.0, 0.12)] {
            frame.stroke(
                &card,
                Stroke::default()
                    .with_color(with_alpha(self.pal.primary, alpha))
                    .with_width(ring * 2.0 + 1.0),
            );
        }
        frame.fill(&card, with_alpha(self.pal.surface_container, 0.8));
        frame.stroke(
            &card,
            Stroke::default().with_color(with_alpha(self.pal.primary, 0.3)).with_width(1.0),
        );
        vec![frame.into_geometry()]
    }
}
