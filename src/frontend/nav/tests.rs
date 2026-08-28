#![cfg(test)]

use super::nav::{follow, step};
use crate::frontend::animation::Spring;

#[test]
fn step_clamps_dedups() {
    assert_eq!(step(0, 1, 5), Some(1));
    assert_eq!(step(4, 1, 5), None);
    assert_eq!(step(0, -1, 5), None);
    assert_eq!(step(2, -1, 5), Some(1));
    assert_eq!(step(2, 10, 5), Some(4));
    assert_eq!(step(0, 1, 0), None);
    assert_eq!(step(0, 1, 1), None);
}

#[test]
fn follow_retargets_offscreen() {
    let mut inside = Spring::for_duration_ms(0.0, 300.0);
    assert!(!follow(&mut inside, 0.0, 100.0, 500.0, 1000.0));
    assert_eq!(inside.target, 0.0);

    let mut below = Spring::for_duration_ms(0.0, 300.0);
    assert!(follow(&mut below, 900.0, 100.0, 500.0, 1000.0));
    assert_eq!(below.target, 500.0);

    let mut above = Spring::for_duration_ms(0.0, 300.0);
    above.retarget(400.0);
    assert!(follow(&mut above, 100.0, 100.0, 500.0, 1000.0));
    assert_eq!(above.target, 100.0);

    let mut clamped = Spring::for_duration_ms(0.0, 300.0);
    assert!(follow(&mut clamped, 5000.0, 100.0, 500.0, 300.0));
    assert_eq!(clamped.target, 300.0);
}
