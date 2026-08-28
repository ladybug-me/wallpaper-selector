use std::time::{Duration, Instant};

use super::{
    TICK_STALL, animation_tick_due, frame_due, record_animation_tick, rephase_animation_deadline,
    retick_due,
};

#[test]
fn frame_cap() {
    let t0 = Instant::now();
    let min = Duration::from_micros(16_667);
    assert!(frame_due(None, t0, min));
    assert!(!frame_due(Some(t0), t0 + Duration::from_millis(5), min));
    assert!(frame_due(Some(t0), t0 + Duration::from_millis(17), min));
    assert!(frame_due(Some(t0), t0 + min, min));
}

#[test]
fn animation_frame_cap() {
    let t0 = Instant::now();
    let min = Duration::from_micros(16_667);
    let mut next = None;
    let mut interval = None;
    assert!(!animation_tick_due(false, next, t0));
    assert!(animation_tick_due(true, next, t0));
    record_animation_tick(true, &mut next, &mut interval, t0, min);
    assert!(!animation_tick_due(true, next, t0 + Duration::from_millis(5)));
    assert!(animation_tick_due(true, next, t0 + Duration::from_millis(17)));
}

#[test]
fn non_divisible_refresh_rate() {
    let start = Instant::now();
    let refresh = Duration::from_nanos(1_000_000_000 / 165);
    let interval = Duration::from_nanos(1_000_000_000 / 120);
    let mut next = None;
    let mut recorded_interval = None;
    let ticks = (0..165)
        .filter(|frame| {
            let now = start + refresh * *frame;
            let due = animation_tick_due(true, next, now);
            if due {
                record_animation_tick(true, &mut next, &mut recorded_interval, now, interval);
            }
            due
        })
        .count();
    assert!((119..=121).contains(&ticks), "{ticks} ticks");
}

#[test]
fn cadence_change_rephases() {
    let now = Instant::now();
    let passive = Duration::from_millis(50);
    let direct = Duration::from_nanos(1_000_000_000 / 120);
    let last_tick = now.checked_sub(Duration::from_millis(5)).unwrap();
    let old_passive_due = now + passive;
    let accelerated = rephase_animation_deadline(
        Some(old_passive_due),
        Some(passive),
        direct,
        Some(last_tick),
        now,
    );
    assert!(accelerated <= now + direct);
    assert_eq!(accelerated, last_tick + direct);

    let old_direct_due = now + direct;
    let slowed = rephase_animation_deadline(
        Some(old_direct_due),
        Some(direct),
        passive,
        Some(last_tick),
        now,
    );
    assert_eq!(slowed, last_tick + passive);
}

#[test]
fn retick_loop_stopped() {
    let t0 = Instant::now();
    let min = Duration::from_micros(16_667);
    let soon = t0 + Duration::from_millis(5);
    assert!(retick_due(false, Some(t0), soon, min));
    assert!(!retick_due(true, Some(t0), soon, min));
    assert!(retick_due(true, Some(t0), t0 + Duration::from_millis(17), min));
}

#[test]
fn retick_stall_rescue() {
    let t0 = Instant::now();
    let big = Duration::from_secs(3600);
    assert!(!retick_due(true, Some(t0), t0 + Duration::from_millis(100), big));
    assert!(retick_due(true, Some(t0), t0 + TICK_STALL, big));
}
