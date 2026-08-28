fn block_base(thumb: &str) -> String {
    let base = thumb
        .replace("/thumbs/", "/blocks/")
        .replace("/video-thumbs/", "/blocks/vid--")
        .replace("/we-thumbs/", "/blocks/we--");
    base.strip_suffix(".webp").map(String::from).unwrap_or(base)
}

pub fn far_block_path(thumb: &str) -> String {
    format!("{}.bc1", block_base(thumb))
}

pub fn near_block_path(thumb: &str) -> String {
    format!("{}.bc7", block_base(thumb))
}
