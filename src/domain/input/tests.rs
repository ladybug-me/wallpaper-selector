use super::{
    InputAction, InputMap, KeyId, KeySpec, Mods, MouseButton, MouseSpec, Trigger, binding_config,
    binding_label, parse_binding,
};

fn key(text: &str) -> Trigger {
    Trigger::parse(text).expect("valid trigger")
}

#[test]
fn parser_accepts_triggers() {
    assert_eq!(
        Trigger::parse("p"),
        Some(Trigger::Key(KeySpec { mods: Mods::NONE, id: KeyId::Char("p".into()) }))
    );
    assert_eq!(
        Trigger::parse(" Shift + LEFT "),
        Some(Trigger::Key(KeySpec { mods: Mods::new(false, false, true), id: KeyId::Left }))
    );
    assert_eq!(
        Trigger::parse("shift++"),
        Some(Trigger::Key(KeySpec {
            mods: Mods::new(false, false, true),
            id: KeyId::Char("+".into())
        }))
    );
    assert_eq!(
        Trigger::parse("click"),
        Some(Trigger::Mouse(MouseSpec { mods: Mods::NONE, button: MouseButton::Left }))
    );
    assert_eq!(
        Trigger::parse("ctrl+right-click"),
        Some(Trigger::Mouse(MouseSpec {
            mods: Mods::new(true, false, false),
            button: MouseButton::Right
        }))
    );
    assert_eq!(
        Trigger::parse("middle-click"),
        Some(Trigger::Mouse(MouseSpec { mods: Mods::NONE, button: MouseButton::Middle }))
    );
}

#[test]
fn parser_rejects_invalid() {
    for invalid in ["", "banana+x", "pp", "shift+", "pageup", "scroll", "click+ctrl"] {
        assert_eq!(Trigger::parse(invalid), None);
    }
}

#[test]
fn label_config_round_trip() {
    for (raw, shown) in [
        ("shift+left", "Shift+Left"),
        ("ctrl+alt+shift+d", "Ctrl+Alt+Shift+D"),
        ("enter", "Enter"),
        ("return", "Enter"),
        ("tab", "Tab"),
        ("space", "Space"),
        ("click", "Click"),
        ("right-click", "Right-click"),
        ("ctrl+click", "Ctrl+Click"),
        ("shift+middle-click", "Shift+Middle-click"),
    ] {
        let trigger = Trigger::parse(raw).expect(raw);
        assert_eq!(trigger.label(), shown);
        assert_eq!(Trigger::parse(&trigger.config()), Some(trigger));
    }
    assert_eq!(key("ctrl+alt+shift+d").config(), "ctrl+alt+shift+d");
}

#[test]
fn bindings_hold_several_triggers() {
    let triggers = parse_binding("i, right-click").expect("valid binding");
    assert_eq!(triggers, vec![key("i"), key("right-click")]);
    assert_eq!(binding_config(&triggers), "i, right-click");
    assert_eq!(binding_label(&triggers), "I \u{b7} Right-click");
    assert_eq!(parse_binding("none"), Some(Vec::new()));
    assert_eq!(binding_config(&[]), "none");
    assert_eq!(parse_binding(""), None);
    assert_eq!(parse_binding("i, banana"), None);
}

#[test]
fn defaults_reachable() {
    for action in InputAction::ALL {
        assert!(parse_binding(action.default_binding()).is_some(), "{action:?}");
    }
    let map = InputMap::default();
    assert_eq!(map.lookup_key(&KeyId::Char("p".into()), Mods::NONE), Some(InputAction::Playlists));
    assert_eq!(
        map.lookup_key(&KeyId::Char("p".into()), Mods::new(false, false, true)),
        Some(InputAction::Playlists)
    );
    assert_eq!(map.lookup_key(&KeyId::Char("p".into()), Mods::new(true, false, false)), None);
    assert_eq!(map.lookup_key(&KeyId::Enter, Mods::NONE), Some(InputAction::Apply));
    assert_eq!(map.lookup_key(&KeyId::Tab, Mods::NONE), Some(InputAction::Autocomplete));
    assert_eq!(map.lookup_key(&KeyId::Left, Mods::NONE), Some(InputAction::NavLeft));
    assert_eq!(
        map.lookup_key(&KeyId::Left, Mods::new(false, false, true)),
        Some(InputAction::ColorPrev)
    );
}

#[test]
fn clicks_resolve_like_keys() {
    let map = InputMap::default();
    let click = |mods, button| map.lookup_mouse(MouseSpec { mods, button });
    assert_eq!(click(Mods::NONE, MouseButton::Left), Some(InputAction::Select));
    assert_eq!(click(Mods::NONE, MouseButton::Right), Some(InputAction::Flip));
    assert_eq!(click(Mods::new(true, false, false), MouseButton::Left), Some(InputAction::Effects));
    assert_eq!(click(Mods::new(false, false, true), MouseButton::Right), Some(InputAction::Studio));
    assert_eq!(click(Mods::NONE, MouseButton::Middle), None);
    assert_eq!(click(Mods::new(false, true, false), MouseButton::Left), None);
}

#[test]
fn any_trigger_kind() {
    let map = InputMap::from_bindings([
        (InputAction::Playlists, "middle-click".to_string()),
        (InputAction::Select, "u".to_string()),
        (InputAction::Studio, "ctrl+alt+shift+d".to_string()),
    ]);
    assert_eq!(
        map.lookup_mouse(MouseSpec { mods: Mods::NONE, button: MouseButton::Middle }),
        Some(InputAction::Playlists)
    );
    assert_eq!(map.lookup_key(&KeyId::Char("u".into()), Mods::NONE), Some(InputAction::Select));
    assert_eq!(
        map.lookup_key(&KeyId::Char("d".into()), Mods::new(true, true, true)),
        Some(InputAction::Studio)
    );
    assert_eq!(map.lookup_mouse(MouseSpec { mods: Mods::NONE, button: MouseButton::Left }), None);
}

#[test]
fn invalid_override_fallback() {
    let map = InputMap::from_bindings([
        (InputAction::Playlists, "f".to_string()),
        (InputAction::Flip, "banana+q".to_string()),
    ]);
    assert_eq!(
        map.lookup_mouse(MouseSpec { mods: Mods::NONE, button: MouseButton::Right }),
        Some(InputAction::Flip)
    );
    assert_eq!(map.conflicts(), vec![(InputAction::Favourite, InputAction::Playlists)]);
    assert_eq!(map.shared_trigger(InputAction::Playlists, &key("f")), Some(InputAction::Favourite));
    assert_eq!(map.shared_trigger(InputAction::Playlists, &key("z")), None);
}

#[test]
fn card_actions_need_target() {
    for action in [
        InputAction::Select,
        InputAction::Apply,
        InputAction::Flip,
        InputAction::Favourite,
        InputAction::Effects,
        InputAction::Studio,
    ] {
        assert!(action.targets_card(), "{action:?}");
    }
    for action in [InputAction::Playlists, InputAction::Settings, InputAction::NavLeft] {
        assert!(!action.targets_card(), "{action:?}");
    }
}
