#![cfg(test)]

use std::sync::{Arc, Mutex};

use super::{PREVIEW_H, PREVIEW_W, PreviewFrame, TransitionPreviewWorker, publish_frame};

#[test]
fn stop_clears_key() {
    let mut worker = TransitionPreviewWorker::default();
    worker.key = String::from("sand-helix\u{0}3000");
    worker.stop();
    assert!(worker.key.is_empty());
}

#[test]
fn preview_aspect_ratio() {
    assert_eq!(PREVIEW_W * 9, PREVIEW_H * 16);
}

#[test]
fn stale_generation_rejected() {
    let latest = Arc::new(Mutex::new(PreviewFrame { generation: 7, pixels: None }));
    assert!(!publish_frame(&latest, 6, &[1, 2, 3, 4]));
    assert!(latest.lock().unwrap().pixels.is_none());
    assert!(publish_frame(&latest, 7, &[5, 6, 7, 8]));
    assert_eq!(latest.lock().unwrap().pixels.as_deref(), Some([5, 6, 7, 8].as_slice()));
}
