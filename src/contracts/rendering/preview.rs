use std::sync::Arc;

use crate::domain::effects::ShaderSpec;

#[derive(Debug, Clone)]
pub struct PreviewRequest {
    pub rgba: Arc<Vec<u8>>,
    pub width: u32,
    pub height: u32,
    pub version: u64,
    pub shader: ShaderSpec,
}

/// Consumer-owned boundary between effect presentation and a preview renderer.
///
/// The associated output keeps this contract independent of any UI or GPU framework.
pub trait PreviewRenderer<Message> {
    type Output;

    fn view(&self, request: PreviewRequest) -> Self::Output
    where
        Message: 'static;
}
