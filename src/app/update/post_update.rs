use iced::Task;

#[allow(clippy::wildcard_imports)]
use super::*;

pub(super) fn finish(
    app: &mut App,
    task: Task<Message>,
    detail_was_open: bool,
    sync_settings_preview: bool,
    sync_filter_bar: bool,
) -> Task<Message> {
    if detail_was_open != app.detail_open() {
        app.chrome.bar.cache.clear();
    }
    app.sync_filter_bar_chrome();
    let hard_overlay = app.picker_obscured();
    let (design_mode, scene_hidden) = scene_visibility(
        app.panels.settings.open,
        &app.panels.settings.tab,
        app.panels.settings.section,
        hard_overlay,
    );
    app.scene.set_visible(!scene_hidden);
    let inset = if design_mode {
        crate::frontend::settings::picker_layout_studio_width(
            app.scene.viewport.0,
            app.config.ui_scale(),
        ) + 36.0 * app.config.ui_scale()
    } else if app.panels.playlists.is_some() && !scene_hidden {
        PLAYLIST_DOCK_W
    } else {
        0.0
    };
    app.panels.settings.inset_target = if design_mode { inset } else { 0.0 };
    app.scene.set_center_inset(inset);
    if sync_filter_bar {
        let bar_footprint = (app.chrome.filter_bar_fade() > 0.001 && !hard_overlay).then(|| {
            let (vw, vh) = app.scene.viewport;
            crate::app::view::filter_bar_footprint(app, vw, vh)
        });
        app.scene.set_filter_bar_footprint(bar_footprint);
    }
    app.schedule_frame();
    if sync_settings_preview { Task::batch([task, app.sync_settings_preview()]) } else { task }
}
