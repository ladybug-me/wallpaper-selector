use super::Uniforms;
use crate::rendering::wgsl_test_support::assert_uniform_layout;

#[test]
fn effect_preview_uniform_layout() {
    use std::mem::offset_of;
    assert_uniform_layout(
        "effect_preview",
        include_str!("../../../shaders/effect_preview.wgsl"),
        "Uniforms",
        &[
            ("bounds", offset_of!(Uniforms, bounds)),
            ("tex_size", offset_of!(Uniforms, tex_size)),
            ("effect", offset_of!(Uniforms, effect)),
            ("out_srgb", offset_of!(Uniforms, out_srgb)),
            ("params", offset_of!(Uniforms, params)),
            ("color_a", offset_of!(Uniforms, color_a)),
            ("color_b", offset_of!(Uniforms, color_b)),
        ],
        std::mem::size_of::<Uniforms>(),
    );
}

#[test]
fn effect_preview_uniform_alignment() {
    assert_eq!(std::mem::size_of::<Uniforms>() % 16, 0);
}
