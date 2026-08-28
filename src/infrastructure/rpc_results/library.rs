use serde_json::Value;

use crate::domain::library::catalog::Catalog;
use crate::infrastructure::library::LibraryPaths;

use super::common::{DecodeResult, envelope, required_typed};

pub fn decode_library_list(value: &Value, paths: LibraryPaths<'_>) -> DecodeResult<Catalog> {
    let object = envelope("wall.list", value)?;
    let rows = required_typed::<Vec<wall_proto::WallpaperItem>>(
        "wall.list",
        object,
        "wallpapers",
        "wallpaper array",
    )?;
    Ok(crate::infrastructure::library::decode_rows(&rows, paths))
}
