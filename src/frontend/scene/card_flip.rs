use crate::frontend::animation::window;

const SHADER_END: f32 = 0.82;
const SURFACE_START: f32 = 0.74;
const SURFACE_END: f32 = 0.94;
const CONTENT_START: f32 = 0.84;
const CONTENT_END: f32 = 1.0;
const BACK_ONLY_SURFACE_START: f32 = 0.05;
const BACK_ONLY_SURFACE_END: f32 = 0.65;
const BACK_ONLY_CONTENT_START: f32 = 0.25;
const BACK_ONLY_CONTENT_END: f32 = 0.85;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct CardFlipPhases {
    pub shader: f32,
    pub surface: f32,
    pub content: f32,
}

pub(crate) fn card_flip_phases(
    progress: f32,
    animate_shader: bool,
    animate_back: bool,
) -> CardFlipPhases {
    let progress = progress.clamp(0.0, 1.0);
    let shader = if animate_shader { (progress / SHADER_END).clamp(0.0, 1.0) } else { 0.0 };
    let (surface, content) = match (animate_shader, animate_back) {
        (true, true) => (
            window(progress, SURFACE_START, SURFACE_END),
            ((progress - CONTENT_START) / (CONTENT_END - CONTENT_START)).clamp(0.0, 1.0),
        ),
        (false, true) => (
            window(progress, BACK_ONLY_SURFACE_START, BACK_ONLY_SURFACE_END),
            ((progress - BACK_ONLY_CONTENT_START)
                / (BACK_ONLY_CONTENT_END - BACK_ONLY_CONTENT_START))
                .clamp(0.0, 1.0),
        ),
        (true, false) => {
            let revealed = f32::from(progress >= SHADER_END);
            (revealed, revealed)
        }
        (false, false) => {
            let revealed = f32::from(progress >= 0.5);
            (revealed, revealed)
        }
    };
    CardFlipPhases { shader, surface, content }
}

pub(crate) fn card_flip_shader_payload(
    shader_progress: f32,
    card_index: usize,
    effect: u32,
) -> [f32; 4] {
    if shader_progress <= 0.001 {
        return [0.0; 4];
    }
    [shader_progress, (card_index as f32 % 7.0) * 1.3 + 0.5, effect as f32, 0.0]
}

#[cfg(test)]
#[path = "card_flip_tests.rs"]
mod tests;
