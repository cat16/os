use core::sync::atomic::{AtomicBool, Ordering};

pub struct SpinLock {
    pub locked: AtomicBool,
}

impl SpinLock {
    pub fn new() -> Self {
        Self {
            locked: AtomicBool::new(false),
        }
    }
    pub fn lock(&mut self) {
        while self.locked.swap(true, Ordering::Acquire) {}
    }
    pub fn unlock(&mut self) {
        self.locked.store(false, Ordering::Release)
    }
}
