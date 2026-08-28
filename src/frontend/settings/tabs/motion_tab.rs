#[allow(clippy::wildcard_imports)]
use super::*;
use crate::i18n::tr;

pub(super) fn tab_motion(builder: &mut Builder<'_>) {
    builder.card(tr("settings-motion-motion-card"), tr("settings-motion-motion-card-desc"));
    builder.motion_weights();
}
