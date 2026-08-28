#![cfg(test)]

use super::state::TagUpdate;
use super::{LibraryState, RenameMetadata};
use crate::domain::library::catalog::{Catalog, Wallpaper, WallpaperKind};

fn static_wallpaper(key: &str, name: &str) -> Wallpaper {
    Wallpaper {
        key: key.to_string(),
        name: name.to_string(),
        kind: WallpaperKind::Static,
        ..Wallpaper::default()
    }
}

fn we_wallpaper(key: &str, name: &str, we_id: &str) -> Wallpaper {
    Wallpaper {
        key: key.to_string(),
        name: name.to_string(),
        kind: WallpaperKind::We,
        we_id: we_id.to_string(),
        ..Wallpaper::default()
    }
}

fn loaded_state() -> LibraryState {
    let catalog = Catalog {
        items: vec![static_wallpaper("a.png", "a.png"), we_wallpaper("we:111", "scene", "111")],
        ..Catalog::default()
    };
    let mut state = LibraryState::default();
    state.replace(catalog);
    state
}

#[test]
fn replace_rebuilds_dedup() {
    let mut state = loaded_state();
    assert_eq!(state.tag_revision(), 1);
    assert!(!state.insert(static_wallpaper("other", "a.png")));
    assert!(!state.insert(we_wallpaper("we:duplicate", "renamed scene", "111")));
    assert!(state.insert(static_wallpaper("b.png", "b.png")));
}

#[test]
fn replace_keeps_duplicate_rows() {
    let catalog = Catalog {
        items: vec![
            we_wallpaper("we:1", "scene one", "111"),
            we_wallpaper("we:2", "scene two", "111"),
        ],
        ..Catalog::default()
    };
    let mut state = LibraryState::default();
    state.replace(catalog);

    assert_eq!(state.catalog().items.len(), 2);
    assert!(!state.insert(we_wallpaper("we:3", "scene three", "111")));
    assert!(state.remove_by_key("we:2"));
    assert!(state.insert(we_wallpaper("we:3", "scene three", "111")));
    assert_eq!(state.catalog().items.len(), 2);
}

#[test]
fn remove_by_key_last_match() {
    let catalog = Catalog {
        items: vec![we_wallpaper("same", "first", "111"), we_wallpaper("same", "second", "222")],
        ..Catalog::default()
    };
    let mut state = LibraryState::default();
    state.replace(catalog);

    assert!(!state.remove_by_key("missing"));
    assert!(state.remove_by_key("same"));
    assert_eq!(state.catalog().items.len(), 1);
    assert_eq!(state.catalog().items[0].we_id, "111");
    assert!(state.insert(we_wallpaper("replacement", "third", "222")));
}

#[test]
fn remove_file_last_match() {
    let catalog = Catalog {
        items: vec![
            static_wallpaper("a.png", "a.png"),
            we_wallpaper("we:111", "scene", "111"),
            static_wallpaper("duplicate", "a.png"),
        ],
        ..Catalog::default()
    };
    let mut state = LibraryState::default();
    state.replace(catalog);

    state.remove_file("a.png");
    assert_eq!(state.catalog().items.len(), 2);
    assert_eq!(state.catalog().items[0].key, "a.png");
    assert!(state.insert(static_wallpaper("new", "a.png")));
}

#[test]
fn rename_file_first_match() {
    let mut state = loaded_state();
    state.rename_file(
        "a.png",
        "sub/b.png",
        "/wallpapers/sub/b.png",
        RenameMetadata { mtime: 2, filesize: 3, width: 4, height: 5 },
    );

    assert_eq!(state.catalog().items[0].key, "static:sub/b.png");
    assert_eq!(state.catalog().items[0].name, "sub/b.png");
    assert_eq!(state.catalog().items[0].path, "/wallpapers/sub/b.png");
    assert_eq!(state.catalog().items[0].filesize, 3);
    assert!(state.insert(static_wallpaper("old", "a.png")));
    assert!(!state.insert(static_wallpaper("duplicate", "sub/b.png")));

    state.rename_file(
        "missing.png",
        "reserved.png",
        "/wallpapers/reserved.png",
        RenameMetadata::default(),
    );
    assert!(!state.insert(static_wallpaper("reserved", "reserved.png")));
}

#[test]
fn remove_folder_named_rows() {
    let mut state = loaded_state();
    assert!(state.insert(static_wallpaper("f/x.png", "f/x.png")));
    assert!(state.insert(static_wallpaper("f/y.png", "f/y.png")));

    state.remove_folder(&["f/x.png".to_string(), "f/y.png".to_string()]);

    assert_eq!(state.catalog().items.len(), 2);
    assert!(state.catalog().items.iter().all(|wallpaper| !wallpaper.name.starts_with("f/")));
    assert!(state.insert(static_wallpaper("f/x.png", "f/x.png")));
}

#[test]
fn record_applied_precedence() {
    let mut state = loaded_state();

    assert_eq!(state.record_applied("a", "", ""), Some(0));
    assert_eq!(state.catalog().items[0].apply_count, 1);
    assert_eq!(state.catalog().items[0].last_applied, 1);
    assert_eq!(state.record_applied("", "111", ""), Some(1));
    assert_eq!(state.catalog().items[1].apply_count, 1);
    assert_eq!(state.catalog().items[1].last_applied, 2);
    assert_eq!(state.record_applied("", "", "a.png"), Some(0));
    assert_eq!(state.catalog().items[0].apply_count, 2);
    assert_eq!(state.catalog().items[0].last_applied, 3);
    assert_eq!(state.record_applied("we:111", "", ""), Some(1));
    assert_eq!(state.catalog().items[1].last_applied, 4);
    assert_eq!(state.record_applied("", "", ""), None);
    assert_eq!(state.record_applied("missing", "111", "a.png"), None);
    assert_eq!(state.catalog().items[1].apply_count, 2);
}

#[test]
fn tag_revision_wraps() {
    let mut state = LibraryState::default();
    state.tag_revision = u64::MAX;
    assert!(state.add_tag("a", String::from("tag")).is_some());
    assert_eq!(state.tag_revision(), 0);
}

#[test]
fn remove_tag_keeps_entry() {
    let mut catalog = Catalog::default();
    catalog.tags.insert("a".into(), vec!["forest".into(), "night".into()]);
    let mut state = LibraryState::default();
    state.replace(catalog);

    assert_eq!(
        state.remove_tag_at("a", 0),
        Some(TagUpdate { key: "a".into(), tags: vec!["night".into()] })
    );
    assert_eq!(state.catalog().tags["a"], ["night"]);
    assert_eq!(state.tag_revision(), 2);

    assert_eq!(state.remove_tag_at("a", 0), Some(TagUpdate { key: "a".into(), tags: Vec::new() }));
    assert!(state.catalog().tags.contains_key("a"));
    assert!(state.catalog().tags["a"].is_empty());
    assert_eq!(state.tag_revision(), 3);
}

#[test]
fn remove_tag_out_of_range() {
    let mut catalog = Catalog::default();
    catalog.tags.insert("a".into(), vec!["forest".into()]);
    let mut state = LibraryState::default();
    state.replace(catalog);

    assert_eq!(state.remove_tag_at("a", 1), None);
    assert_eq!(state.remove_tag_at("missing", 0), None);
    assert_eq!(state.catalog().tags["a"], ["forest"]);
    assert_eq!(state.tag_revision(), 1);
}

#[test]
fn add_tag_exact_values() {
    let mut catalog = Catalog::default();
    catalog.tags.insert("a".into(), vec!["forest".into()]);
    let mut state = LibraryState::default();
    state.replace(catalog);

    assert_eq!(state.add_tag("a", "forest".into()), None);
    assert_eq!(state.tag_revision(), 1);
    assert_eq!(
        state.add_tag("a", "Forest".into()),
        Some(TagUpdate { key: "a".into(), tags: vec!["forest".into(), "Forest".into()] })
    );
    assert_eq!(state.catalog().tags["a"], ["forest", "Forest"]);
    assert_eq!(state.tag_revision(), 2);

    assert_eq!(
        state.add_tag("", String::new()),
        Some(TagUpdate { key: String::new(), tags: vec![String::new()] })
    );
    assert_eq!(state.catalog().tags[""], [""]);
    assert_eq!(state.tag_revision(), 3);
}

#[test]
fn replace_tags_atomic() {
    let mut state = LibraryState::default();
    state.replace_tags("a", vec!["old".into()]).unwrap();
    let revision = state.tag_revision();
    let update = state.replace_tags("a", vec!["new".into(), "other".into()]).unwrap();
    assert_eq!(update.tags, ["new", "other"]);
    assert_eq!(state.tag_revision(), revision.wrapping_add(1));
    assert!(state.replace_tags("a", vec!["new".into(), "other".into()]).is_none());
}

#[test]
fn mass_tags_skip_invalid() {
    let mut catalog = Catalog {
        items: vec![
            static_wallpaper("a", "a.png"),
            static_wallpaper("b", "b.png"),
            static_wallpaper("a", "duplicate-a.png"),
        ],
        ..Catalog::default()
    };
    catalog.tags.insert("a".into(), vec!["forest".into()]);
    catalog.tag_vocab.insert("snapshot-only".into());
    catalog.tag_counts.insert("snapshot-only".into(), 7);
    let mut state = LibraryState::default();
    state.replace(catalog);

    let updates = state.add_tags_to_indices(
        &[0, u32::MAX, 1, 2],
        &["forest".into(), "night".into(), "night".into()],
    );

    assert_eq!(
        updates,
        vec![
            TagUpdate { key: "a".into(), tags: vec!["forest".into(), "night".into()] },
            TagUpdate { key: "b".into(), tags: vec!["forest".into(), "night".into()] },
        ]
    );
    assert_eq!(state.catalog().tags["a"], ["forest", "night"]);
    assert_eq!(state.catalog().tags["b"], ["forest", "night"]);
    assert_eq!(state.tag_revision(), 2);
    assert_eq!(state.catalog().tag_vocab.len(), 1);
    assert!(state.catalog().tag_vocab.contains("snapshot-only"));
    assert_eq!(state.catalog().tag_counts.get("snapshot-only"), Some(&7));

    assert!(state.add_tags_to_indices(&[0, 1], &["forest".into(), "night".into()]).is_empty());
    assert!(state.add_tags_to_indices(&[99], &["new".into()]).is_empty());
    assert!(state.add_tags_to_indices(&[0], &[]).is_empty());
    assert_eq!(state.tag_revision(), 2);
}

#[test]
fn toggle_favourite_any_key() {
    let mut state = loaded_state();

    assert!(state.toggle_favourite("unknown"));
    assert!(state.catalog().favourites.contains("unknown"));
    assert!(!state.toggle_favourite("unknown"));
    assert!(!state.catalog().favourites.contains("unknown"));
    assert!(state.toggle_favourite(""));
    assert!(state.catalog().favourites.contains(""));
    assert_eq!(state.tag_revision(), 1);
}
