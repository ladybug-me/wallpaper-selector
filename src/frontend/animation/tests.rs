#![cfg(test)]

use super::animation::{Approach, Spring, Timeline, Tween};
use super::{MotionProfile, MotionTier};

#[test]
fn motion_profile_tiers() {
    let motion = MotionProfile::new(100.0, 250.0, 700.0);
    assert_eq!(motion.duration_ms(MotionTier::Fast), 100.0);
    assert_eq!(motion.duration_ms(MotionTier::Standard), 250.0);
    assert_eq!(motion.duration_ms(MotionTier::Slow), 700.0);

    let mut fast = motion.spring(0.0, MotionTier::Fast);
    let mut slow = motion.spring(0.0, MotionTier::Slow);
    fast.retarget(1.0);
    slow.retarget(1.0);
    fast.tick(1.0 / 60.0);
    slow.tick(1.0 / 60.0);
    assert!(fast.x > slow.x);

    let mut authored = motion.override_tween(0.0, 1_000.0);
    authored.retarget(1.0);
    assert_eq!(authored.duration_ms(), 1_000.0);
    authored.tick(0.05);
    assert!((authored.x - 0.05).abs() < 1e-4);
}

#[test]
fn durations_use_motion_policy() {
    use std::fs;
    use std::path::Path;

    let source_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let animation_root = source_root.join("frontend/animation");
    let mut stack = vec![source_root.clone()];
    let mut offenders = Vec::new();
    let forbidden = ["Spring::for_duration_ms(", "Tween::for_duration_ms(", ".set_duration_ms("];

    while let Some(dir) = stack.pop() {
        for entry in fs::read_dir(&dir).expect("read source directory") {
            let path = entry.expect("source entry").path();
            if path.is_dir() {
                if path.file_name().is_none_or(|name| name != "tests") {
                    stack.push(path);
                }
                continue;
            }
            if path.extension().is_none_or(|extension| extension != "rs")
                || path.starts_with(&animation_root)
                || path.file_name().is_some_and(|name| name == "tests.rs")
            {
                continue;
            }
            let source = fs::read_to_string(&path).expect("read Rust source");
            for pattern in forbidden {
                for (line, text) in
                    source.lines().enumerate().filter(|(_, text)| text.contains(pattern))
                {
                    offenders.push(format!(
                        "{}:{} uses raw `{pattern}`: {}",
                        path.strip_prefix(&source_root).unwrap_or(&path).display(),
                        line + 1,
                        text.trim()
                    ));
                }
            }
        }
    }

    assert!(offenders.is_empty(), "bypass MotionProfile:\n{}", offenders.join("\n"));
}

#[test]
fn approach_settles_to_target() {
    let mut track = Approach::new(0.0, 1.0, 0.08);
    assert!(!track.settled());
    for _ in 0..200 {
        track.tick(1.0 / 60.0);
    }
    assert!(track.settled());
    assert_eq!(track.x, 1.0);
    track.toward(0.0);
    assert!(!track.settled());
    track.snap(0.5);
    assert!(track.settled());
    assert_eq!(track.x, 0.5);
}

#[test]
fn approach_matches_tween_time() {
    let motion = MotionProfile::default();
    for tier in [MotionTier::Fast, MotionTier::Standard, MotionTier::Slow] {
        let frames = (motion.duration_seconds(tier) * 60.0).ceil() as usize;
        let mut x = 0.0;
        for _ in 0..frames {
            x = motion.approach(x, 1.0, 1.0 / 60.0, tier);
        }
        assert!(x > 0.96, "{tier:?} lags: {x}");
        assert!(x < 0.995, "{tier:?} races: {x}");
    }
}

#[test]
fn retime_keeps_position() {
    let motion = MotionProfile::default();
    let mut tween = motion.tween(0.0, MotionTier::Standard);
    tween.retarget(1.0);
    tween.tick(0.1);
    let mid = tween.x;
    assert!(mid > 0.0 && mid < 1.0);
    motion.retime_tween(&mut tween, MotionTier::Slow);
    assert_eq!(tween.x, mid);
    assert_eq!(tween.target, 1.0);
    assert!((tween.duration_ms() - motion.duration_ms(MotionTier::Slow)).abs() < 0.01);
}

#[test]
fn timeline_settles_tracks() {
    let mut timeline = Timeline::new().with("open", 0.0, 1.0, 0.07).with("fade", 1.0, 1.0, 0.11);
    assert_eq!(timeline.get("open"), 0.0);
    assert_eq!(timeline.get("fade"), 1.0);
    assert_eq!(timeline.get("missing"), 0.0);
    assert!(!timeline.settled());

    timeline.restart("fade", 0.0);
    assert_eq!(timeline.get("fade"), 0.0);
    assert!(!timeline.settled());
    for _ in 0..300 {
        timeline.tick(1.0 / 60.0);
    }
    assert!(timeline.settled());
    assert_eq!(timeline.get("open"), 1.0);
    assert_eq!(timeline.get("fade"), 1.0);

    timeline.snap("fade", 0.0);
    timeline.tick(1.0 / 60.0);
    assert_eq!(timeline.get("fade"), 0.0);
    assert!(timeline.settled());

    timeline.toward("open", 0.0);
    assert!(!timeline.settled());
    assert!(timeline.ease("open") <= 1.0);
}

#[test]
fn tween_wall_clock() {
    let mut up = Tween::for_duration_ms(0.0, 1000.0);
    up.retarget(1.0);
    for _ in 0..25 {
        up.tick(0.02);
    }
    assert!((up.x - 0.5).abs() < 1e-4);
    assert!(!up.settled());
    for _ in 0..25 {
        up.tick(0.02);
    }
    assert_eq!(up.x, 1.0);
    assert!(up.settled());
    up.snap(0.4);
    assert!(up.settled());
    let mut down = Tween::for_duration_ms(1.0, 500.0);
    down.retarget(0.0);
    down.tick(0.05);
    assert!((down.x - 0.9).abs() < 1e-4);
}

#[test]
fn spring_no_overshoot() {
    let mut spring = Spring::for_duration_ms(0.0, 300.0);
    spring.retarget(1.0);
    let mut max_x = 0.0f32;
    let mut settled_ms = None;
    for frame in 1..=64 {
        spring.tick(0.016);
        max_x = max_x.max(spring.x);
        if spring.settled() {
            settled_ms = Some(frame as f32 * 16.0);
            break;
        }
    }
    let ms = settled_ms.expect("spring never settled");
    assert!(ms <= 450.0);
    assert!(max_x <= 1.001);
    assert_eq!(spring.x, 1.0);
    assert_eq!(spring.v, 0.0);
}

#[test]
fn tick_clamps_dt() {
    let mut spring = Spring::for_duration_ms(0.0, 300.0);
    spring.retarget(1.0);
    let mut twin = spring;
    spring.tick(10.0);
    twin.tick(0.05);
    assert!(spring.x.is_finite() && spring.v.is_finite());
    assert_eq!(spring.x, twin.x);
    assert_eq!(spring.v, twin.v);
}

#[test]
fn hostile_dt() {
    for dt in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY, -1.0, 1e30, 1e-40, 0.0] {
        let mut spring = Spring::for_duration_ms(0.0, 300.0);
        spring.retarget(1.0);
        spring.tick(dt);
        assert!(spring.x.is_finite(), "dt={dt:e}");
        assert!(spring.v.is_finite(), "dt={dt:e}");
    }
}

#[test]
fn hitch_stability() {
    let mut spring = Spring::for_duration_ms(0.0, 200.0);
    spring.retarget(1.0);
    for _ in 0..100 {
        spring.tick(0.05);
    }
    assert_eq!(spring.x, 1.0);
    assert!(spring.settled());
}

#[test]
fn snap_moves_and_stops() {
    let mut spring = Spring::for_duration_ms(0.0, 300.0);
    spring.retarget(1.0);
    spring.tick(0.016);
    assert!(spring.v != 0.0);
    spring.snap(0.25);
    assert_eq!((spring.x, spring.v, spring.target), (0.25, 0.0, 0.25));
    assert!(spring.settled());
    spring.tick(0.016);
    assert_eq!(spring.x, 0.25);
}

#[test]
fn settle_threshold() {
    let mut near = Spring::for_duration_ms(0.96, 300.0);
    near.retarget(1.0);
    assert!(near.settled());
    let mut far = Spring::for_duration_ms(0.94, 300.0);
    far.retarget(1.0);
    assert!(!far.settled());
}

#[test]
fn underdamped_overshoot() {
    let mut spring = Spring::for_duration_ms(0.0, 300.0);
    spring.set_zeta(0.5);
    spring.retarget(1.0);
    let mut max_x = 0.0f32;
    let mut settled = false;
    for _ in 0..200 {
        spring.tick(0.016);
        max_x = max_x.max(spring.x);
        if spring.settled() {
            settled = true;
            break;
        }
    }
    assert!(max_x > 1.05);
    assert!(settled);
    assert_eq!(spring.x, 1.0);
}

#[test]
fn tween_tick_motion() {
    let mut tween = Tween::for_duration_ms(0.0, 40.0);
    tween.retarget(1.0);
    assert!(tween.tick(0.016));
    assert!(!tween.tick(0.05));
    assert!(!tween.tick(0.016));
}

#[test]
fn approach_framerate() {
    let one = super::animation::approach(0.0, 1.0, 0.032, 0.07);
    let mut split = super::animation::approach(0.0, 1.0, 0.016, 0.07);
    split = super::animation::approach(split, 1.0, 0.016, 0.07);
    assert!((one - split).abs() < 1e-5);
    assert_eq!(super::animation::approach(0.5, 0.5, 0.016, 0.07), 0.5);
}

#[test]
fn window_smoothstep() {
    assert_eq!(super::animation::window(0.10, 0.10, 0.35), 0.0);
    assert_eq!(super::animation::window(0.35, 0.10, 0.35), 1.0);
    assert_eq!(super::animation::window(0.05, 0.10, 0.35), 0.0);
    assert_eq!(super::animation::window(0.99, 0.10, 0.35), 1.0);
    let mid = super::animation::window(0.225, 0.10, 0.35);
    assert!((mid - 0.5).abs() < 1e-6);
}

#[cfg(feature = "obs-heap")]
#[test]
fn anim_ticks_alloc_free() {
    use crate::infrastructure::observability::allocation as countalloc;
    let mut spring = Spring::for_duration_ms(0.0, 300.0);
    spring.retarget(1.0);
    let mut tween = Tween::for_duration_ms(0.0, 300.0);
    tween.retarget(1.0);
    spring.tick(0.016);
    tween.tick(0.016);
    let (allocs, frees, live) =
        (countalloc::alloc_count(), countalloc::free_count(), countalloc::live_bytes());
    for _ in 0..4096 {
        spring.tick(0.016);
        tween.tick(0.016);
        spring.retarget(spring.x + 0.001);
    }
    assert_eq!(countalloc::alloc_count() - allocs, 0);
    assert_eq!(countalloc::free_count() - frees, 0);
    assert_eq!(countalloc::live_bytes(), live);
}
