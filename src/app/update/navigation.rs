use iced::Task;

use crate::frontend::scene::layout::Mode;

#[allow(clippy::wildcard_imports)]
use super::super::*;
use super::browser::browser_apply_current;
use super::effects::effects_nav;
use super::update_inner;

pub(super) fn wheel(app: &mut App, amount: f32) -> Task<Message> {
    if app.detail_open() || app.menu_capturing() {
        return Task::none();
    }
    match app.scene.mode {
        Mode::Slices => {
            app.scene.slice_scroll(amount, app.library_session.filtered.len());
        }
        Mode::Grid => app.scene.grid_scroll(-amount),
        Mode::Sandy => app.scene.slice_scroll(amount, app.library_session.filtered.len()),
        Mode::Hex => hex_wheel(app, amount),
    }
    app.retick();
    Task::none()
}

fn hex_wheel(app: &mut App, amount: f32) {
    let count = app.library_session.filtered.len() as i64;
    if count == 0 {
        return;
    }
    let rows = app.scene.hp.rows.max(1) as i64;
    let step = app.scene.hp.scroll_step as i64;
    let total_cols = (count + rows - 1) / rows;
    let col = app.scene.current as i64 / rows;
    let new_col = (col - amount.signum() as i64 * step).clamp(0, total_cols - 1);
    let idx = (new_col * rows + app.scene.hex_row as i64).min(count - 1).max(0);
    app.scene.set_current(idx as usize, app.library_session.filtered.len());
}

pub(super) fn key_prev(app: &mut App) -> Task<Message> {
    if app.source_browser.browser.is_some() {
        return browser_key_nav(app, -1);
    }
    if let Some(task) = effects_nav(app, -1, 0) {
        return task;
    }
    if app.menu_capturing() || app.detail_open() {
        return Task::none();
    }
    app.scene.kb_nav = true;
    let cur = app.scene.current;
    if cur > 0 {
        app.scene.set_current(cur - 1, app.library_session.filtered.len());
        app.retick();
    }
    Task::none()
}

pub(super) fn key_next(app: &mut App) -> Task<Message> {
    if app.source_browser.browser.is_some() {
        return browser_key_nav(app, 1);
    }
    if let Some(task) = effects_nav(app, 1, 0) {
        return task;
    }
    if app.menu_capturing() || app.detail_open() {
        return Task::none();
    }
    app.scene.kb_nav = true;
    let cur = app.scene.current;
    if cur + 1 < app.library_session.filtered.len() {
        app.scene.set_current(cur + 1, app.library_session.filtered.len());
        app.retick();
    }
    Task::none()
}

pub(super) fn apply_current(app: &mut App) -> Task<Message> {
    if app.source_browser.browser.is_some() {
        return browser_apply_current(app);
    }
    if app.panels.effects.as_ref().is_some_and(|eff| {
        eff.mode() == crate::frontend::effects::EffectsMode::Studio && eff.has_effects_page()
    }) {
        return update_inner(app, Message::Effects(crate::frontend::effects::EffectsMsg::Apply));
    }
    if app.menu_capturing() {
        return Task::none();
    }
    apply_task(app, app.scene.current)
}

pub(super) fn key_favourite(app: &mut App) -> Task<Message> {
    if app.menu_capturing() {
        return Task::none();
    }
    toggle_favourite(app, app.scene.flipped().unwrap_or(app.scene.current));
    Task::none()
}

pub(super) fn key_flip(app: &mut App) -> Task<Message> {
    if app.menu_capturing() {
        return Task::none();
    }
    if app.detail_open() || app.scene.flip_open() {
        app.scene.close_flip();
        app.retick();
        return Task::none();
    }
    if app.library_session.filtered.is_empty() {
        return Task::none();
    }
    let idx = app.scene.current;
    match app.scene.mode {
        Mode::Slices => app.scene.toggle_flip(idx),
        Mode::Sandy => {
            app.scene.sandy_settle_now();
            app.scene.toggle_flip(app.scene.current);
        }
        _ => {
            let rect = app
                .scene
                .render
                .hits
                .iter()
                .find(|hit| hit.index == idx)
                .map(|hit| [hit.cx, hit.cy, hit.hw, hit.hh]);
            if let Some(rect) = rect {
                app.scene.open_detail(idx, rect);
            }
        }
    }
    app.retick();
    Task::none()
}

pub(super) fn key_effects(app: &mut App) -> Task<Message> {
    let Some(idx) = effects_target(app) else {
        return Task::none();
    };
    open_effects(app, idx, crate::frontend::effects::EffectsMode::Displays);
    Task::none()
}

pub(super) fn key_studio(app: &mut App) -> Task<Message> {
    let Some(idx) = effects_target(app) else {
        return Task::none();
    };
    super::picker::open_studio(app, idx)
}

fn effects_target(app: &mut App) -> Option<usize> {
    if app.panels.effects.is_some() {
        app.panels.effects = None;
        app.retick();
        return None;
    }
    if app.menu_capturing() || app.library_session.filtered.is_empty() {
        return None;
    }
    Some(app.scene.flipped().unwrap_or(app.scene.current))
}

pub(super) fn key_up(app: &mut App) -> Task<Message> {
    if app.source_browser.browser.is_some() {
        let cols = app.source_browser.wall.scene.gp.cols.max(1) as i64;
        return browser_key_nav(app, -cols);
    }
    if let Some(task) = effects_nav(app, 0, -1) {
        return task;
    }
    if app.menu_capturing() || app.detail_open() {
        return Task::none();
    }
    if app.scene.mode == Mode::Hex && app.scene.hex_row > 0 {
        app.scene.kb_nav = true;
        let col = app.scene.current / app.scene.hp.rows.max(1);
        let idx = col * app.scene.hp.rows + app.scene.hex_row - 1;
        app.scene.set_current(
            idx.min(app.library_session.filtered.len().saturating_sub(1)),
            app.library_session.filtered.len(),
        );
        app.retick();
    }
    if app.scene.mode == Mode::Grid {
        let cols = app.scene.gp.cols.max(1);
        let cur = app.scene.current;
        if cur >= cols {
            app.scene.kb_nav = true;
            app.scene.set_current(cur - cols, app.library_session.filtered.len());
            app.retick();
        }
    }
    Task::none()
}

pub(super) fn key_down(app: &mut App) -> Task<Message> {
    if app.source_browser.browser.is_some() {
        let cols = app.source_browser.wall.scene.gp.cols.max(1) as i64;
        return browser_key_nav(app, cols);
    }
    if let Some(task) = effects_nav(app, 0, 1) {
        return task;
    }
    if app.menu_capturing() || app.detail_open() {
        return Task::none();
    }
    if app.scene.mode == Mode::Hex {
        let rows = app.scene.hp.rows.max(1);
        if app.scene.hex_row + 1 < rows {
            let col = app.scene.current / rows;
            let idx = col * rows + app.scene.hex_row + 1;
            if idx < app.library_session.filtered.len() {
                app.scene.kb_nav = true;
                app.scene.set_current(idx, app.library_session.filtered.len());
                app.retick();
            }
        }
    }
    if app.scene.mode == Mode::Grid {
        let cols = app.scene.gp.cols.max(1);
        let idx = app.scene.current + cols;
        if idx < app.library_session.filtered.len() {
            app.scene.kb_nav = true;
            app.scene.set_current(idx, app.library_session.filtered.len());
            app.retick();
        }
    }
    Task::none()
}
