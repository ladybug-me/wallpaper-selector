use std::sync::atomic::{AtomicU32, Ordering};

static SURFACE_SCALE: AtomicU32 = AtomicU32::new(0);

pub fn set_surface_scale(scale: f32) {
    SURFACE_SCALE.store(scale.to_bits(), Ordering::Relaxed);
}

pub fn surface_scale() -> f32 {
    let scale = f32::from_bits(SURFACE_SCALE.load(Ordering::Relaxed));
    if scale.is_finite() && scale >= 0.25 { scale } else { 1.0 }
}
