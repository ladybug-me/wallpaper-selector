use crate::contracts::preview::UploadQueue;
use crate::infrastructure::preview::DecodePool;
use crate::rendering::scene::atlas::AtlasMap;

pub(crate) struct PreviewResources {
    pub(crate) decoder: DecodePool,
    pub(crate) uploads: UploadQueue,
    pub(crate) atlas: Option<AtlasMap>,
    pub(crate) decoder_was_busy: bool,
    pub(crate) render_loop_active: bool,
}

impl PreviewResources {
    pub(crate) fn new(decoder: DecodePool, uploads: UploadQueue) -> Self {
        Self { decoder, uploads, atlas: None, decoder_was_busy: false, render_loop_active: false }
    }
}
