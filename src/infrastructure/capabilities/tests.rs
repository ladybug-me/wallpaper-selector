use std::cell::Cell;

use crate::contracts::capabilities::{GraphicsCard, GraphicsProbe, GraphicsTier};

use super::gpu_cache::{decode, encode, load_or_probe};

struct CountingProbe<'a> {
    calls: &'a Cell<usize>,
    card: GraphicsCard,
}

impl GraphicsProbe for CountingProbe<'_> {
    fn probe(&self) -> GraphicsCard {
        self.calls.set(self.calls.get() + 1);
        self.card.clone()
    }
}

#[test]
fn codec_round_trip() {
    let card = GraphicsCard {
        name: String::from("NVIDIA GeForce RTX 5080"),
        tier: GraphicsTier::Discrete,
    };
    let text = encode(&card, "boot|123|nv");
    assert_eq!(decode(&text, "boot|123|nv"), Some(card));
    assert_eq!(decode(&text, "boot|999|nv"), None);
    assert_eq!(decode("garbage", "boot|123|nv"), None);
}

#[test]
fn codec_odd_names() {
    for name in ["weird\tname gpu", "line\nbreak gpu", "quoted \"gpu\"", "trailing space "] {
        let card = GraphicsCard { name: String::from(name), tier: GraphicsTier::Integrated };
        assert_eq!(decode(&encode(&card, "key"), "key"), Some(card), "{name}");
    }
}

#[test]
fn legacy_cache_rejected() {
    assert_eq!(decode("boot|123|nv\nNVIDIA GeForce RTX 5080\tDiscrete", "boot|123|nv"), None);
    assert_eq!(decode(r#"{"key":"k","name":"gpu","tier":"Quantum"}"#, "k"), None);
}

#[test]
fn probes_once_then_caches() {
    let unique =
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
    let directory =
        std::env::temp_dir().join(format!("skwd-wall-gpu-cache-{}-{unique}", std::process::id()));
    let path = directory.join("nested").join("gpu-tier");
    let calls = Cell::new(0);
    let probe = CountingProbe {
        calls: &calls,
        card: GraphicsCard { name: String::from("first"), tier: GraphicsTier::Discrete },
    };

    assert_eq!(load_or_probe(&path, "driver-a", &probe), probe.card);
    assert_eq!(calls.get(), 1);

    let different_probe = CountingProbe {
        calls: &calls,
        card: GraphicsCard { name: String::from("second"), tier: GraphicsTier::Other },
    };
    assert_eq!(load_or_probe(&path, "driver-a", &different_probe), probe.card);
    assert_eq!(calls.get(), 1);

    assert_eq!(load_or_probe(&path, "driver-b", &different_probe), different_probe.card);
    assert_eq!(calls.get(), 2);

    let _ = std::fs::remove_dir_all(directory);
}
