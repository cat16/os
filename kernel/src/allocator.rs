use core::{alloc::GlobalAlloc, ops::Range};

use crate::{heap::Heap, util::mutex::Mutex};

#[global_allocator]
pub static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub struct LockedHeap(Mutex<Heap>);

// should look into why I need this, didn't see it in linked list alloc crate
unsafe impl Sync for LockedHeap {}

impl LockedHeap {
    pub const fn empty() -> Self {
        Self(Mutex::new(Heap::empty()))
    }
    pub unsafe fn init(&self, range: Range<*mut u8>) {
        self.0.lock().init(range);
    }
    pub fn print(&self) {
        self.0.lock().print();
    }
}

unsafe impl GlobalAlloc for LockedHeap {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        self.0.lock().alloc(layout)
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        self.0.lock().dealloc(ptr, layout)
    }
}
