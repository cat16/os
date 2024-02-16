use core::{alloc::GlobalAlloc, ops::Range, ptr::null_mut};

use crate::util::mutex::Mutex;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

struct Heap {
    cur: *mut u8,
    end: *mut u8,
}

impl Heap {
    pub const fn empty() -> Self {
        Self {
            cur: null_mut(),
            end: null_mut(),
        }
    }

    pub fn init(&mut self, start: *mut u8, end: *mut u8) {
        self.cur = start;
        self.end = end;
    }
}

pub fn init_heap(range: Range<*mut u8>) {
    ALLOCATOR.init(range.start, range.end);
}

struct LockedHeap(Mutex<Heap>);

// should look into why I need this, didn't see it in linked list alloc crate
unsafe impl Sync for LockedHeap {}

impl LockedHeap {
    pub const fn empty() -> Self {
        Self(Mutex::new(Heap::empty()))
    }
    pub fn init(&self, start: *mut u8, end: *mut u8) {
        self.0.lock().init(start, end);
    }
}

unsafe impl GlobalAlloc for LockedHeap {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        // blazing fast implementation :sunglasses:
        // (gonna switch to my own linked list later)
        let mut heap = self.0.lock();
        let pointer = heap.cur;
        heap.cur = heap.cur.add(layout.size());
        if heap.cur >= heap.end {
            return null_mut();
        }
        return pointer;
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        // bet ur impl is slower
    }
}
