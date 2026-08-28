#![cfg(test)]

use std::time::{Duration, Instant};

use super::metrics::*;

#[test]
fn ring_percentiles_full() {
    let mut ring = Ring::new();
    for val in 1..=240 {
        ring.push(val as f32);
    }
    assert_eq!(ring.percentiles([0.5, 0.95, 0.99]), [120.0, 228.0, 237.0]);
    assert_eq!(ring.percentiles([0.0, 1.0, 1.0]), [1.0, 240.0, 240.0]);
}

#[test]
fn ring_empty_zero() {
    let ring = Ring::new();
    assert_eq!(ring.percentiles([0.5, 0.95, 0.99]), [0.0, 0.0, 0.0]);
}

#[test]
fn ring_partial_fill() {
    let mut ring = Ring::new();
    ring.push(10.0);
    ring.push(30.0);
    ring.push(20.0);
    assert_eq!(ring.len, 3);
    assert_eq!(ring.percentiles([0.0, 0.5, 1.0]), [10.0, 20.0, 30.0]);
}

#[test]
fn ring_wraparound() {
    let mut ring = Ring::new();
    for val in 1..=241 {
        ring.push(val as f32);
    }
    assert_eq!(ring.len, RING);
    assert_eq!(ring.percentiles([0.0, 1.0, 1.0]), [2.0, 241.0, 241.0]);
}

#[test]
fn frame_gap_discard() {
    let mut metrics = Metrics::new();
    metrics.log_enabled = false;
    let t0 = Instant::now();
    metrics.on_frame(t0, Duration::from_millis(1), 3);
    let t1 = t0 + Duration::from_secs(2);
    metrics.on_frame(t1, Duration::from_millis(2), 3);
    assert_eq!(metrics.frame_ms.len, 0);
    assert_eq!(metrics.tick_ms.len, 2);
    let t2 = t1 + Duration::from_millis(1000);
    metrics.on_frame(t2, Duration::from_millis(1), 3);
    assert_eq!(metrics.frame_ms.len, 0);
    let t3 = t2 + Duration::from_millis(16);
    metrics.on_frame(t3, Duration::from_millis(1), 5);
    assert_eq!(metrics.frame_ms.len, 1);
    let pcts = metrics.frame_ms.percentiles([0.5, 0.5, 0.5]);
    assert!((pcts[0] - 16.0).abs() < 1.0);
    assert_eq!(metrics.frames, 4);
    assert_eq!(metrics.instances, 5);
}

#[test]
fn note_decodes() {
    let mut metrics = Metrics::new();
    metrics.note_decodes(4, 2.0, 9.0);
    metrics.note_decodes(1, 3.0, 5.0);
    assert_eq!(metrics.decoded, 5);
    assert_eq!(metrics.decode_ms_avg, 3.0);
    assert_eq!(metrics.decode_ms_max, 9.0);
    assert!(metrics.report().contains("decoded 5"));
}
