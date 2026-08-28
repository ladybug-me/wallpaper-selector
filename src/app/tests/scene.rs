use super::*;

#[test]
fn filter_swap_startup() {
    let dir = std::env::temp_dir().join(format!(
        "skwd-flipms-{}-{}",
        std::process::id(),
        TEST_DIR_SEQ.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    ));
    std::fs::create_dir_all(&dir).unwrap();
    let mut config = Config::from_data(json!({
        "paths": { "cache": dir.join("cache").to_string_lossy(), "wallpaper": "/wp",
            "videoWallpaper": "/vids" },
        "motion": { "slowMs": 1600.0 }
    }));
    config.config_path = dir.join("config.json");
    let app =
        App::with_config_using(config, |_| crate::infrastructure::ipc::DaemonClient::recording());
    assert!((app.scene.filter_swap_ms() - 1600.0).abs() < 0.01);
}

#[test]
fn open_fade_startup() {
    let dir = std::env::temp_dir().join(format!(
        "skwd-openfade-{}-{}",
        std::process::id(),
        TEST_DIR_SEQ.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    ));
    std::fs::create_dir_all(&dir).unwrap();
    let mut config = Config::from_data(json!({
        "paths": { "cache": dir.join("cache").to_string_lossy(), "wallpaper": "/wp",
            "videoWallpaper": "/vids" },
        "motion": { "standardMs": 777.0 }
    }));
    config.config_path = dir.join("config.json");
    let app =
        App::with_config_using(config, |_| crate::infrastructure::ipc::DaemonClient::recording());
    assert!((app.scene.open_fade_ms() - 777.0).abs() < 0.01);
    assert!(app.scene.open_fade() < 1.0);
}

#[test]
fn motion_weights_reach_all_state() {
    use crate::frontend::animation::MotionTier;

    let dir = std::env::temp_dir().join(format!(
        "skwd-motion-policy-{}-{}",
        std::process::id(),
        TEST_DIR_SEQ.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    ));
    std::fs::create_dir_all(&dir).unwrap();
    let mut config = Config::from_data(json!({
        "paths": {
            "cache": dir.join("cache").to_string_lossy(),
            "wallpaper": "/wp",
            "videoWallpaper": "/vids"
        },
        "motion": { "fastMs": 90.0, "standardMs": 330.0, "slowMs": 810.0 }
    }));
    config.config_path = dir.join("config.json");
    let mut app =
        App::with_config_using(config, |_| crate::infrastructure::ipc::DaemonClient::recording());

    assert_eq!(app.scene.motion_duration_ms(MotionTier::Fast), 90.0);
    assert_eq!(app.scene.motion_duration_ms(MotionTier::Standard), 330.0);
    assert_eq!(app.scene.motion_duration_ms(MotionTier::Slow), 810.0);
    assert_eq!(app.panels.settings.control_anim.duration_ms(), 90.0);
    assert_eq!(app.panels.settings.entrance.duration_ms(), 330.0);
    assert!((app.chrome.bar.menu_scroll.duration_ms() - 330.0).abs() < 0.01);
    assert_eq!(app.source_browser.entrance.duration_ms(), 810.0);
    assert_eq!(app.source_browser.wall.scene.motion_duration_ms(MotionTier::Standard), 330.0);
    assert_eq!(app.tags.cloud_entrance.duration_ms(), 330.0);
    assert!((app.tags.cloud_scroll.duration_ms() - 330.0).abs() < 0.01);
    assert_eq!(app.theme.fade_t.duration_ms(), 330.0);

    app.panels.audio =
        Some(crate::frontend::audio_panel::AudioPanel::new_with_motion(app.motion_profile()));
    assert_eq!(app.panels.audio.as_ref().unwrap().open.duration_ms(), 90.0);
}

#[test]
fn set_view_mode() {
    let mut app = test_app();
    seed(&mut app, &[wall("a.png", "static", 1, 0)]);
    assert_eq!(app.scene.mode, Mode::Slices);
    let _ = update(&mut app, Message::SetViewMode(String::from("grid")));
    assert_eq!(app.scene.mode, Mode::Grid);
    assert_eq!(app.config.display_mode(), "wall");
    let text = std::fs::read_to_string(&app.config.config_path).expect("mode saved to disk");
    let saved: Value = serde_json::from_str(&text).unwrap();
    assert_eq!(saved["components"]["wallpaperSelector"]["displayMode"], "wall");
    let _ = update(&mut app, Message::SetViewMode(String::from("hex")));
    assert_eq!(app.scene.mode, Mode::Hex);
}

#[test]
fn shader_mode_degrades() {
    let cfg = Config::from_data(json!({
        "components": {"wallpaperSelector": {"displayMode": "shader:plasma"}}
    }));
    assert_eq!(layout_params(&cfg).mode, Mode::Slices);
    for (name, expected) in [
        ("grid", Mode::Grid),
        ("wall", Mode::Grid),
        ("hex", Mode::Hex),
        ("sandy", Mode::Sandy),
        ("nova", Mode::Sandy),
    ] {
        let cfg = Config::from_data(json!({
            "components": {"wallpaperSelector": {"displayMode": name}}
        }));
        assert_eq!(layout_params(&cfg).mode, expected, "{name}");
    }
    for name in ["spiral", "mosaic", "cascade", "warp", "fluid", "lens", "shelves"] {
        let cfg = Config::from_data(json!({
            "components": {"wallpaperSelector": {"displayMode": name}}
        }));
        assert_eq!(layout_params(&cfg).mode, Mode::Slices, "{name}");
    }
}

#[test]
fn apply_static_neighbors() {
    let mut app = test_app();
    seed(
        &mut app,
        &[
            wall("a.png", "static", 1, 0),
            wall("b.png", "static", 2, 0),
            wall("c.png", "static", 3, 0),
        ],
    );
    let _ = update(&mut app, Message::ApplyCurrent);
    let calls = drain_calls(&app);
    let apply = calls.iter().find(|(method, _)| method == "wall.apply").expect("wall.apply sent");
    assert_eq!(apply.1["type"], "static");
    assert_eq!(apply.1["path"], "/wp/a.png");
    assert_eq!(apply.1["neighbors"], json!(["/wp/b.png", "/wp/c.png"]));
}

#[test]
fn apply_we_item() {
    let mut app = test_app();
    let mut we = wall("workshop item", "we", 1, 0);
    we["we_id"] = json!("w42");
    seed(&mut app, &[we, wall("b.png", "static", 2, 0)]);
    let _ = update(&mut app, Message::ApplyCurrent);
    let calls = drain_calls(&app);
    let apply = calls.iter().find(|(method, _)| method == "wall.apply").expect("wall.apply sent");
    assert_eq!(apply.1["type"], "we");
    assert_eq!(apply.1["we_id"], "w42");
    assert_eq!(apply.1["screens"], json!([]));
    assert!(apply.1.get("neighbors").is_none());
    assert!(apply.1.get("path").is_none());
}

#[test]
fn scene_props_load_and_write() {
    let mut app = test_app();
    let mut we = wall("workshop item", "we", 1, 0);
    we["we_id"] = json!("scene-42");
    seed(&mut app, &[we]);

    let _ = update(&mut app, Message::OpenSceneProps);
    let panel = app.panels.scene_properties.as_ref().unwrap();
    assert_eq!((panel.we_id.as_str(), panel.title.as_str()), ("scene-42", "workshop item"));
    let calls = drain_calls(&app);
    assert!(calls.iter().any(|(method, params)| {
        method == wall_proto::rpc::WALL_WE_PROPERTIES && params["we_id"] == "scene-42"
    }));

    app.on_result(
        Pending::SceneProperties { we_id: "scene-42".into() },
        &json!({
            "we_id": "scene-42",
            "properties": [{
                "name": "glow",
                "label": "Glow",
                "kind": "bool",
                "value": true,
                "default": true
            }]
        }),
    );
    let _ = update(
        &mut app,
        Message::SceneProps(crate::frontend::scene_properties::ScenePropMsg::Toggle("glow".into())),
    );
    let calls = drain_calls(&app);
    assert!(calls.iter().any(|(method, params)| {
        method == wall_proto::rpc::WALL_SET_WE_PROPERTY
            && params["we_id"] == "scene-42"
            && params["name"] == "glow"
            && params["value"] == false
    }));
}

#[test]
fn collect_neighbors_caps() {
    let mut app = test_app();
    let walls: Vec<Value> =
        (0..30).map(|i| wall(&format!("i{i:02}.png"), "static", 5, 0)).collect();
    seed(&mut app, &walls);
    let picks = collect_neighbors(&app, "/wp/i05.png");
    let expected: Vec<String> =
        [6, 4, 7, 3, 8, 2, 9, 1, 10, 0, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]
            .iter()
            .map(|i| format!("/wp/i{i:02}.png"))
            .collect();
    assert_eq!(picks, expected);
    assert_eq!(picks.len(), 20);
    assert!(!picks.contains(&String::from("/wp/i05.png")));
    assert!(collect_neighbors(&app, "/wp/unknown.png").is_empty());
}

#[test]
fn apply_params_kinds() {
    use crate::domain::library::catalog::Wallpaper;
    let img =
        Wallpaper { kind: WallpaperKind::Static, path: "/wp/a.png".into(), ..Default::default() };
    let params = apply_params(&img, vec!["/wp/b.png".into()]);
    assert_eq!(params["type"], "static");
    assert_eq!(params["neighbors"][0], "/wp/b.png");

    let we = Wallpaper { kind: WallpaperKind::We, we_id: "111".into(), ..Default::default() };
    assert_eq!(apply_params(&we, vec![])["type"], "we");
}

#[test]
fn keyboard_nav_clamps() {
    let mut app = test_app();
    seed(
        &mut app,
        &[
            wall("a.png", "static", 1, 0),
            wall("b.png", "static", 2, 0),
            wall("c.png", "static", 3, 0),
        ],
    );
    let _ = update(&mut app, Message::KeyPrev);
    assert_eq!(app.scene.current, 0);
    let _ = update(&mut app, Message::KeyNext);
    let _ = update(&mut app, Message::KeyNext);
    let _ = update(&mut app, Message::KeyNext);
    assert_eq!(app.scene.current, 2);
    app.panels.settings.open = true;
    let _ = update(&mut app, Message::KeyPrev);
    assert_eq!(app.scene.current, 2);
    drain_calls(&app);
    let _ = update(&mut app, Message::ApplyCurrent);
    assert!(drain_calls(&app).iter().all(|(method, _)| method != "wall.apply"));
}
