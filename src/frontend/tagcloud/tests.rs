#![cfg(test)]

use super::tagcloud::ghost_text;
use crate::domain::library::search::TagEntry;

fn entry(tag: &str, count: usize) -> TagEntry {
    TagEntry { tag: tag.into(), count, selected: false, excluded: false }
}

#[test]
fn ghost_prefix() {
    let entries = vec![entry("istanbul", 5), entry("island", 2)];
    assert_eq!(ghost_text("nature ist", &entries, "ist"), "nature istanbul");
    assert_eq!(ghost_text("Ist", &entries, "ist"), "istanbul");
}

#[test]
fn ghost_rightmost() {
    let entries = vec![entry("istanbul", 5)];
    assert_eq!(ghost_text("ist ist", &entries, "ist"), "ist istanbul");
}

#[test]
fn ghost_empty() {
    let entries = vec![entry("istanbul", 5)];
    assert_eq!(ghost_text("nature ist", &entries, ""), "");
    assert_eq!(ghost_text("xyz", &entries, "xyz"), "");
    let exact = vec![entry("ist", 5)];
    assert_eq!(ghost_text("ist", &exact, "ist"), "");
}

#[test]
fn ghost_unicode() {
    let entries = vec![entry("stone", 5)];
    let out = ghost_text("\u{1e9e}\u{1e9e}st", &entries, "st");
    assert_eq!(out, "\u{1e9e}\u{1e9e}stone");
    let turkish = ghost_text("\u{130}st", &entries, "st");
    assert_eq!(turkish, "\u{130}stone");
}
