use std::time::{Duration, Instant};

use futures_channel::mpsc::unbounded;

use super::FrameClock;
use crate::infrastructure::runtime::Wake;

#[test]
fn idle_clock_silent() {
    let (wake, mut receiver) = unbounded();
    let clock = FrameClock::start(wake);
    assert!(receiver.try_recv().is_err());

    clock.schedule(Some(Instant::now() + Duration::from_millis(10)));
    let limit = Instant::now() + Duration::from_secs(1);
    loop {
        if matches!(receiver.try_recv(), Ok(Wake::Frame(_))) {
            break;
        }
        assert!(Instant::now() < limit);
        std::thread::yield_now();
    }
    std::thread::sleep(Duration::from_millis(20));
    assert!(receiver.try_recv().is_err());
}

#[test]
fn deadline_waits_for_redraw() {
    let (wake, mut receiver) = unbounded();
    let clock = FrameClock::start(wake);
    clock.schedule(Some(Instant::now()));
    let limit = Instant::now() + Duration::from_secs(1);
    while !matches!(receiver.try_recv(), Ok(Wake::Frame(_))) {
        assert!(Instant::now() < limit);
        std::thread::yield_now();
    }

    clock.schedule(Some(Instant::now()));
    let blocked_until = Instant::now() + Duration::from_millis(50);
    while Instant::now() < blocked_until {
        assert!(!matches!(receiver.try_recv(), Ok(Wake::Frame(_))));
        std::thread::sleep(Duration::from_millis(2));
    }

    clock.acknowledge_redraw();
    let limit = Instant::now() + Duration::from_secs(1);
    while !matches!(receiver.try_recv(), Ok(Wake::Frame(_))) {
        assert!(Instant::now() < limit);
        std::thread::yield_now();
    }
}

#[test]
fn newer_schedule_replaces_pending_deadline() {
    let (wake, mut receiver) = unbounded();
    let clock = FrameClock::start(wake);
    clock.schedule(Some(Instant::now() + Duration::from_millis(200)));
    clock.schedule(None);
    let limit = Instant::now() + Duration::from_millis(400);
    while Instant::now() < limit {
        assert!(!matches!(receiver.try_recv(), Ok(Wake::Frame(_))));
        std::thread::sleep(Duration::from_millis(2));
    }
}
