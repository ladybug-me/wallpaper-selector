use iced::widget::canvas::{self, Frame};
use iced::{Alignment, Color, Point, Rectangle, Vector, mouse};

use super::typography::{NERD_FONT, mid_text};

pub struct Spinner {
    pub angle: f32,
    pub color: Color,
    pub size: f32,
}

impl<Message> canvas::Program<Message> for Spinner {
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
        let center = Point::new(bounds.width / 2.0, bounds.height / 2.0);
        frame.translate(Vector::new(center.x, center.y));
        frame.rotate(self.angle);
        frame.fill_text(mid_text(
            "\u{f051f}".to_string(),
            Point::new(0.0, 0.0),
            self.color,
            self.size,
            NERD_FONT,
            Alignment::Center,
        ));
        vec![frame.into_geometry()]
    }
}
