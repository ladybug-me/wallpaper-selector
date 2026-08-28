use crate::frontend::animation::Spring;

pub fn step(idx: usize, delta: i32, len: usize) -> Option<usize> {
    if len == 0 {
        return None;
    }
    let next = (idx as i32 + delta).clamp(0, len as i32 - 1) as usize;
    (next != idx).then_some(next)
}

pub fn follow(spring: &mut Spring, item_top: f32, item_h: f32, view_h: f32, max: f32) -> bool {
    if item_top < spring.target {
        spring.retarget(item_top.max(0.0));
        true
    } else if item_top + item_h > spring.target + view_h {
        spring.retarget((item_top + item_h - view_h).clamp(0.0, max));
        true
    } else {
        false
    }
}
