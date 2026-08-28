use std::time::{Duration, Instant};

const TICK_STALL: Duration = Duration::from_millis(300);

fn frame_due(last: Option<Instant>, now: Instant, min: Duration) -> bool {
    match last {
        Some(prev) => now.duration_since(prev) >= min,
        None => true,
    }
}

pub(crate) fn animation_tick_due(animating: bool, next: Option<Instant>, now: Instant) -> bool {
    if !animating {
        return false;
    }
    next.is_none_or(|due| now >= due)
}

pub(crate) fn record_animation_tick(
    animating: bool,
    next: &mut Option<Instant>,
    interval: &mut Option<Duration>,
    now: Instant,
    min: Duration,
) {
    if !animating {
        *next = None;
        *interval = None;
        return;
    }
    *interval = Some(min);
    let Some(due) = *next else {
        *next = now.checked_add(min);
        return;
    };
    if now < due {
        *next = now.checked_add(min);
        return;
    }
    let step_ns = min.as_nanos().max(1);
    let elapsed_steps = now.duration_since(due).as_nanos() / step_ns + 1;
    let advance = min.saturating_mul(elapsed_steps.min(u32::MAX.into()) as u32);
    *next = due.checked_add(advance).or_else(|| now.checked_add(min));
}

pub(crate) fn rephase_animation_deadline(
    next: Option<Instant>,
    previous: Option<Duration>,
    current: Duration,
    last_tick: Option<Instant>,
    now: Instant,
) -> Instant {
    if previous.is_some_and(|previous| previous != current) {
        return last_tick.and_then(|last| last.checked_add(current)).unwrap_or(now).max(now);
    }
    next.unwrap_or(now)
}

pub(crate) fn retick_due(
    loop_active: bool,
    last: Option<Instant>,
    now: Instant,
    min: Duration,
) -> bool {
    !loop_active
        || frame_due(last, now, min)
        || last.is_some_and(|prev| now.duration_since(prev) >= TICK_STALL)
}

#[cfg(test)]
mod tests;
