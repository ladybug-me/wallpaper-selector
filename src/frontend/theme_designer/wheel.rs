use iced::mouse;
use iced::widget::canvas::{self, Action, Event, Frame, Path, Stroke};
use iced::{Color, Point, Rectangle, Size};

use crate::app::Message;
use crate::domain::theme::hsv_to_rgb;
use crate::frontend::theme_designer::ThemeMsg;

const RING_FRACTION: f32 = 0.16;
const WEDGES: usize = 90;

pub struct ColorWheel {
    pub hsv: (f32, f32, f32),
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum Drag {
    #[default]
    None,
    Ring,
    Square,
}

fn hsv_color(h: f32, s: f32, v: f32) -> Color {
    let (r, g, b) = hsv_to_rgb(h, s, v);
    Color::from_rgb(r, g, b)
}

struct Geom {
    center: Point,
    outer: f32,
    inner: f32,
    square: Rectangle,
}

fn geom(bounds: Rectangle) -> Geom {
    let size = bounds.width.min(bounds.height);
    let center = Point::new(bounds.width / 2.0, bounds.height / 2.0);
    let outer = size / 2.0 - 2.0;
    let inner = outer * (1.0 - RING_FRACTION);
    let half = (inner - 6.0) / std::f32::consts::SQRT_2;
    let square =
        Rectangle { x: center.x - half, y: center.y - half, width: half * 2.0, height: half * 2.0 };
    Geom { center, outer, inner, square }
}

fn ring_hit(geo: &Geom, p: Point) -> Option<f32> {
    let dx = p.x - geo.center.x;
    let dy = p.y - geo.center.y;
    let d = (dx * dx + dy * dy).sqrt();
    if d < geo.inner * 0.92 || d > geo.outer * 1.08 {
        return None;
    }
    Some(dy.atan2(dx).to_degrees().rem_euclid(360.0))
}

fn square_sv(geo: &Geom, p: Point) -> (f32, f32) {
    let s = ((p.x - geo.square.x) / geo.square.width).clamp(0.0, 1.0);
    let v = (1.0 - (p.y - geo.square.y) / geo.square.height).clamp(0.0, 1.0);
    (s, v)
}

impl canvas::Program<Message> for ColorWheel {
    type State = Drag;

    fn update(
        &self,
        state: &mut Drag,
        event: &Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<Action<Message>> {
        let geo = geom(Rectangle { x: 0.0, y: 0.0, ..bounds });
        let local = |p: Point| Point::new(p.x - bounds.x, p.y - bounds.y);
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                let p = local(cursor.position_over(bounds)?);
                if let Some(h) = ring_hit(&geo, p) {
                    *state = Drag::Ring;
                    return Some(Action::publish(Message::Theme(ThemeMsg::Hue(h))).and_capture());
                }
                if geo.square.contains(p) {
                    *state = Drag::Square;
                    let (s, v) = square_sv(&geo, p);
                    return Some(Action::publish(Message::Theme(ThemeMsg::SV(s, v))).and_capture());
                }
                None
            }
            Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                let p = local(cursor.position()?);
                match *state {
                    Drag::Ring => {
                        let dx = p.x - geo.center.x;
                        let dy = p.y - geo.center.y;
                        let h = dy.atan2(dx).to_degrees().rem_euclid(360.0);
                        Some(Action::publish(Message::Theme(ThemeMsg::Hue(h))).and_capture())
                    }
                    Drag::Square => {
                        let (s, v) = square_sv(&geo, p);
                        Some(Action::publish(Message::Theme(ThemeMsg::SV(s, v))).and_capture())
                    }
                    Drag::None => None,
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                if *state == Drag::None {
                    return None;
                }
                *state = Drag::None;
                Some(Action::publish(Message::Theme(ThemeMsg::DragEnd)).and_capture())
            }
            _ => None,
        }
    }

    fn draw(
        &self,
        _state: &Drag,
        renderer: &iced::Renderer,
        _theme: &iced::Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let geo = geom(Rectangle { x: 0.0, y: 0.0, ..bounds });
        let mut frame = Frame::new(renderer, bounds.size());
        let step = 360.0 / WEDGES as f32;
        for idx in 0..WEDGES {
            let a0 = (idx as f32 * step).to_radians();
            let a1 = ((idx as f32 + 1.15) * step).to_radians();
            let hue = (idx as f32 + 0.5) * step;
            let path = Path::new(|builder| {
                builder.move_to(Point::new(
                    geo.center.x + geo.outer * a0.cos(),
                    geo.center.y + geo.outer * a0.sin(),
                ));
                builder.line_to(Point::new(
                    geo.center.x + geo.outer * a1.cos(),
                    geo.center.y + geo.outer * a1.sin(),
                ));
                builder.line_to(Point::new(
                    geo.center.x + geo.inner * a1.cos(),
                    geo.center.y + geo.inner * a1.sin(),
                ));
                builder.line_to(Point::new(
                    geo.center.x + geo.inner * a0.cos(),
                    geo.center.y + geo.inner * a0.sin(),
                ));
                builder.close();
            });
            frame.fill(&path, hsv_color(hue, 1.0, 1.0));
        }

        let steps = 28;
        let cell_w = geo.square.width / steps as f32;
        let cell_h = geo.square.height / steps as f32;
        for xi in 0..steps {
            for yi in 0..steps {
                let s = (xi as f32 + 0.5) / steps as f32;
                let v = 1.0 - (yi as f32 + 0.5) / steps as f32;
                frame.fill_rectangle(
                    Point::new(
                        geo.square.x + xi as f32 * cell_w,
                        geo.square.y + yi as f32 * cell_h,
                    ),
                    Size::new(cell_w + 0.6, cell_h + 0.6),
                    hsv_color(self.hsv.0, s, v),
                );
            }
        }

        let (h, s, v) = self.hsv;
        let ha = h.to_radians();
        let hr = f32::midpoint(geo.inner, geo.outer);
        let hue_marker = Point::new(geo.center.x + hr * ha.cos(), geo.center.y + hr * ha.sin());
        frame.stroke(
            &Path::circle(hue_marker, (geo.outer - geo.inner) * 0.42),
            Stroke::default().with_color(Color::WHITE).with_width(2.0),
        );
        frame.stroke(
            &Path::circle(hue_marker, (geo.outer - geo.inner) * 0.42 + 1.5),
            Stroke::default().with_color(Color::BLACK).with_width(1.0),
        );
        let sv_marker = Point::new(
            geo.square.x + s * geo.square.width,
            geo.square.y + (1.0 - v) * geo.square.height,
        );
        frame.stroke(
            &Path::circle(sv_marker, 6.0),
            Stroke::default().with_color(Color::WHITE).with_width(2.0),
        );
        frame.stroke(
            &Path::circle(sv_marker, 7.5),
            Stroke::default().with_color(Color::BLACK).with_width(1.0),
        );
        vec![frame.into_geometry()]
    }

    fn mouse_interaction(
        &self,
        state: &Drag,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> mouse::Interaction {
        if *state != Drag::None || cursor.is_over(bounds) {
            mouse::Interaction::Crosshair
        } else {
            mouse::Interaction::default()
        }
    }
}
