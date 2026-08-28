use std::sync::OnceLock;
use std::time::Instant;

use log::info;

use super::timing;

static START: OnceLock<Instant> = OnceLock::new();

pub fn init_startup_clock() {
    let _ = START.set(Instant::now());
}

pub fn elapsed_ms() -> u128 {
    START.get().map_or(0, |start| start.elapsed().as_millis())
}

pub fn log_startup_checkpoint(label: &str) {
    if !timing::checkpoint_is_new(label) {
        return;
    }
    info!("startup: {label} at {} ms, rss = {}", elapsed_ms(), rss_label());
    timing::line(&format!("{label} at {} ms", elapsed_ms()));
}

#[cfg(target_os = "linux")]
pub fn rss_label() -> String {
    let status = std::fs::read_to_string("/proc/self/status").unwrap_or_default();
    for line in status.lines() {
        if let Some(rest) = line.strip_prefix("VmRSS:") {
            let kb: u64 = rest.trim().trim_end_matches("kB").trim().parse().unwrap_or(0);
            return format!("{:.1} MB", kb as f64 / 1024.0);
        }
    }
    String::from("unknown")
}

#[cfg(not(target_os = "linux"))]
pub fn rss_label() -> String {
    String::from("unknown")
}
