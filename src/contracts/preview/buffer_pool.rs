use std::sync::{Condvar, Mutex};

struct PoolState {
    free: Vec<Vec<u8>>,
    issued: usize,
}

pub struct BufPool {
    state: Mutex<PoolState>,
    ready: Condvar,
    bytes: usize,
    cap: usize,
    bounded: bool,
}

impl std::fmt::Debug for BufPool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BufPool")
            .field("bytes", &self.bytes)
            .field("cap", &self.cap)
            .finish_non_exhaustive()
    }
}

impl BufPool {
    pub fn new(bytes: usize, cap: usize) -> Self {
        Self {
            state: Mutex::new(PoolState { free: Vec::new(), issued: 0 }),
            ready: Condvar::new(),
            bytes,
            cap,
            bounded: false,
        }
    }

    pub(crate) fn new_bounded(bytes: usize, cap: usize) -> Self {
        Self { bounded: true, ..Self::new(bytes, cap.max(1)) }
    }

    pub fn take(&self) -> Vec<u8> {
        if let Some(mut buf) =
            self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner).free.pop()
        {
            buf.clear();
            buf.resize(self.bytes, 0);
            buf
        } else {
            vec![0u8; self.bytes]
        }
    }

    pub(crate) fn take_bounded(&self) -> Vec<u8> {
        debug_assert!(self.bounded);
        let mut state = self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        loop {
            if let Some(mut buf) = state.free.pop() {
                drop(state);
                buf.clear();
                buf.resize(self.bytes, 0);
                return buf;
            }
            if state.issued < self.cap {
                state.issued += 1;
                drop(state);
                return vec![0u8; self.bytes];
            }
            state = self.ready.wait(state).unwrap_or_else(std::sync::PoisonError::into_inner);
        }
    }

    pub fn put(&self, buf: Vec<u8>) {
        if buf.capacity() != self.bytes {
            if self.bounded {
                let mut state =
                    self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
                state.issued = state.issued.saturating_sub(1);
                self.ready.notify_one();
            }
            return;
        }
        let mut state = self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        if state.free.len() < self.cap {
            state.free.push(buf);
            self.ready.notify_one();
        }
    }
}

mod tests;
