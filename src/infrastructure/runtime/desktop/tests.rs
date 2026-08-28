#![cfg(test)]

use super::desktop_is_niri;

#[test]
fn niri_detect_case_insensitive() {
    assert!(desktop_is_niri("niri"));
    assert!(desktop_is_niri("NIRI"));
    assert!(desktop_is_niri("niri:GNOME"));
    assert!(!desktop_is_niri("Hyprland"));
    assert!(!desktop_is_niri(""));
}
