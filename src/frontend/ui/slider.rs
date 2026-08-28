use iced::widget::canvas::{self, Action, Frame, Path, Stroke};
use iced::{Element, Event, Point, Rectangle, mouse};

use crate::frontend::theme::Palette;

use super::with_alpha;

const FOLIO_SLIDER_INSET: f32 = 7.0;

fn slider_fraction(min: f64, max: f64, value: f64) -> f32 {
    if max <= min { 0.0 } else { (((value - min) / (max - min)) as f32).clamp(0.0, 1.0) }
}

fn marker_snap(raw_x: f32) -> f32 {
    (raw_x * 2.0).round() * 0.5
}

fn slider_value(min: f64, max: f64, step: f64, x_local: f32, w: f32) -> f64 {
    let frac = (x_local / w.max(1.0)).clamp(0.0, 1.0) as f64;
    let raw = min + frac * (max - min);
    let snapped = if step > 0.0 { (raw / step).round() * step } else { raw };
    snapped.clamp(min, max)
}

pub struct FolioSlider<Message> {
    min: f64,
    max: f64,
    value: f64,
    step: f64,
    pal: Palette,
    on_change: Box<dyn Fn(f64) -> Message>,
    on_release: Message,
}

impl<Message> FolioSlider<Message> {
    fn value_at(&self, x: f32, width: f32) -> f64 {
        slider_value(
            self.min,
            self.max,
            self.step,
            x - FOLIO_SLIDER_INSET,
            width - FOLIO_SLIDER_INSET * 2.0,
        )
    }
}

impl<Message: Clone> canvas::Program<Message> for FolioSlider<Message> {
    type State = bool;

    fn update(
        &self,
        dragging: &mut bool,
        event: &Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<Action<Message>> {
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                if let Some(point) = cursor.position_in(bounds) {
                    *dragging = true;
                    let value = self.value_at(point.x, bounds.width);
                    return Some(Action::publish((self.on_change)(value)).and_capture());
                }
                None
            }
            Event::Mouse(mouse::Event::CursorMoved { .. }) if *dragging => {
                let x = cursor.position().map_or(0.0, |point| point.x - bounds.x);
                let value = self.value_at(x, bounds.width);
                Some(Action::publish((self.on_change)(value)).and_capture())
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) if *dragging => {
                *dragging = false;
                Some(Action::publish(self.on_release.clone()).and_capture())
            }
            _ => None,
        }
    }

    fn draw(
        &self,
        _state: &bool,
        renderer: &iced::Renderer,
        _theme: &iced::Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        let inset = FOLIO_SLIDER_INSET;
        let start = Point::new(inset, bounds.height * 0.5);
        let end = Point::new((bounds.width - inset).max(inset), start.y);
        let raw_x = start.x + (end.x - start.x) * slider_fraction(self.min, self.max, self.value);
        let x = marker_snap(raw_x);

        frame.stroke(
            &Path::line(start, end),
            Stroke::default().with_color(with_alpha(self.pal.outline, 0.5)).with_width(1.0),
        );
        if x > start.x {
            frame.stroke(
                &Path::line(start, Point::new(x, start.y)),
                Stroke::default().with_color(self.pal.primary).with_width(2.0),
            );
        }

        let radius = 5.0;
        let marker = Path::new(|builder| {
            builder.move_to(Point::new(x, start.y - radius));
            builder.line_to(Point::new(x + radius, start.y));
            builder.line_to(Point::new(x, start.y + radius));
            builder.line_to(Point::new(x - radius, start.y));
            builder.close();
        });
        frame.fill(&marker, self.pal.surface);
        frame.stroke(&marker, Stroke::default().with_color(self.pal.primary).with_width(1.0));
        vec![frame.into_geometry()]
    }
}

pub fn folio_slider<'a, Message: Clone + 'a>(
    min: f64,
    max: f64,
    value: f64,
    step: f64,
    on_change: impl Fn(f64) -> Message + 'static,
    on_release: Message,
    pal: &Palette,
) -> Element<'a, Message> {
    iced::widget::canvas(FolioSlider {
        min,
        max,
        value,
        step,
        pal: *pal,
        on_change: Box::new(on_change),
        on_release,
    })
    .width(iced::Length::Fill)
    .height(iced::Length::Fixed(26.0))
    .into()
}

mod tests;
