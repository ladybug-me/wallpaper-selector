use std::sync::{Arc, Mutex};

use super::BufPool;

pub const PREVIEW_WIDTH: u32 = 640;
pub const PREVIEW_HEIGHT: u32 = 360;

#[derive(Debug)]
pub struct Upload {
    pub tier: u32,
    pub layer: u32,
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
    pub compressed: bool,
    pub data: Vec<u8>,
    pub recycle: Option<Arc<BufPool>>,
}

impl Drop for Upload {
    fn drop(&mut self) {
        if let Some(pool) = self.recycle.take() {
            pool.put(std::mem::take(&mut self.data));
        }
    }
}

pub type UploadQueue = Arc<Mutex<Vec<Upload>>>;
