use super::*;

#[test]
fn source_registry_round_trip() {
    let expected = [
        ("wallhaven", "Wallhaven", true, ApplyKind::Static),
        ("steam", "Steam Workshop", true, ApplyKind::WallpaperEngine),
        ("unsplash", "Unsplash", true, ApplyKind::Static),
        ("pexels", "Pexels", true, ApplyKind::Static),
        ("youtube", "YouTube", true, ApplyKind::Video),
        ("bing", "Bing Daily", false, ApplyKind::Static),
    ];
    assert_eq!(Source::ALL.len(), expected.len());
    for (source, (key, label, searchable, kind)) in Source::ALL.into_iter().zip(expected) {
        assert_eq!(source.key(), key);
        assert_eq!(source.label(), label);
        assert_eq!(source.searchable(), searchable);
        assert_eq!(source.apply_kind(), kind);
        assert_eq!(Source::from_key(key), Some(source));
    }
    assert_eq!(Source::from_key("unknown"), None);
}
