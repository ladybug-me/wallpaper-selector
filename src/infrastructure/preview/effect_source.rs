use futures_channel::mpsc::UnboundedSender;

use crate::infrastructure::runtime::Wake;

pub(crate) fn decode_effect_source(path: &str, cap: u32) -> Option<(Vec<u8>, u32, u32)> {
    let image = image::open(path).ok()?;
    let image = if image.width().max(image.height()) > cap {
        image.resize(cap, cap, image::imageops::FilterType::Triangle)
    } else {
        image
    };
    let rgba = image.to_rgba8();
    let (width, height) = rgba.dimensions();
    Some((rgba.into_raw(), width, height))
}

pub(crate) fn spawn_effect_source_decode(
    card: usize,
    source: String,
    cap: u32,
    sender: UnboundedSender<Wake>,
) {
    let _ = std::thread::Builder::new().name("effect-source".into()).spawn(move || {
        if let Some((rgba, width, height)) = decode_effect_source(&source, cap) {
            let _ = sender.unbounded_send(Wake::EffectSrc {
                card,
                source,
                rgba: std::sync::Arc::new(rgba),
                w: width,
                h: height,
            });
        }
    });
}
