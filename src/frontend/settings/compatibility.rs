pub fn canonical_category(category: &str) -> &str {
    match category {
        "general" | "selector" | "keybinds" => "picker",
        "launch" | "transitions" => "motion",
        "paper" | "wallpaper-engine" => "playback",
        "paths" => "library",
        "wallhaven" | "steam" => "sources",
        "tagging" => "search",
        "schedule" | "postprocessing" => "automation",
        "matugen" => "theme",
        "niri" => "integrations",
        _ => category,
    }
}
