use core::alloc::Allocator;

#[global_allocator]
static ALLOCATOR: Alloc = Alloc::empty();

struct Alloc {
    base: *mut u8,
    cur: *mut u8,
}

unsafe impl Allocator for Alloc {
    fn allocate(&self, layout: core::alloc::Layout) -> Result<core::ptr::NonNull<[u8]>, core::alloc::AllocError> {
        
    }
}

impl Alloc {
    pub fn empty() -> Self {
    }

    pub fn init(&mut self, start: *mut u8, len: usize) {

    }
}

pub fn init_heap() {
    unsafe {
        ALLOCATOR.init(crate::arch::paging::first_free(), crate::arch::paging::free_len());
    }
}
