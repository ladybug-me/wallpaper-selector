#![cfg(test)]

use super::*;
use iced::widget::canvas::Program;
use iced::{Point, Size, mouse};

fn wheel_event() -> Event {
    Event::Mouse(mouse::Event::WheelScrolled {
        delta: mouse::ScrollDelta::Lines { x: 0.0, y: -1.0 },
    })
}

fn over_pane() -> (Rectangle, mouse::Cursor) {
    let bounds = Rectangle::new(Point::ORIGIN, Size::new(400.0, 300.0));
    (bounds, mouse::Cursor::Available(Point::new(200.0, 150.0)))
}

#[test]
fn wheel_captures_enabled() {
    let _wheel = crate::frontend::components::wheel_test_guard();
    let (bounds, cursor) = over_pane();
    let armed = PaneWheel { key: "settings", enabled: true, on_wheel: |key, delta| (key, delta) };
    let mut state = ();
    assert!(armed.update(&mut state, &wheel_event(), bounds, cursor).is_some());
}

#[test]
fn wheel_passes_disabled() {
    let _wheel = crate::frontend::components::wheel_test_guard();
    let (bounds, cursor) = over_pane();
    let disarmed =
        PaneWheel { key: "settings", enabled: false, on_wheel: |key, delta| (key, delta) };
    let mut state = ();
    assert!(disarmed.update(&mut state, &wheel_event(), bounds, cursor).is_none());
}
