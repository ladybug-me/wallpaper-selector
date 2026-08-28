pub fn init(debug: bool) {
    skwd_log::init_facade("skwd-wall", debug);
}

#[cfg(feature = "obs-trace")]
pub use super::trace_setup::{flush_chrome, init_tracing};
