use core::ops::{Deref, DerefMut};

use super::spin::SpinLock;

pub struct Mutex<T> {
    lock: SpinLock,
    val: T,
}

impl<T> Mutex<T> {
    pub const fn new(val: T) -> Self {
        Self {
            lock: SpinLock::new(),
            val,
        }
    }
    pub fn lock(&self) -> MutexGuard<T> {
        self.lock.lock();
        MutexGuard { mutex: self }
    }
}

pub struct MutexGuard<'a, T> {
    mutex: &'a Mutex<T>,
}

impl<'a, T> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        self.mutex.lock.unlock();
    }
}

impl<'a, T> Deref for MutexGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.mutex.val
    }
}

impl<'a, T> DerefMut for MutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe {&mut (*(self.mutex as *const Mutex<T> as *mut Mutex<T>)).val}
    }
}
