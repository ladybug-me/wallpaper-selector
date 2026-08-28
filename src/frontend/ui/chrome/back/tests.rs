#![cfg(test)]

use iced::{Point, Rectangle, Size};

use super::background::masthead_divider;

#[test]
fn masthead_divider_embedded() {
    let masthead = Rectangle::new(Point::new(190.0, 105.0), Size::new(900.0, 32.0));
    assert_eq!(masthead_divider(masthead, true), None);
    let (from, to) = masthead_divider(masthead, false).expect("divider");
    assert_eq!(from, Point::new(190.0, 137.0));
    assert_eq!(to, Point::new(1090.0, 137.0));
}
