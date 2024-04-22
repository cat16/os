use super::heap::Heap;
use crate::util::mutex::{Mutex, MutexGuard};
use core::{alloc::GlobalAlloc, ops::Range};

#[global_allocator]
pub static ALLOCATOR: Allocator = Allocator::empty();

pub struct Allocator(Mutex<Heap>);

// should look into why I need this, didn't see it in linked list alloc crate
unsafe impl Sync for Allocator {}

impl Allocator {
    pub const fn empty() -> Self {
        Self(Mutex::new(Heap::empty()))
    }
    pub unsafe fn init(&self, range: Range<*mut u8>) {
        self.0.lock().init(range);
    }
    pub fn print(&self) {
        self.0.lock().print();
    }
    pub fn heap(&self) -> MutexGuard<Heap> {
        self.0.lock()
    }
}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        self.0.lock().alloc(layout)
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        self.0.lock().dealloc(ptr, layout)
    }
}
