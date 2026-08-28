use std::path::Path;

use serde_json::Value;

use crate::contracts::browser::{
    BrowserCollection, BrowserSearchResult, DownloadResponse, SearchPage,
};

use super::common::{DecodeResult, envelope, required_array, required_string, root};

pub fn decode_browser_search(value: &Value) -> DecodeResult<SearchPage> {
    let object = envelope("browser.search", value)?;
    required_array("browser.search", object, "results")?;
    let result = root::<wall_proto::sources::ListResult>("browser.search", value, "search page")?;
    Ok(SearchPage {
        generation: result.generation,
        last_page: result.last_page,
        current_page: result.current_page,
        next_cursor: result.next_cursor.unwrap_or_default(),
        results: result.results.iter().map(decode_search_result).collect(),
    })
}

pub fn decode_browser_download(value: &Value) -> DecodeResult<DownloadResponse> {
    let object = envelope("browser.download", value)?;
    Ok(match required_string("browser.download", object, "status")?.as_str() {
        "exists" => DownloadResponse::Exists,
        "done" => DownloadResponse::Done,
        "started" => DownloadResponse::Started,
        _ => DownloadResponse::Other,
    })
}

pub fn decode_browser_collections(value: &Value) -> DecodeResult<Vec<BrowserCollection>> {
    let object = envelope("browser.collections", value)?;
    Ok(required_array("browser.collections", object, "collections")?
        .iter()
        .filter_map(|collection| {
            let object = collection.as_object()?;
            Some(BrowserCollection {
                id: object.get("id")?.as_u64()?.to_string(),
                label: object.get("label").and_then(Value::as_str).unwrap_or_default().to_string(),
            })
        })
        .collect())
}

fn decode_search_result(row: &wall_proto::sources::ListItem) -> BrowserSearchResult {
    let thumb_path = row.thumb_path.clone();
    let thumb_ready = !thumb_path.is_empty() && Path::new(&thumb_path).exists();
    BrowserSearchResult {
        id: row.id.clone(),
        full_url: row.full_url.clone(),
        thumb_path,
        title: row.title.clone(),
        resolution: row.resolution.clone(),
        purity: row.purity.clone(),
        file_size: row.file_size,
        category: row.category.clone(),
        duration_secs: row.duration_secs,
        downloaded: row.downloaded,
        attribution: row.attribution.clone(),
        attribution_url: row.attribution_url.clone(),
        track_url: row.track_url.clone(),
        thumb_ready,
    }
}
