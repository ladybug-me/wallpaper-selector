#![cfg(test)]

use super::Mode;

#[test]
fn canonical_keys_round_trip() {
    for mode in [Mode::Slices, Mode::Grid, Mode::Hex, Mode::Sandy] {
        assert_eq!(Mode::from_key(mode.as_key()), mode);
        assert_eq!(Mode::try_from_key(mode.as_key()), Some(mode));
    }
}

#[test]
fn aliases_decode_only() {
    assert_eq!(Mode::from_key("grid"), Mode::Grid);
    assert_eq!(Mode::from_key("nova"), Mode::Sandy);
    assert_eq!(Mode::Grid.as_key(), "wall");
    assert_eq!(Mode::Sandy.as_key(), "sandy");
}

#[test]
fn unknown_keys_default_slices() {
    for key in ["", "spiral", "mosaic", "WALL", "Hex"] {
        assert_eq!(Mode::try_from_key(key), None);
        assert_eq!(Mode::from_key(key), Mode::Slices);
    }
}
