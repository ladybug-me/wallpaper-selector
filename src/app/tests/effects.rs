use super::*;

#[test]
fn modifier_clicks_open_effects() {
    let mut app = test_app();
    seed(&mut app, &[wall("a.png", "static", 1, 0), wall("v.mp4", "video", 2, 0)]);
    app.scene.mode = Mode::Sandy;
    app.scene.viewport = (1280.0, 720.0);
    app.scene.set_visible(true);
    app.scene.set_current(0, app.library_session.filtered.len());
    app.scene.sandy_settle_now();
    let mut now = Instant::now();
    tick_frames(&mut app, &mut now, 24);

    let static_hit =
        app.scene.render.hits.iter().find(|hit| hit.index == 0).copied().expect("static hit");
    let _ = update(&mut app, Message::SetMods(crate::domain::input::Mods::new(false, false, true)));
    let _ = update(
        &mut app,
        Message::Click(static_hit.cx, static_hit.cy, crate::domain::input::MouseButton::Right),
    );
    let effects = app.panels.effects.as_ref().expect("shift+right-click opens effects");
    assert_eq!(effects.mode(), crate::frontend::effects::EffectsMode::Studio);
    assert_eq!(effects.card(), 0);
    app.panels.effects = None;
    tick_frames(&mut app, &mut now, 24);

    let video_hit =
        app.scene.render.hits.iter().find(|hit| hit.index == 1).copied().expect("video hit");
    let _ = update(
        &mut app,
        Message::Click(video_hit.cx, video_hit.cy, crate::domain::input::MouseButton::Right),
    );
    assert!(app.panels.effects.is_none());
    app.scene.close_flip();
    tick_frames(&mut app, &mut now, 24);

    let static_hit =
        app.scene.render.hits.iter().find(|hit| hit.index == 0).copied().expect("static hit");
    let _ = update(&mut app, Message::SetMods(crate::domain::input::Mods::new(true, false, false)));
    let _ = update(
        &mut app,
        Message::Click(static_hit.cx, static_hit.cy, crate::domain::input::MouseButton::Left),
    );
    let effects = app.panels.effects.as_ref().expect("ctrl+left-click opens effects");
    assert_eq!(effects.mode(), crate::frontend::effects::EffectsMode::Displays);
}

#[test]
fn clicks_run_bound_action() {
    use crate::domain::input::{Mods, MouseButton};
    use crate::frontend::settings::SettingsMsg;
    let mut app = test_app();
    seed(&mut app, &[wall("a.png", "static", 1, 0), wall("b.png", "static", 2, 0)]);
    app.scene.mode = Mode::Sandy;
    app.scene.viewport = (1280.0, 720.0);
    app.scene.set_visible(true);
    app.scene.set_current(0, app.library_session.filtered.len());
    app.scene.sandy_settle_now();
    let mut now = Instant::now();
    tick_frames(&mut app, &mut now, 24);
    let hit = app.scene.render.hits.iter().find(|hit| hit.index == 0).copied().expect("hit");

    let _ = update(
        &mut app,
        Message::Settings(SettingsMsg::Input("keys.playlists".into(), "middle-click".into())),
    );
    let _ = update(&mut app, Message::SetMods(Mods::NONE));
    let _ = update(&mut app, Message::Click(hit.cx, hit.cy, MouseButton::Middle));
    assert!(app.panels.playlists.is_some());
    let _ = update(&mut app, Message::ClosePlaylists);

    let _ =
        update(&mut app, Message::Settings(SettingsMsg::KeybindCapture("keys.favourite".into())));
    let _ =
        update(&mut app, Message::Settings(SettingsMsg::KeybindCaptureClick(MouseButton::Left)));
    let _ = update(&mut app, Message::Settings(SettingsMsg::KeybindCaptureApply));
    assert_eq!(app.config.str_path("keys.select"), "none");
    let favourited = |app: &App| {
        let catalog = app.library_session.library.catalog();
        catalog.is_favourite(&catalog.items[0])
    };
    let before = favourited(&app);
    let _ = update(&mut app, Message::Click(hit.cx, hit.cy, MouseButton::Left));
    assert_ne!(favourited(&app), before);

    let _ =
        update(&mut app, Message::Settings(SettingsMsg::KeybindCapture("keys.favourite".into())));
    let _ = update(&mut app, Message::Settings(SettingsMsg::KeybindCaptureUnbind));
    let _ = update(&mut app, Message::Settings(SettingsMsg::KeybindCaptureApply));
    let before = favourited(&app);
    let _ = update(&mut app, Message::Click(hit.cx, hit.cy, MouseButton::Left));
    assert_eq!(favourited(&app), before);
}

#[test]
fn studio_renders_default_effect() {
    let mut app = test_app();
    seed(&mut app, &[wall("a.png", "static", 1, 0)]);
    app.daemon.effect_definitions = crate::infrastructure::effects::decode_definitions(&[json!({
        "id": "theme", "label": "Theme", "category": "Colour",
        "params": [{ "id": "theme", "type": "dropdown", "default": "Catppuccin" }]
    })]);
    let _ = drain_calls(&app);
    crate::app::helpers::open_effects(&mut app, 0, crate::frontend::effects::EffectsMode::Studio);
    assert_eq!(app.panels.effects.as_ref().unwrap().effect_id(), "theme");
    let calls = drain_calls(&app);
    assert!(calls.iter().any(|(method, _)| method == "effects.preview"));

    app.panels.effects = None;
    let _ = drain_calls(&app);
    crate::app::helpers::open_effects(&mut app, 0, crate::frontend::effects::EffectsMode::Displays);
    let calls = drain_calls(&app);
    assert!(!calls.iter().any(|(method, _)| method == "effects.preview"));
}

#[test]
fn sandy_click_applies_non_hovered() {
    let mut app = test_app();
    seed(
        &mut app,
        &[
            wall("a", "static", 10, 1),
            wall("b", "static", 20, 2),
            wall("c", "static", 30, 3),
            wall("d", "static", 40, 4),
            wall("e", "static", 50, 5),
        ],
    );
    app.scene.mode = Mode::Sandy;
    app.scene.viewport = (1280.0, 720.0);
    app.scene.set_visible(true);
    app.scene.set_current(2, app.library_session.filtered.len());
    app.scene.sandy_settle_now();
    let mut now = Instant::now();
    tick_frames(&mut app, &mut now, 24);
    let _ = drain_calls(&app);
    let hit = app
        .scene
        .render
        .hits
        .iter()
        .find(|hit| hit.index != app.scene.current)
        .copied()
        .expect("strip card hit");
    let _ =
        update(&mut app, Message::Click(hit.cx, hit.cy, crate::domain::input::MouseButton::Left));
    let calls = drain_calls(&app);
    assert!(calls.iter().any(|(method, _)| method == "wall.apply"));
}

#[test]
fn we_monitor_thumb_by_id() {
    let mut app = test_app();
    let mut scene = wall("Cool Scene", "we", 5, 1);
    scene["we_id"] = json!("2057951800");
    scene["thumb"] = json!("/thumbs/cool-scene.png");
    seed(&mut app, &[scene, wall("plain.png", "static", 2, 0)]);
    app.panels.effects = Some(crate::frontend::effects::Effects::new(
        Vec::new(),
        String::from("/src.png"),
        None,
        0,
        WallpaperKind::We,
        true,
        80,
        String::from("we:2057951800"),
    ));
    app.daemon.pending.insert(9, Pending::Outputs);
    respond(
        &mut app,
        9,
        json!({
            "outputs": [{
                "name": "DP-1", "type": "we", "we_id": "2057951800",
                "current": "2057951800", "path": "", "mute": true, "volume": 100
            }]
        }),
    );
    let mons = app.panels.effects.as_ref().unwrap().monitors();
    assert_eq!(mons.len(), 1);
    assert_eq!(mons[0].current_thumb.as_deref(), Some("/thumbs/cool-scene.png"));
}

#[test]
fn monitor_selector_transformed_size() {
    let mut app = test_app();
    seed(&mut app, &[wall("portrait.png", "static", 5, 1)]);
    app.panels.effects = Some(crate::frontend::effects::Effects::new(
        Vec::new(),
        String::from("/src.png"),
        None,
        0,
        WallpaperKind::Static,
        true,
        100,
        String::from("static:portrait"),
    ));
    app.daemon.pending.insert(9, Pending::Outputs);
    respond(
        &mut app,
        9,
        json!({
            "outputs": [{
                "name": "DP-5", "width": 2560, "height": 1440,
                "logical_width": 1440, "logical_height": 2560,
                "type": "static", "mute": true, "volume": 100
            }]
        }),
    );

    let monitors = app.panels.effects.as_ref().unwrap().monitors();
    assert_eq!(monitors.len(), 1);
    assert_eq!((monitors[0].width, monitors[0].height), (1440, 2560));
}

#[test]
fn monitor_lock_shared_setting() {
    let mut app = test_app();
    seed(&mut app, &[wall("locked.png", "static", 5, 1)]);
    let path = format!("{}.DP-1", skwd_config::keys::display::OUTPUT_LOCKS);
    app.config.save_key(&path, json!(true));
    app.panels.effects = Some(crate::frontend::effects::Effects::new(
        Vec::new(),
        String::from("/src.png"),
        None,
        0,
        WallpaperKind::Static,
        true,
        100,
        String::from("static:locked"),
    ));
    app.daemon.pending.insert(9, Pending::Outputs);
    respond(
        &mut app,
        9,
        json!({
            "outputs": [{
                "name": "DP-1", "width": 1920, "height": 1080,
                "type": "static", "mute": true, "volume": 100
            }]
        }),
    );
    assert!(app.panels.effects.as_ref().unwrap().monitors()[0].locked);

    let _ = update(
        &mut app,
        Message::Effects(crate::frontend::effects::EffectsMsg::MonitorLock(
            String::from("DP-1"),
            false,
        )),
    );
    assert!(!app.config.flag_default_config(&path));
    assert!(!app.panels.effects.as_ref().unwrap().monitors()[0].locked);
}

#[test]
fn video_monitor_thumb_by_file() {
    let mut app = test_app();
    let mut vid = wall("Rainsong", "video", 5, 1);
    vid["video_file"] = json!("/vids/Rainsong.mp4");
    vid["thumb"] = json!("/thumbs/rainsong.png");
    seed(&mut app, &[vid, wall("other.png", "static", 2, 0)]);
    app.panels.effects = Some(crate::frontend::effects::Effects::new(
        Vec::new(),
        String::from("/src.png"),
        Some(String::from("/thumbs/NEW-source.png")),
        0,
        WallpaperKind::Video,
        true,
        80,
        String::from("video:new"),
    ));
    app.daemon.pending.insert(9, Pending::Outputs);
    respond(
        &mut app,
        9,
        json!({
            "outputs": [{
                "name": "DP-1", "type": "video",
                "current": "/vids/Rainsong.mp4", "path": "/vids/Rainsong.mp4",
                "we_id": "", "mute": true, "volume": 100
            }]
        }),
    );
    let mons = app.panels.effects.as_ref().unwrap().monitors();
    assert_eq!(mons.len(), 1);
    assert_eq!(mons[0].current_thumb.as_deref(), Some("/thumbs/rainsong.png"));
}
