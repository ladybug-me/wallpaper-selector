#[allow(clippy::module_inception)]
mod animation;
mod motion;
mod tests;

pub use animation::{
    Spring, Timeline, Tween, approach, approach_k, ease_out_cubic, smoothstep, window,
};
pub use motion::{MotionProfile, MotionTier};
