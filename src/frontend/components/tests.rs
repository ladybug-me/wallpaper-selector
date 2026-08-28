#![cfg(test)]

use std::sync::atomic::AtomicU64;

use iced::Color;
use iced::mouse::ScrollDelta;

use super::components::*;

#[test]
fn with_alpha_only() {
    let base = Color::from_rgb(0.2, 0.4, 0.6);
    let faded = with_alpha(base, 0.3);
    assert_eq!((faded.r, faded.g, faded.b), (base.r, base.g, base.b));
    assert_eq!(faded.a, 0.3);
}

fn lines_y(delta: Option<ScrollDelta>) -> f32 {
    match delta.expect("wheel delta") {
        ScrollDelta::Lines { y, .. } => y,
        ScrollDelta::Pixels { .. } => panic!("expected Lines"),
    }
}

fn pixels_y(delta: Option<ScrollDelta>) -> f32 {
    match delta.expect("wheel delta") {
        ScrollDelta::Pixels { y, .. } => y,
        ScrollDelta::Lines { .. } => panic!("expected Pixels"),
    }
}

fn fresh(delta: &ScrollDelta) -> Option<ScrollDelta> {
    norm_wheel_tracked(delta, 10_000, &AtomicU64::new(0))
}

#[test]
fn norm_wheel_scale() {
    let _wheel = wheel_test_guard();
    assert_eq!(surface_scale(), 1.0);
    assert_eq!(lines_y(fresh(&ScrollDelta::Lines { x: 0.0, y: 1.0 })), 1.0);
    assert_eq!(lines_y(fresh(&ScrollDelta::Lines { x: 0.0, y: -2.0 })), -2.0);

    set_surface_scale(1.5);
    assert_eq!(surface_scale(), 1.5);
    assert_eq!(lines_y(fresh(&ScrollDelta::Lines { x: 0.0, y: 1.5 })), 1.0);
    assert_eq!(lines_y(fresh(&ScrollDelta::Lines { x: 0.0, y: -1.5 })), -1.0);
    assert_eq!(lines_y(fresh(&ScrollDelta::Lines { x: 0.0, y: 3.0 })), 2.0);
    assert_eq!(lines_y(fresh(&ScrollDelta::Lines { x: 0.0, y: 0.0 })), 0.0);
    assert_eq!(pixels_y(fresh(&ScrollDelta::Pixels { x: 0.0, y: 90.0 })), 60.0);

    set_surface_scale(2.0);
    assert_eq!(lines_y(fresh(&ScrollDelta::Lines { x: 0.0, y: 2.0 })), 1.0);
    let (x, y) = match fresh(&ScrollDelta::Lines { x: 4.0, y: 0.0 }).unwrap() {
        ScrollDelta::Lines { x, y } => (x, y),
        ScrollDelta::Pixels { .. } => panic!("expected Lines"),
    };
    assert_eq!((x, y), (2.0, 0.0));

    set_surface_scale(1.25);
    assert_eq!(lines_y(fresh(&ScrollDelta::Lines { x: 0.0, y: 1.25 })), 1.0);
    assert_eq!(lines_y(fresh(&ScrollDelta::Lines { x: 0.0, y: 0.5 })), 1.0);

    set_surface_scale(0.0);
    assert_eq!(surface_scale(), 1.0);
    set_surface_scale(f32::NAN);
    assert_eq!(surface_scale(), 1.0);
    set_surface_scale(1.0);
}

#[test]
fn wheel_echo_suppressed() {
    let _wheel = wheel_test_guard();
    let last = AtomicU64::new(0);
    assert_eq!(
        pixels_y(norm_wheel_tracked(&ScrollDelta::Pixels { x: 0.0, y: -0.0 }, 1000, &last)),
        0.0
    );
    assert_eq!(
        lines_y(norm_wheel_tracked(&ScrollDelta::Lines { x: 0.0, y: -1.0 }, 1000, &last)),
        -1.0
    );
    assert_eq!(norm_wheel_tracked(&ScrollDelta::Pixels { x: 0.0, y: -15.0 }, 1000, &last), None);
    assert_eq!(norm_wheel_tracked(&ScrollDelta::Pixels { x: 0.0, y: -15.0 }, 1099, &last), None);
    assert_eq!(
        pixels_y(norm_wheel_tracked(&ScrollDelta::Pixels { x: 0.0, y: -8.0 }, 1100, &last)),
        -8.0
    );

    let touchpad_only = AtomicU64::new(0);
    assert_eq!(
        pixels_y(norm_wheel_tracked(&ScrollDelta::Pixels { x: 0.0, y: 4.0 }, 50, &touchpad_only)),
        4.0
    );
}
