use std::sync::atomic::{AtomicBool, Ordering};

static COMPRESSED_THUMBNAILS: AtomicBool = AtomicBool::new(true);

pub fn set_compressed_thumbnails(enabled: bool) {
    COMPRESSED_THUMBNAILS.store(enabled, Ordering::Release);
}

pub fn compressed_thumbnails() -> bool {
    COMPRESSED_THUMBNAILS.load(Ordering::Acquire)
}
