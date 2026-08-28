use serde_json::json;

use super::*;
use crate::contracts::browser::{
    ApplyKind, ApplyTarget, CatalogSearch, DownloadRequest, DownloadStatus, PreviewRequest,
    SearchRequest, Source, SteamSearch, WallhavenSearch,
};

#[test]
fn source_registry_parity() {
    let mut sources: Vec<&str> = Source::ALL.iter().map(|source| source.key()).collect();
    let mut wire: Vec<&str> = wall_proto::sources::keys().collect();
    sources.sort_unstable();
    wire.sort_unstable();
    assert_eq!(sources, wire);
    for source in Source::ALL {
        let spec = wall_proto::sources::spec(source.key()).expect("source key exists on the wire");
        assert_eq!(source.label(), spec.label);
        assert_eq!(source.searchable(), spec.searchable);
        assert_eq!(
            encode_apply(&ApplyTarget::Path {
                kind: source.apply_kind(),
                path: String::from("/wallpaper"),
            })["type"],
            spec.media.apply_kind()
        );
    }
}

#[test]
fn wallhaven_search_encoding() {
    let call = encode_search(&SearchRequest::Wallhaven(WallhavenSearch {
        query: String::from("forest cabin"),
        categories: [true, true, true],
        sorting: String::from("toplist"),
        purity: [true, false, false],
        top_range: String::from("1w"),
        atleast: String::from("3840x2160"),
        atmost: String::new(),
        resolutions: String::new(),
        ratios: String::from("16x9"),
        collection: String::new(),
        color_hue: Some(0),
        page: 3,
    }));
    assert_eq!(call.method, "wallhaven.search");
    assert_eq!(call.params["query"], "forest cabin");
    assert_eq!(call.params["categories"], "111");
    assert_eq!(call.params["purity"], "100");
    assert_eq!(call.params["order"], "desc");
    assert_eq!(call.params["topRange"], "1w");
    assert_eq!(call.params["atleast"], "3840x2160");
    assert_eq!(call.params["ratios"], "16x9");
    assert_eq!(call.params["colors"], "cc0000");
    assert_eq!(call.params["page"], 3);
}

#[test]
fn provider_search_encoding() {
    let call = encode_search(&SearchRequest::Steam(SteamSearch {
        query: String::from("city"),
        query_type: 3,
        trend_days: 3,
        request_type: String::from("Video"),
        category: String::from("Anime"),
        resolution: String::from("1920 x 1080"),
        allow_nsfw: false,
        page: 2,
    }));
    assert_eq!(call.method, "steam.search");
    assert_eq!(call.params["days"], 3);
    assert_eq!(call.params["tags"], json!(["Video", "Anime", "1920 x 1080"]));
    assert_eq!(
        call.params["excluded_tags"],
        json!([
            "Web",
            "Application",
            "Mature",
            "Questionable",
            "NSFW",
            "Partial Nudity",
            "Nudity",
            "Gore"
        ])
    );

    let unsplash = encode_search(&SearchRequest::Catalog(CatalogSearch {
        source: Source::Unsplash,
        query: String::from("misty forest"),
        page: 4,
        max_duration: 0,
        order_by: String::from("latest"),
        orientation: String::from("landscape"),
        size: String::new(),
        color: String::from("green"),
        content_filter: String::from("high"),
    }));
    assert_eq!(unsplash.method, "source.list");
    assert_eq!(unsplash.params["order_by"], "latest");
    assert_eq!(unsplash.params["orientation"], "landscape");
    assert_eq!(unsplash.params["color"], "green");
    assert_eq!(unsplash.params["content_filter"], "high");

    let pexels = encode_search(&SearchRequest::Catalog(CatalogSearch {
        source: Source::Pexels,
        query: String::from("city"),
        page: 1,
        max_duration: 0,
        order_by: String::new(),
        orientation: String::from("portrait"),
        size: String::from("large"),
        color: String::from("blue"),
        content_filter: String::new(),
    }));
    assert_eq!(pexels.params["orientation"], "portrait");
    assert_eq!(pexels.params["size"], "large");
    assert_eq!(pexels.params["color"], "blue");
}

#[test]
fn download_preview_apply_encoding() {
    let download = encode_download(&DownloadRequest::Catalog {
        source: Source::Youtube,
        id: String::from("video"),
        full_url: String::from("https://example.invalid/video"),
        attribution: String::from("artist"),
        track_url: String::from("https://example.invalid/track"),
        clip: Some(crate::contracts::browser::Clip { start_secs: 7, duration_secs: Some(30) }),
    });
    assert_eq!(download.method, "source.download");
    assert_eq!(download.params["source"], "youtube");
    assert_eq!(download.params["start"], 7);
    assert_eq!(download.params["dur"], 30);

    let preview = encode_preview(&PreviewRequest {
        source: Source::Wallhaven,
        id: String::from("wall"),
        full_url: String::from("https://example.invalid/wall"),
    });
    assert_eq!(preview.method, "wallhaven.preview");

    assert_eq!(
        encode_apply(&ApplyTarget::Path { kind: ApplyKind::Video, path: String::from("/video") }),
        json!({"type": "video", "path": "/video"})
    );
}

#[test]
fn event_decoders_neutral() {
    assert_eq!(
        decode_preview_ready_path(&json!({"status": "ready", "path": "/cache/full"})).as_deref(),
        Some("/cache/full")
    );
    assert_eq!(decode_preview_ready_path(&json!({"status": "fetching"})), None);

    let update = decode_download_event(&json!({
        "id": "video",
        "status": "downloading",
        "progress": 0.5,
        "message": "video"
    }))
    .expect("valid download event");
    assert_eq!(update.status, DownloadStatus::Downloading);
    assert_eq!(update.progress, Some(0.5));
}

#[test]
fn source_availability_gates() {
    let config = super::super::config::Config::from_data(json!({
        "features": {"steam": false},
        "sources": {
            "unsplash": {"enabled": true, "accessKey": ""},
            "pexels": {"enabled": true, "apiKey": "key"}
        }
    }));
    let availability = source_availability(&config);
    let get = |source| availability.iter().find(|entry| entry.source == source).unwrap();
    assert_eq!(get(Source::Wallhaven).unavailable, None);
    assert_eq!(
        get(Source::Steam).unavailable,
        Some(crate::contracts::browser::SourceUnavailableReason::Disabled)
    );
    assert_eq!(
        get(Source::Unsplash).unavailable,
        Some(crate::contracts::browser::SourceUnavailableReason::MissingCredentials)
    );
    assert_eq!(get(Source::Pexels).unavailable, None);
}
