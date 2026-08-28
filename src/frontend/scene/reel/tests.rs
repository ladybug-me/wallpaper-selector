#![cfg(test)]

use super::{roll_in_cut, roll_out_cut};
use crate::frontend::scene::InstanceRaw;

#[test]
fn roll_cut_tiling() {
    let mut outgoing = InstanceRaw { rect: [100.0, 120.0, 40.0, 40.0], ..Default::default() };
    roll_out_cut(&mut outgoing, 0.25);
    assert_eq!(outgoing.rect[0], 90.0);
    assert_eq!(outgoing.rect[2], 30.0);
    assert!((outgoing.rect[0] - outgoing.rect[2] - 60.0).abs() < 1e-3);
    assert!((outgoing.crop[0] - 0.25).abs() < 1e-6);
    assert!((outgoing.crop[2] - 0.75).abs() < 1e-6);

    let mut incoming = InstanceRaw { rect: [100.0, 120.0, 40.0, 40.0], ..Default::default() };
    roll_in_cut(&mut incoming, 0.25);
    assert_eq!(incoming.rect[0], 130.0);
    assert_eq!(incoming.rect[2], 10.0);
    assert!((incoming.rect[0] + incoming.rect[2] - 140.0).abs() < 1e-3);
    assert_eq!(incoming.crop[0], 0.0);
    assert!((incoming.crop[2] - 0.25).abs() < 1e-6);
    assert!(
        (outgoing.rect[0] + outgoing.rect[2] - (incoming.rect[0] - incoming.rect[2])).abs() < 1e-3
    );

    let mut complete = InstanceRaw { rect: [100.0, 120.0, 40.0, 40.0], ..Default::default() };
    roll_in_cut(&mut complete, 1.0);
    assert_eq!(complete.rect, [100.0, 120.0, 40.0, 40.0]);
}
