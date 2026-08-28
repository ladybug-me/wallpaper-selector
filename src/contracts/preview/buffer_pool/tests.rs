#![cfg(test)]

use super::BufPool;

#[test]
fn recycles_without_realloc() {
    let pool = BufPool::new(8, 2);
    let mut first = pool.take();
    assert_eq!(first.len(), 8);
    first.fill(7);
    let ptr = first.as_ptr();
    pool.put(first);
    let second = pool.take();
    assert_eq!(second.as_ptr(), ptr);
    assert_eq!(second.len(), 8);
    assert!(second.iter().all(|&byte| byte == 0));
}

#[test]
fn cap_bounds_retained_buffers() {
    let pool = BufPool::new(8, 2);
    pool.put(vec![0u8; 8]);
    pool.put(vec![0u8; 8]);
    pool.put(vec![0u8; 8]);
    assert_eq!(pool.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner).free.len(), 2);
}

#[test]
fn rejects_oversized_buffers() {
    let pool = BufPool::new(8, 2);
    pool.put(vec![1u8; 32]);
    assert!(pool.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner).free.is_empty());
}

#[test]
fn rejects_undersized_buffers() {
    let pool = BufPool::new(8, 2);
    pool.put(Vec::with_capacity(4));
    assert!(pool.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner).free.is_empty());
}

#[test]
fn bounded_pool_waits() {
    let pool = std::sync::Arc::new(BufPool::new_bounded(8, 1));
    let first = pool.take_bounded();
    let pointer = first.as_ptr() as usize;
    let (tx, rx) = std::sync::mpsc::channel();
    let worker_pool = pool.clone();
    let worker = std::thread::spawn(move || {
        let next = worker_pool.take_bounded();
        tx.send(next.as_ptr() as usize).unwrap();
        worker_pool.put(next);
    });
    assert!(rx.recv_timeout(std::time::Duration::from_millis(20)).is_err());
    pool.put(first);
    assert_eq!(rx.recv_timeout(std::time::Duration::from_secs(1)).unwrap(), pointer);
    worker.join().unwrap();
}
