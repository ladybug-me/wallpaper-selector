use serde_json::Value;

use crate::contracts::daemon::{
    CardPickerCreateResult, CardPickerMembershipsResult, PlaylistListResult, PlaylistMembersResult,
    PlaylistOutputsResult,
};
use crate::contracts::playlists::{Playlist, PlaylistAssignment, PlaylistKind, PlaylistMember};

use super::common::{DecodeResult, array, envelope, integer, typed};

pub fn decode_playlist_list(value: &Value) -> DecodeResult<PlaylistListResult> {
    let object = envelope("playlist.list", value)?;
    let playlists = typed::<Vec<wall_proto::PlaylistRow>>(
        "playlist.list",
        object,
        "playlists",
        "playlist array",
    )?
    .map(|rows| rows.into_iter().map(map_playlist).collect());
    let assignments = typed::<Vec<wall_proto::PlaylistAssign>>(
        "playlist.list",
        object,
        "assign",
        "playlist assignment array",
    )?
    .map(|rows| {
        rows.into_iter().map(|row| PlaylistAssignment { output: row.output, id: row.id }).collect()
    });
    Ok(PlaylistListResult { playlists, assignments })
}

pub fn decode_playlist_members(value: &Value) -> DecodeResult<PlaylistMembersResult> {
    let object = envelope("playlist.members", value)?;
    let members = typed::<Vec<wall_proto::WallpaperItem>>(
        "playlist.members",
        object,
        "members",
        "wallpaper array",
    )?
    .map(|rows| rows.into_iter().map(map_member).collect());
    Ok(PlaylistMembersResult { id: integer("playlist.members", object, "id")?, members })
}

pub fn decode_playlist_outputs(value: &Value) -> DecodeResult<PlaylistOutputsResult> {
    let object = envelope("wall.outputs", value)?;
    let outputs = array("wall.outputs", object, "outputs")?.map(|rows| {
        rows.iter()
            .filter_map(|row| row.get("name").and_then(Value::as_str).map(str::to_string))
            .collect()
    });
    Ok(PlaylistOutputsResult { outputs })
}

pub fn decode_card_picker_memberships(value: &Value) -> DecodeResult<CardPickerMembershipsResult> {
    let object = envelope("playlist.memberships", value)?;
    let ids = array("playlist.memberships", object, "ids")?
        .map(|ids| ids.iter().filter_map(Value::as_i64).collect());
    Ok(CardPickerMembershipsResult { ids })
}

pub fn decode_card_picker_create(value: &Value) -> DecodeResult<CardPickerCreateResult> {
    let object = envelope("playlist.create", value)?;
    Ok(CardPickerCreateResult { id: integer("playlist.create", object, "id")? })
}

fn map_playlist(row: wall_proto::PlaylistRow) -> Playlist {
    Playlist {
        id: row.id,
        name: row.name,
        kind: PlaylistKind::from(row.kind.as_str()),
        source: row.source,
        order: row.order,
        dwell: row.dwell,
        position: row.position,
        count: row.count,
    }
}

fn map_member(row: wall_proto::WallpaperItem) -> PlaylistMember {
    PlaylistMember {
        key: row.key,
        kind: row.kind,
        preview: row.preview,
        thumb: row.thumb,
        thumb_sm: row.thumb_sm,
    }
}
