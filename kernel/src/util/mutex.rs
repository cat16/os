use super::spin::SpinLock;

struct Mutex<T> {
    lock: SpinLock,
    val: T,
}

impl<T> Mutex<T> {
    pub fn new(val: T) -> Self {
        Self {
            lock: SpinLock::new(),
            val,
        }
    }
    pub fn lock(&mut self) -> MutexGuard<T> {
        self.lock.lock();
        MutexGuard { mutex: self }
    }
}

struct MutexGuard<'a, T> {
    mutex: &'a mut Mutex<T>,
}

impl <'a, T> Drop for MutexGuard<'a, T> {
    fn drop(&mut self) {
        self.mutex.lock.unlock();
    }
}
