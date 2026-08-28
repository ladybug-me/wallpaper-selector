const PORT: &str = include_str!("../source.rs");

#[test]
fn port_has_no_bodies() {
    let bodied: Vec<&str> = PORT
        .lines()
        .map(str::trim)
        .filter(|line| line.starts_with("fn ") && !line.ends_with(';'))
        .collect();
    assert!(bodied.is_empty(), "{bodied:?}");
}

#[test]
fn port_has_no_typed_accessors() {
    assert!(!PORT.contains("Setting<"));
    for declaration in ["fn motion_fast_ms", "fn motion_standard_ms", "fn motion_slow_ms"] {
        assert!(PORT.contains(&format!("{declaration}(&self) -> f32;")), "{declaration}");
    }
}
