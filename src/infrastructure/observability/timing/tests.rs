#![cfg(test)]

use super::timing::*;

#[test]
fn checkpoint_once() {
    assert!(checkpoint_is_new("probe_alpha"));
    assert!(!checkpoint_is_new("probe_alpha"));
    assert!(!checkpoint_is_new("probe_alpha"));
    assert!(checkpoint_is_new("probe_beta"));

    for _ in 0..50 {
        assert!(!checkpoint_is_new("probe_alpha"));
    }
}

#[test]
fn line_after_init() {
    let cache = std::env::temp_dir().join(format!("skwd-timing-test-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&cache);
    let log = cache.join("wallpaper").join("selector-timing.log");
    std::fs::create_dir_all(log.parent().unwrap()).unwrap();
    append_line(&log, "cold start");
    append_line(&log, "gpu ready");
    let text = std::fs::read_to_string(&log).expect("init + line must create the log");
    let lines: Vec<&str> = text.lines().collect();
    assert_eq!(lines.len(), 2);
    for (entry, msg) in lines.iter().zip(["cold start", "gpu ready"]) {
        let (epoch, rest) = entry.split_once(' ').expect("epoch prefix");
        assert!(epoch.parse::<u128>().is_ok());
        assert!(rest.starts_with(&format!("wallpaper-selector timing: {msg} (rss: ")));
        assert!(rest.contains(" MB, pss: ") && rest.contains(" MB, vss: "));
        assert!(rest.ends_with(" MB) [rust]"));
    }
    let _ = std::fs::remove_dir_all(&cache);
}

#[cfg(target_os = "linux")]
#[test]
fn parse_kb_fields() {
    assert_eq!(parse_kb("     1234 kB"), 1234.0);
    assert_eq!(parse_kb("0 kB"), 0.0);
    assert_eq!(parse_kb(" garbage "), 0.0);
}

#[cfg(target_os = "linux")]
#[test]
fn self_mem_positive() {
    let (rss, _pss, vss) = self_mem_mb();
    assert!(rss > 0.0);
    assert!(vss >= rss);
}
