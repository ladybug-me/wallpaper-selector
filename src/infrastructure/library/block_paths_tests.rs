#![cfg(test)]

use super::far_block_path;

#[test]
fn far_block_kinds() {
    assert_eq!(far_block_path("/c/skwd-wall/thumbs/a.webp"), "/c/skwd-wall/blocks/a.bc1");
    assert_eq!(
        far_block_path("/c/skwd-wall/video-thumbs/clip.webp"),
        "/c/skwd-wall/blocks/vid--clip.bc1"
    );
    assert_eq!(far_block_path("/c/skwd-wall/we-thumbs/77.webp"), "/c/skwd-wall/blocks/we--77.bc1");
}
