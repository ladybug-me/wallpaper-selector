#![cfg(test)]

use serde_json::json;

use super::{LibraryPaths, decode_cached};

fn decode_list(
    value: &serde_json::Value,
    paths: LibraryPaths<'_>,
) -> crate::domain::library::catalog::Catalog {
    crate::infrastructure::rpc_results::decode_library_list(value, paths)
        .expect("valid library result")
}
use crate::domain::library::catalog::WallpaperKind;

fn paths() -> LibraryPaths<'static> {
    LibraryPaths::new("/wallpapers", "/videos")
}

#[test]
fn kind_paths_decode() {
    let catalog = decode_list(
        &json!({
            "wallpapers": [
                {"name":"still.png","type":"static","thumb":"/thumb/still.webp"},
                {"name":"clip.mp4","type":"video","thumb":"/thumb/clip.webp"},
                {
                    "name":"external.mp4",
                    "type":"video",
                    "thumb":"/thumb/external.webp",
                    "video_file":"/steam/external.mp4"
                },
                {"name":"scene","type":"we","thumb":"/thumb/scene.webp","we_id":"123"}
            ]
        }),
        paths(),
    );

    assert_eq!(catalog.items.len(), 4);
    assert_eq!(catalog.items[0].kind, WallpaperKind::Static);
    assert_eq!(catalog.items[0].path, "/wallpapers/still.png");
    assert_eq!(catalog.items[1].kind, WallpaperKind::Video);
    assert_eq!(catalog.items[1].path, "/videos/clip.mp4");
    assert_eq!(catalog.items[2].path, "/steam/external.mp4");
    assert_eq!(catalog.items[3].kind, WallpaperKind::We);
    assert!(catalog.items[3].path.is_empty());
}

#[test]
fn invalid_rows_rejected() {
    let catalog = decode_list(
        &json!({
            "wallpapers": [
                {"name":"","type":"static","thumb":"/thumb/empty.webp"},
                {"name":"///","type":"static","thumb":"/thumb/slashes.webp"},
                {"name":"missing-thumb.png","type":"static"},
                {"name":"shader.wgsl","type":"shader","thumb":"/thumb/shader.webp"},
                {"name":"folder//nested///still.png/","thumb":"/thumb/still.webp"}
            ]
        }),
        paths(),
    );

    assert_eq!(catalog.items.len(), 1);
    assert_eq!(catalog.items[0].name, "folder/nested/still.png");
    assert_eq!(catalog.items[0].key, "folder/nested/still.png");
    assert_eq!(catalog.items[0].kind, WallpaperKind::Static);
    assert_eq!(catalog.items[0].path, "/wallpapers/folder/nested/still.png");
}

#[test]
fn unknown_kind_fallback() {
    let catalog = decode_list(
        &json!({
            "wallpapers": [
                {
                    "key":"",
                    "name":"future.asset",
                    "type":"future-renderer",
                    "thumb":"/thumb/future.webp"
                }
            ]
        }),
        paths(),
    );

    assert_eq!(catalog.items.len(), 1);
    assert_eq!(catalog.items[0].key, "future.asset");
    assert_eq!(catalog.items[0].kind, WallpaperKind::Static);
    assert!(catalog.items[0].path.is_empty());
}

#[test]
fn analysis_collections() {
    let catalog = decode_list(
        &json!({
            "wallpapers": [
                {
                    "key":"a",
                    "name":"a.png",
                    "thumb":"/thumb/a.webp",
                    "tags":"forest, green",
                    "colors":"{\"hue\":7}",
                    "weather":"[\"rain\",\"cloud\"]",
                    "favourite":1,
                    "mtime":42,
                    "hue":5,
                    "sat":6,
                    "richness":8,
                    "apply_count":9,
                    "last_applied":17,
                    "width":1920,
                    "height":1080,
                    "filesize":1234,
                    "duration_ms":92500
                },
                {
                    "key":"b",
                    "name":"b.png",
                    "thumb":"/thumb/b.webp",
                    "tags":"[\"green\", \" sky \", \"\"]",
                    "colors":"{}",
                    "weather":"[]"
                },
                {
                    "key":"c",
                    "name":"c.png",
                    "thumb":"/thumb/c.webp",
                    "tags":"",
                    "colors":"not-json",
                    "weather":"not-json"
                }
            ]
        }),
        paths(),
    );

    let first = &catalog.items[0];
    assert_eq!(first.mtime, 42);
    assert_eq!(first.hue, 5);
    assert_eq!(first.sat, 6);
    assert_eq!(first.richness, 8);
    assert_eq!(first.apply_count, 9);
    assert_eq!(first.last_applied, 17);
    assert_eq!((first.width, first.height, first.filesize), (1920, 1080, 1234));
    assert_eq!(first.duration_ms, 92_500);

    let second = &catalog.items[1];
    assert_eq!(second.mtime, 0);
    assert_eq!(second.hue, 99);
    assert_eq!(second.sat, 0);
    assert_eq!(second.richness, 0);
    assert_eq!(second.apply_count, 0);
    assert_eq!(second.last_applied, 0);
    assert_eq!((second.width, second.height, second.filesize), (0, 0, 0));
    assert_eq!(second.duration_ms, 0);

    assert_eq!(catalog.tags["a"], vec!["forest", "green"]);
    assert_eq!(catalog.tags["b"], vec!["green", "sky"]);
    assert!(!catalog.tags.contains_key("c"));
    assert_eq!(catalog.tag_counts.get("green"), Some(&2));
    assert_eq!(catalog.tag_counts.get("forest"), Some(&1));
    assert_eq!(catalog.tag_counts.get("sky"), Some(&1));
    assert_eq!(catalog.tag_vocab.len(), 3);
    assert_eq!(catalog.colors["a"].hue, 7);
    assert_eq!(catalog.colors["b"].hue, 99);
    assert!(!catalog.colors.contains_key("c"));
    assert_eq!(catalog.weather["a"], vec!["rain", "cloud"]);
    assert!(catalog.weather["b"].is_empty());
    assert!(!catalog.weather.contains_key("c"));
    assert!(catalog.favourites.contains("a"));
    assert!(!catalog.favourites.contains("b"));
}

#[test]
fn malformed_list_rejected() {
    assert!(crate::infrastructure::rpc_results::decode_library_list(&json!({}), paths(),).is_err());
    assert!(
        crate::infrastructure::rpc_results::decode_library_list(
            &json!({"wallpapers": "wrong"}),
            paths(),
        )
        .is_err()
    );
}

#[test]
fn cached_row_rules() {
    let wallpaper = decode_cached(
        &json!({
            "key":"we:222",
            "name":"clip",
            "type":"video",
            "thumb":"/thumb/clip.webp",
            "video_file":"/steam/clip.mp4"
        }),
        paths(),
    )
    .unwrap();
    assert_eq!(wallpaper.key, "we:222");
    assert_eq!(wallpaper.kind, WallpaperKind::Video);
    assert_eq!(wallpaper.path, "/steam/clip.mp4");

    assert!(
        decode_cached(
            &json!({"name":"shader.wgsl","type":"shader","thumb":"/thumb/shader.webp"}),
            paths()
        )
        .is_none()
    );
    assert!(decode_cached(&json!({"name":"no-thumb.png"}), paths()).is_none());
}

#[test]
fn renamed_static_path_join() {
    assert_eq!(
        paths().renamed_static_path("nested/new name.png"),
        "/wallpapers/nested/new name.png"
    );
}
