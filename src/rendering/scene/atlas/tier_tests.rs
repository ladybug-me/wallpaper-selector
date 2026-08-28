#![cfg(test)]

use super::*;

#[test]
fn layer_extent_gap() {
    let mut tier = TierMap::new(8);
    for index in 0..6usize {
        tier.acquire(index);
    }
    tier.release(3);
    assert_eq!(tier.slot_of.len() as u32, 5);
    assert_eq!(tier.layer_extent(), 6);
    assert!(tier.acquire(99).is_some());
    assert_eq!(tier.layer_extent(), 6);
}

#[test]
fn acquire_full_pending() {
    let mut tier = TierMap::new(2);
    assert_eq!(tier.acquire(10), Some(0));
    assert_eq!(tier.acquire(11), Some(1));
    assert_eq!(tier.acquire(12), None);
    assert!(tier.is_known(10) && tier.is_known(11));
    assert!(!tier.is_known(12));
}

#[test]
fn acquire_evicts_lru() {
    let mut tier = TierMap::new(2);
    tier.acquire(10);
    tier.acquire(11);
    tier.mark_ready(10);
    tier.mark_ready(11);
    tier.touch(10);
    let id = tier.acquire(12);
    assert_eq!(id, Some(1));
    assert!(!tier.is_known(11));
    assert!(tier.is_known(10));
    assert_eq!(tier.ready(12), None);
    tier.mark_ready(12);
    assert_eq!(tier.ready(12), Some(1));
}

#[test]
fn acquire_resident_refresh() {
    let mut tier = TierMap::new(2);
    tier.acquire(10);
    tier.acquire(11);
    tier.mark_ready(10);
    tier.mark_ready(11);
    assert_eq!(tier.acquire(10), None);
    assert_eq!(tier.acquire(12), Some(1));
    assert!(tier.is_known(10));
}

#[test]
fn acquire_prefers_free() {
    let mut tier = TierMap::new(2);
    tier.acquire(10);
    tier.acquire(11);
    tier.mark_ready(10);
    tier.mark_ready(11);
    tier.release(10);
    assert_eq!(tier.acquire(12), Some(0));
    assert!(tier.is_known(11));
}

#[test]
fn ready_gates() {
    let mut tier = TierMap::new(2);
    tier.acquire(10);
    assert_eq!(tier.ready(10), None);
    tier.mark_ready(10);
    assert_eq!(tier.ready(10), Some(0));
    assert_eq!(tier.ready(99), None);
}

#[test]
fn pressure_lru_order() {
    let mut tier = TierMap::new(3);
    for index in 0..3usize {
        tier.acquire(index);
        tier.mark_ready(index);
    }
    for index in 3..9usize {
        let id = tier.acquire(index);
        assert!(id.is_some());
        assert!(id.unwrap() < 3);
        assert!(!tier.is_known(index - 3));
        tier.mark_ready(index);
    }
    assert_eq!(tier.slot_of.len(), 3);
    assert_eq!(tier.layer_extent(), 3);
}

#[test]
fn frame_pins_block_reuse() {
    let mut tier = TierMap::new(2);
    tier.acquire(10);
    tier.acquire(11);
    tier.mark_ready(10);
    tier.mark_ready(11);

    tier.begin_frame();
    tier.pin(10);
    tier.pin(11);
    assert_eq!(tier.acquire(12), None);
    assert_eq!(tier.ready(10), Some(0));
    assert_eq!(tier.ready(11), Some(1));

    tier.begin_frame();
    tier.pin(12);
    assert_eq!(tier.acquire(12), None);

    tier.begin_frame();
    tier.pin(12);
    assert!(tier.acquire(12).is_some());
}

#[test]
fn overflow_falls_back() {
    let mut tier = TierMap::new(4);
    for index in 0..4 {
        tier.acquire(index);
        tier.mark_ready(index);
    }

    tier.begin_frame();
    for index in 0..8 {
        tier.pin(index);
        if !tier.is_known(index) {
            let _ = tier.acquire(index);
        }
    }

    for index in 0..4 {
        assert_eq!(tier.ready(index), Some(index as u32));
    }
    for index in 4..8 {
        assert!(!tier.is_known(index), "{index}");
    }
    let unique =
        tier.slot_of.values().map(|slot| slot.id).collect::<std::collections::HashSet<_>>();
    assert_eq!(unique.len(), tier.slot_of.len());
}

#[test]
fn rgba_profile_bounds() {
    let atlas = AtlasMap::with_budget(10_000, budget(false));
    assert_eq!(atlas.size, RGBA_ATLAS_SIZE);
    assert_eq!(atlas.far_layers, RGBA_FAR_LAYERS);
    assert_eq!(atlas.near_capacity(), RGBA_NEAR_CAP);
    assert_eq!(atlas.far_px(65), (0, 800, 880));
    assert_eq!(atlas.far_px(66), (1, 0, 0));
}

#[test]
fn budget_map_agreement() {
    for compressed in [true, false] {
        let spec = budget(compressed);
        let map = AtlasMap::with_budget(10_000, spec);
        assert_eq!(spec.size, map.size, "{compressed}");
        assert_eq!(spec.near_cap, map.near_capacity(), "{compressed}");
    }
    for near_all_cards in [true, false] {
        let spec = browser_budget(near_all_cards);
        let map = AtlasMap::with_budget(768, spec);
        assert_eq!(spec.size, map.size);
        assert_eq!(spec.near_cap, map.near_capacity());
    }
}

#[test]
fn browser_budget_uncompressed() {
    let compressed = budget(true);
    let browser = browser_budget(true);
    assert_eq!(
        browser,
        AtlasBudget {
            size: RGBA_ATLAS_SIZE,
            near_cap: BROWSER_NEAR_CAP,
            near_init_layers: BROWSER_NEAR_INIT_LAYERS,
            near_grow_chunk: BROWSER_NEAR_GROW_CHUNK,
            far_initial_layers: RGBA_FAR_LAYERS,
            far_cap: RGBA_FAR_CAP,
        }
    );
    assert_ne!(browser.size, compressed.size);
    assert!(browser.near_cap < compressed.near_cap);
    assert_eq!(browser_budget(false), budget(false));
    let map = AtlasMap::with_budget(768, browser);
    assert_eq!(map.far_layers, RGBA_FAR_LAYERS);
    assert_eq!(AtlasMap::with_budget(768, compressed).far_layers, 3);
}

#[test]
fn browser_near_cap_page() {
    let (cols, rows) = (6u32, 3u32);
    let spec = browser_budget(true);
    assert!(spec.near_cap >= cols * (rows + 1));
    assert!(spec.near_init_layers <= spec.near_cap);
    assert_eq!(spec.near_cap % spec.near_grow_chunk, 0);
    assert_eq!(spec.far_cap, far_slots_per_layer(spec.size) * spec.far_initial_layers);
    assert!(spec.far_cap >= 3 * cols * (rows + 1));
}
