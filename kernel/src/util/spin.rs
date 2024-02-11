use core::sync::atomic::{AtomicBool, Ordering};

pub struct SpinLock {
    pub locked: AtomicBool,
}

impl SpinLock {
    pub const fn new() -> Self {
        Self {
            locked: AtomicBool::new(false),
        }
    }
    pub fn lock(&self) {
        while self.locked.swap(true, Ordering::Acquire) {}
    }
    pub fn unlock(&self) {
        self.locked.store(false, Ordering::Release)
    }
}
