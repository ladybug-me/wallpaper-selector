use std::sync::OnceLock;

static DEBUG: OnceLock<bool> = OnceLock::new();

pub fn set_debug(debug: bool) {
    let _ = DEBUG.set(debug);
}

pub fn debug() -> bool {
    DEBUG.get().copied().unwrap_or(false)
}
