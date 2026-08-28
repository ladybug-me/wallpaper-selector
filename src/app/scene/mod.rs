mod core;
mod timing;
mod widget;

pub(crate) use core::{RebuildCtx, SceneCore};
pub(crate) use timing::{
    animation_tick_due, record_animation_tick, rephase_animation_deadline, retick_due,
};
pub(crate) use widget::{BrowserSceneProgram, SceneProgram};

#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum FrameDemand {
    #[default]
    Idle,
    Passive,
    Preview,
    Motion,
    Direct,
}
