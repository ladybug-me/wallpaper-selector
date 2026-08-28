use std::sync::Arc;

use crate::frontend::animation::Spring;

use super::InstanceRaw;

#[cfg(test)]
mod tests;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LaunchAnim {
    None,
    Fade,
    Rise,
    Zoom,
}

impl LaunchAnim {
    pub fn from_key(kind: &str) -> Self {
        match kind {
            "none" => Self::None,
            "rise" => Self::Rise,
            "zoom" => Self::Zoom,
            _ => Self::Fade,
        }
    }
}

pub(crate) struct Transition {
    pub(crate) old: Arc<Vec<InstanceRaw>>,
    pub(crate) progress: Spring,
    pub(crate) kind: u32,
    pub(crate) origin: [f32; 2],
}

pub(crate) fn apply_entrance_motion(
    instances: &mut [InstanceRaw],
    animation: LaunchAnim,
    entrance: f32,
    viewport_width: f32,
    viewport_height: f32,
) {
    if entrance >= 0.999 {
        return;
    }
    match animation {
        LaunchAnim::Rise => {
            let offset_y = (1.0 - entrance).powi(2) * viewport_height * 0.03;
            for instance in instances {
                instance.rect[1] += offset_y;
            }
        }
        LaunchAnim::Zoom => {
            let scale = 0.94 + 0.06 * entrance;
            let (center_x, center_y) = (viewport_width * 0.5, viewport_height * 0.5);
            for instance in instances {
                instance.rect[0] = center_x + (instance.rect[0] - center_x) * scale;
                instance.rect[1] = center_y + (instance.rect[1] - center_y) * scale;
                instance.rect[2] *= scale;
                instance.rect[3] *= scale;
            }
        }
        LaunchAnim::None | LaunchAnim::Fade => {}
    }
}
