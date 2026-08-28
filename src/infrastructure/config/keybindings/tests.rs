#![cfg(test)]

use serde_json::json;

use super::*;
use crate::domain::input::{InputAction, KeyId, Mods, MouseButton, MouseSpec};

#[test]
fn key_overrides_decode() {
    let config = Config::from_data(json!({
        "keys": {
            "playlists": "x",
            "flip": "banana+q",
            "effects": "ctrl+e",
            "select": "middle-click",
        }
    }));
    let bindings = load_bindings(&config);
    assert_eq!(
        bindings.lookup_key(&KeyId::Char("x".into()), Mods::NONE),
        Some(InputAction::Playlists)
    );
    assert_eq!(
        bindings.lookup_mouse(MouseSpec { mods: Mods::NONE, button: MouseButton::Right }),
        Some(InputAction::Flip)
    );
    assert_eq!(
        bindings.lookup_key(&KeyId::Char("e".into()), Mods::new(true, false, false)),
        Some(InputAction::Effects)
    );
    assert_eq!(
        bindings.lookup_mouse(MouseSpec { mods: Mods::NONE, button: MouseButton::Middle }),
        Some(InputAction::Select)
    );
}
