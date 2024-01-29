use linked_list_allocator::LockedHeap;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub fn init_heap() {
    unsafe {
        ALLOCATOR.lock().init(crate::arch::paging::first_free(), crate::arch::paging::free_len());
    }
}
