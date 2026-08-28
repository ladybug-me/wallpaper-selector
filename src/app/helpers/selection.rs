use iced::Task;
use log::info;
use serde_json::json;

use crate::domain::library::catalog::{Wallpaper, WallpaperKind};

#[allow(clippy::wildcard_imports)]
use super::super::*;

const CONCEAL_PAD_MS: u64 = 700;

pub(crate) fn apply_params(item: &Wallpaper, neighbors: Vec<String>) -> serde_json::Value {
    let mut params = match item.kind.as_str() {
        wall_proto::kind::WE => {
            json!({"type": wall_proto::kind::WE, "we_id": item.we_id.clone(), "screens": []})
        }
        wall_proto::kind::VIDEO => {
            json!({"type": wall_proto::kind::VIDEO, "path": item.path.clone()})
        }
        _ => json!({"type": wall_proto::kind::STATIC, "path": item.path.clone()}),
    };
    if !neighbors.is_empty() && item.kind != WallpaperKind::We {
        let list = neighbors.into_iter().map(serde_json::Value::String).collect();
        params["neighbors"] = serde_json::Value::Array(list);
    }
    params
}

pub(crate) fn toggle_favourite(app: &mut App, filtered_index: usize) {
    let Some(&source_index) = app.library_session.filtered.get(filtered_index) else {
        return;
    };
    let key = app.library_session.library.catalog().items[source_index as usize].key.clone();
    let favourite = app.library_session.library.toggle_favourite(&key);
    app.daemon.client.call("wall.set_favourite", json!({ "key": key, "favourite": favourite }));
    app.scene.set_favourite(favourite);
    app.retick();
}

pub(crate) fn apply_task(app: &mut App, filtered_index: usize) -> Task<Message> {
    let Some(&catalog_index) = app.library_session.filtered.get(filtered_index) else {
        return Task::none();
    };
    app.scene.set_current(filtered_index, app.library_session.filtered.len());
    let item = &app.library_session.library.catalog().items[catalog_index as usize];
    let neighbors = collect_neighbors(app, &item.path);
    let params = apply_params(item, neighbors);
    info!("apply {} ({})", item.name, item.kind.as_str());
    app.daemon.client.call("wall.apply", params.clone());
    app.daemon.last_wallpaper = Some(params);
    if app.scene.mode == crate::frontend::scene::layout::Mode::Sandy
        && app.config.flag_default_config(skwd_config::keys::transition::ENABLED)
    {
        let duration =
            app.config.num_path(skwd_config::keys::transition::DURATION_MS).max(0.0) as u64;
        app.scene.conceal_hero(
            std::time::Instant::now() + std::time::Duration::from_millis(duration + CONCEAL_PAD_MS),
        );
    }
    if app.config.close_on_selection() {
        crate::app::warm::request_hide(app);
    }
    app.retick();
    Task::none()
}

pub(crate) fn collect_neighbors(app: &App, path: &str) -> Vec<String> {
    let count = app.library_session.filtered.len();
    if count == 0 || path.is_empty() {
        return Vec::new();
    }
    let Some(index) = app.library_session.filtered.iter().position(|&source_index| {
        app.library_session.library.catalog().items[source_index as usize].path == path
    }) else {
        return Vec::new();
    };
    let mut picks = Vec::new();
    let max = 20;
    let mut step = 1;
    while picks.len() < max && step < count {
        let forward = index + step;
        if forward < count {
            push_neighbor(app, forward, path, &mut picks);
            if picks.len() >= max {
                break;
            }
        }
        if let Some(backward) = index.checked_sub(step) {
            push_neighbor(app, backward, path, &mut picks);
        }
        step += 1;
    }
    picks
}

fn push_neighbor(app: &App, filtered_index: usize, path: &str, picks: &mut Vec<String>) {
    let other = &app.library_session.library.catalog().items
        [app.library_session.filtered[filtered_index] as usize]
        .path;
    if !other.is_empty() && other != path {
        picks.push(other.clone());
    }
}
