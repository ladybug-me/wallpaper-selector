use std::sync::OnceLock;

pub fn is_niri() -> bool {
    static IS_NIRI: OnceLock<bool> = OnceLock::new();
    *IS_NIRI.get_or_init(|| {
        std::env::var("XDG_CURRENT_DESKTOP").is_ok_and(|desktop| desktop_is_niri(&desktop))
    })
}

fn desktop_is_niri(desktop: &str) -> bool {
    desktop.to_lowercase().contains("niri")
}

#[cfg(test)]
mod tests;
