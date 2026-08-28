use iced::widget::canvas::{self, Frame, Stroke};
use iced::{Rectangle, mouse};

use crate::frontend::theme::Palette;

use super::super::{parallelogram, with_alpha};

pub struct SelectionMarks {
    pub marks: Vec<(f32, f32, f32, f32, f32)>,
    pub pal: Palette,
}

impl<Message> canvas::Program<Message> for SelectionMarks {
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
        for &(center_x, center_y, half_width, half_height, skew) in &self.marks {
            let shape = parallelogram(
                center_x - half_width,
                center_y - half_height,
                half_width * 2.0,
                half_height * 2.0,
                skew.min(half_height * 2.0 * 0.4),
            );
            frame.fill(&shape, with_alpha(self.pal.primary, 0.16));
            frame.stroke(
                &shape,
                Stroke::default().with_color(with_alpha(self.pal.primary, 0.9)).with_width(2.5),
            );
        }
        vec![frame.into_geometry()]
    }
}
