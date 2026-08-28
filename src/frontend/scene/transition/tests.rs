#![cfg(test)]

use super::*;

#[test]
fn launch_anim_key_mapping() {
    assert_eq!(LaunchAnim::from_key("none"), LaunchAnim::None);
    assert_eq!(LaunchAnim::from_key("rise"), LaunchAnim::Rise);
    assert_eq!(LaunchAnim::from_key("zoom"), LaunchAnim::Zoom);
    assert_eq!(LaunchAnim::from_key("fade"), LaunchAnim::Fade);
    assert_eq!(LaunchAnim::from_key("garbage"), LaunchAnim::Fade);
}

#[test]
fn entrance_motion_shapes() {
    let card = InstanceRaw { rect: [100.0, 200.0, 40.0, 30.0], ..Default::default() };

    let mut insts = [card];
    apply_entrance_motion(&mut insts, LaunchAnim::Rise, 0.0, 1920.0, 1080.0);
    assert!(insts[0].rect[1] > 200.0);

    let mut insts = [card];
    apply_entrance_motion(&mut insts, LaunchAnim::Zoom, 0.0, 1920.0, 1080.0);
    assert!(insts[0].rect[2] < 40.0);

    for anim in [LaunchAnim::None, LaunchAnim::Fade] {
        let mut insts = [card];
        apply_entrance_motion(&mut insts, anim, 0.0, 1920.0, 1080.0);
        assert_eq!(insts[0].rect, card.rect);
    }

    let mut insts = [card];
    apply_entrance_motion(&mut insts, LaunchAnim::Zoom, 1.0, 1920.0, 1080.0);
    assert_eq!(insts[0].rect, card.rect);
}
