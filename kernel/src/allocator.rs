use linked_list_allocator::LockedHeap;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub const HEAP_START: *mut u8 = 0x9000_0000 as *mut u8;
pub const HEAP_SIZE: usize = 100 * 1024;

pub fn init_heap() {
    unsafe {
        ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
    }
}
