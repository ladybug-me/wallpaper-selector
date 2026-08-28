use iced::Task;

use crate::frontend::scene::layout::GridParams;
use crate::infrastructure::config::Config;

#[allow(clippy::wildcard_imports)]
use super::super::*;

pub(crate) fn browser_wall_params(config: &Config) -> GridParams {
    let layout = config.browser_grid(false);
    let total_w =
        layout.thumb_w * layout.cols as f32 + layout.gap_x * layout.cols.saturating_sub(1) as f32;
    let total_h =
        layout.thumb_h * layout.rows as f32 + layout.gap_y * layout.rows.saturating_sub(1) as f32;
    GridParams {
        cols: layout.cols,
        rows: layout.rows,
        thumb_w: total_w / layout.cols.max(1) as f32,
        thumb_h: total_h / layout.rows.max(1) as f32,
        gap_x: 0.0,
        gap_y: 0.0,
        corner_radius: 0.0,
        border_width: 0.0,
        ..GridParams::default()
    }
}

pub(crate) fn browser_key_nav(app: &mut App, delta: i64) -> Task<Message> {
    let Some(browser) = app.source_browser.browser.as_ref() else {
        return Task::none();
    };
    let count = browser.session.items.len() as i64;
    if count == 0 {
        return Task::none();
    }
    if let Some(current) = browser.session.preview {
        if delta.abs() != 1 {
            return Task::none();
        }
        let next = current as i64 + delta;
        if next < 0 || next >= count {
            return Task::none();
        }
        return update(
            app,
            Message::Browser(crate::frontend::browser::BrowserMsg::OpenPreview(next as usize)),
        );
    }
    let next = match app.source_browser.wall.scene.hover {
        Some(current) => current as i64 + delta,
        None => 0,
    };
    if next < 0 || next >= count {
        return Task::none();
    }
    let next = next as usize;
    app.source_browser.wall.scene.kb_nav = true;
    app.source_browser.wall.scene.set_current(next, count as usize);
    app.source_browser.wall.scene.hover = Some(next);
    if let Some(browser) = app.source_browser.browser.as_mut() {
        browser.session.hover = Some(next);
    }
    app.source_browser.wall.chrome_cache.clear();
    app.retick();
    Task::none()
}
