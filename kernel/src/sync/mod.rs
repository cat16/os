// use core::sync::atomic::AtomicBool;
//
// struct SpinLock(AtomicBool);
//
// impl SpinLock {
//     pub fn new() -> Self {
//         Self(AtomicBool::new(false))
//     }
//     pub fn lock(&mut self) {
//         while self.0.swap(true, core::sync::atomic::Ordering::Acquire) {}
//     }
//     pub fn release(&mut self) {
//         self.0.store(false, core::sync::atomic::Ordering::Release);
//     }
// }
//
// struct Mutex<T> {
//     lock: SpinLock,
//     data: T
// }
//
// struct MutexGuard<T>(T);
//
// impl <T> MutexGuard<T> {
// }
//
