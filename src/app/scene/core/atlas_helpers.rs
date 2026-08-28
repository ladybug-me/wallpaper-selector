use crate::domain::library::catalog::Wallpaper;
use crate::frontend::scene::InstanceRaw;
use crate::infrastructure::preview::{DecodePool, Job};
use crate::rendering::scene::atlas::{self, AtlasMap};

pub(super) fn atlas_debug_enabled() -> bool {
    static ENABLED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *ENABLED.get_or_init(|| std::env::var_os("SKWD_WALL_ATLAS_DBG").is_some())
}

pub(super) fn audit_atlas(
    instances: &[InstanceRaw],
    atlas: Option<&AtlasMap>,
    flips: usize,
    velocity: f32,
) {
    let mut tiles: std::collections::HashMap<(u32, u32, u32, u32), usize> =
        std::collections::HashMap::new();
    for (position, instance) in instances.iter().enumerate() {
        let mode = instance.misc[0];
        if mode != 1 && mode != 2 {
            continue;
        }
        let key = (mode, instance.misc[1], instance.uv[0].to_bits(), instance.uv[1].to_bits());
        if let Some(previous) = tiles.insert(key, position) {
            let prior = &instances[previous];
            log::warn!(
                "ATLAS_DBG dup mode={mode} layer={} insts {previous}@({:.0},{:.0} w{:.0}) & {position}@({:.0},{:.0} w{:.0}) flips={flips} vel={velocity:.0}",
                instance.misc[1],
                prior.rect[0],
                prior.rect[1],
                prior.rect[2],
                instance.rect[0],
                instance.rect[1],
                instance.rect[2],
            );
        }
    }
    let Some(atlas) = atlas else {
        return;
    };
    for (tier, map) in [("near", &atlas.near), ("far", &atlas.far)] {
        let mut ids: std::collections::HashMap<u32, usize> = std::collections::HashMap::new();
        for (&index, slot) in &map.slot_of {
            if let Some(other) = ids.insert(slot.id, index) {
                log::warn!("ATLAS_DBG {tier} double-map id={} idx {other}&{index}", slot.id);
            }
        }
    }
}

pub(super) fn enqueue_near(
    pool: &DecodePool,
    atlas: &mut AtlasMap,
    item: &Wallpaper,
    store_index: usize,
    visible: bool,
    direct: bool,
) {
    if let Some(id) = atlas.near.acquire(store_index) {
        let (layer, x, y) = AtlasMap::near_px(id);
        let compressed = !direct && crate::contracts::preview::compressed_thumbnails();
        pool.enqueue(
            Job {
                store_idx: store_index,
                path: if compressed {
                    crate::infrastructure::library::near_block_path(&item.thumb)
                } else {
                    item.thumb.clone()
                },
                fallback: None,
                tier: 1,
                layer,
                x,
                y,
                w: atlas::NEAR_W,
                h: atlas::NEAR_H,
                compressed,
            },
            visible,
        );
    }
}

pub(super) fn enqueue_far(
    pool: &DecodePool,
    atlas: &mut AtlasMap,
    item: &Wallpaper,
    store_index: usize,
    visible: bool,
    direct: bool,
) {
    if let Some(id) = atlas.far.acquire(store_index) {
        let (layer, x, y) = atlas.far_px(id);
        let compressed = !direct && crate::contracts::preview::compressed_thumbnails();
        pool.enqueue(
            Job {
                store_idx: store_index,
                path: if compressed {
                    crate::infrastructure::library::far_block_path(&item.thumb)
                } else {
                    item.thumb.clone()
                },
                fallback: compressed.then(|| item.thumb.clone()),
                tier: 2,
                layer,
                x,
                y,
                w: atlas::FAR_W,
                h: atlas::FAR_H,
                compressed,
            },
            visible,
        );
    }
}
