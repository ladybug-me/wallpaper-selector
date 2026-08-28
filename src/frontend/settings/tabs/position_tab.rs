use crate::contracts::settings::keys;
use crate::i18n::tr;

use super::Builder;

pub(super) fn tab_position(builder: &mut Builder<'_>) {
    picker_position(
        builder,
        tr("settings-position-slices-card"),
        keys::selector::SLICE_STAGE_X,
        keys::selector::SLICE_STAGE_Y,
    );
    picker_position(
        builder,
        tr("settings-position-hex-card"),
        keys::selector::HEX_STAGE_X,
        keys::selector::HEX_STAGE_Y,
    );
    picker_position(
        builder,
        tr("settings-position-wall-card"),
        keys::selector::GRID_STAGE_X,
        keys::selector::GRID_STAGE_Y,
    );
    picker_position(
        builder,
        tr("settings-position-sandy-card"),
        keys::selector::SANDY_STAGE_X,
        keys::selector::SANDY_STAGE_Y,
    );
    builder.card(tr("settings-filter-position-card"), tr("settings-filter-position-card-desc"));
    builder.num_setting(
        tr("settings-filter-offset-x-label"),
        tr("settings-filter-offset-x-desc"),
        crate::contracts::settings::schema::setting::filter_bar::OFFSET_X,
        "px",
    );
    builder.num_setting(
        tr("settings-filter-offset-y-label"),
        tr("settings-filter-offset-y-desc"),
        crate::contracts::settings::schema::setting::filter_bar::OFFSET_Y,
        "px",
    );
    builder.card(
        tr("settings-selector-tag-cloud-position-card"),
        tr("settings-selector-tag-cloud-position-card-desc"),
    );
    builder.num(
        tr("settings-selector-tag-offset-x-label"),
        tr("settings-selector-tag-offset-x-desc"),
        keys::selector::TAG_CLOUD_OFFSET_X,
        "px",
    );
    builder.num(
        tr("settings-selector-tag-offset-y-label"),
        tr("settings-selector-tag-offset-y-desc"),
        keys::selector::TAG_CLOUD_OFFSET_Y,
        "px",
    );
}

fn picker_position(builder: &mut Builder<'_>, title: &'static str, x: &str, y: &str) {
    builder.card(title, tr("settings-position-picker-card-desc"));
    builder.num(
        tr("settings-position-horizontal-label"),
        tr("settings-position-horizontal-desc"),
        x,
        "%",
    );
    builder.num(
        tr("settings-position-vertical-label"),
        tr("settings-position-vertical-desc"),
        y,
        "%",
    );
}
