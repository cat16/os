use core::{mem::size_of, ops::Range, ptr::null_mut};

// Heap stolen from my own riscv userspace implementation;
// uses a doubly linked list of free blocks
//
// I tried to do this with idomatic rust (wrapper types)
// but it turned out to just be cursed and messy bc
// it's all unsafe raw pointers in the end :pensive:
//
// might try again later, but it'll probably just make it
// slower and not much safer so

use crate::println;

const ALIGN: usize = 0b1000;
const ALIGN_MASK: usize = !(ALIGN - 1);

pub struct BlockInfo(usize);

impl BlockInfo {
    pub const fn new(prev_used: bool, size: usize) -> Self {
        Self(prev_used as usize | size)
    }
    pub fn prev_used(&self) -> bool {
        self.0 & 1 == 1
    }
    pub fn set_prev_used(&mut self) {
        self.0 |= 1;
    }
    pub fn unset_prev_used(&mut self) {
        self.0 &= !1;
    }
    pub fn size(&self) -> usize {
        self.0 & !1
    }
}

pub type BlockPointer = *mut BlockInfo;

pub struct FreeBlockInfo {
    info: BlockInfo,
    prev: FreePointer,
    next: FreePointer,
}

impl FreeBlockInfo {
    pub fn prev_used(&self) -> bool {
        self.info.prev_used()
    }
    pub fn size(&self) -> usize {
        self.info.size()
    }
}

pub type FreePointer = *mut FreeBlockInfo;

const FREE_SIZE: usize = size_of::<FreeBlockInfo>() + size_of::<FreePointer>();
const PTR_SIZE: usize = size_of::<FreePointer>();
const USED_SIZE: usize = size_of::<BlockInfo>();

pub struct Heap {
    head: FreeBlockInfo,
    start: *mut u8,
    end: *mut u8,
}

impl Heap {
    pub const fn empty() -> Self {
        Self {
            head: FreeBlockInfo {
                info: BlockInfo(0),
                prev: null_mut(),
                next: null_mut(),
            },
            start: null_mut(),
            end: null_mut(),
        }
    }

    pub unsafe fn init(&mut self, range: Range<*mut u8>) {
        let head = self.head();
        let first = range.start as FreePointer;
        let size = range.end as usize - range.start as usize;
        create_free(
            first,
            FreeBlockInfo {
                info: BlockInfo::new(true, size),
                next: head,
                prev: head,
            },
        );
        self.head.next = first;
        self.head.prev = first;
        self.start = range.start;
        self.end = range.end;
    }

    pub unsafe fn alloc(&mut self, layout: core::alloc::Layout) -> *mut u8 {
        // calc aligned size
        let mut size = layout.size() + USED_SIZE;
        // does this matter? idr, copying from my riscv userspace impl
        size = ((size - 1) & ALIGN_MASK) + ALIGN;
        if size < FREE_SIZE {
            size = FREE_SIZE;
        }
        // search for free block w enough space
        for free in self.iter_free() {
            let free_size = (*free).info.size();
            // free block found
            if free_size >= size {
                // deal with leftover space
                let leftover = free_size - size;
                if leftover < FREE_SIZE {
                    size = free_size;
                    let mut next_used = free.byte_add(size) as BlockPointer;
                    if next_used as *mut u8 == self.end {
                        next_used = &mut self.head.info;
                    }
                    (*next_used).set_prev_used();
                    let prev = (*free).prev;
                    let next = (*free).next;
                    (*prev).next = next;
                    (*next).prev = prev;
                } else {
                    let new_free = free.byte_add(size);
                    let prev = (*free).prev;
                    let next = (*free).next;
                    create_free(
                        new_free,
                        FreeBlockInfo {
                            info: BlockInfo::new(true, leftover),
                            prev,
                            next,
                        },
                    );
                    (*prev).next = new_free;
                    (*next).prev = new_free;
                }
                // create block
                let used = free as BlockPointer;
                (*used) = BlockInfo::new(true, size);
                let data = used.byte_add(USED_SIZE) as *mut u8;
                return data;
            }
        }
        return null_mut();
    }

    pub unsafe fn dealloc(&mut self, ptr: *mut u8, _: core::alloc::Layout) {
        let used = ptr.byte_sub(USED_SIZE) as BlockPointer;
        let mut size = (*used).size();
        let old_size = size;
        let mut addr = used as FreePointer;
        let mut prev = self.head();
        let mut next = self.head.next;
        if !(*used).prev_used() {
            let prev_free = *(used.byte_sub(PTR_SIZE) as *mut FreePointer);
            addr = prev_free;
            size += (*prev_free).info.size();
            prev = (*prev_free).prev;
            next = (*prev_free).next;
        }
        let mut n_block = used.byte_add(old_size);
        if n_block as *mut u8 != self.end {
            let mut nn_block = n_block.byte_add((*n_block).size());
            if nn_block as *mut u8 == self.end {
                nn_block = &mut self.head.info;
            }
            if !(*nn_block).prev_used() {
                size += (*n_block).size();
                next = (*next).next;
            }
        }
        create_free(
            addr,
            FreeBlockInfo {
                info: BlockInfo::new(true, size),
                prev,
                next,
            },
        );
        (*prev).next = addr;
        (*next).prev = addr;
        if n_block as *mut u8 == self.end {
            n_block = &mut self.head.info;
        }
        (*n_block).unset_prev_used();
    }

    pub fn iter_free(&mut self) -> FreeBlockIter {
        FreeBlockIter {
            prev: &mut self.head,
            end: &mut self.head,
        }
    }
    pub fn iter_block(&mut self) -> BlockIter {
        BlockIter {
            cur: self.start as BlockPointer,
            end: self.end,
        }
    }

    fn head(&mut self) -> FreePointer {
        &mut self.head as FreePointer
    }

    pub fn print(&mut self) {
        unsafe {
            println!("heap: {:?} -> {:?}", self.start, self.end);
            for block in self.iter_block() {
                let size = (*block).size();
                let mut n_block = block.byte_add(size);
                if n_block as *mut u8 == self.end {
                    n_block = &mut self.head.info;
                }
                let used = if (*n_block).prev_used() {
                    "used"
                } else {
                    "free"
                };
                println!(" - {:?}: {}, size 0x{:x}", block, used, size);
            }
            println!();
        }
    }
}

unsafe fn create_free(addr: FreePointer, info: FreeBlockInfo) {
    let len = info.info.size();
    *addr = info;
    let end = addr.byte_add(len).byte_sub(PTR_SIZE) as *mut FreePointer;
    *end = addr;
}

pub struct FreeBlockIter {
    end: FreePointer,
    prev: FreePointer,
}

impl Iterator for FreeBlockIter {
    type Item = FreePointer;
    fn next(&mut self) -> Option<Self::Item> {
        self.prev = unsafe { (*self.prev).next };
        if self.prev == self.end {
            None
        } else {
            Some(self.prev)
        }
    }
}

pub struct BlockIter {
    end: *mut u8,
    cur: BlockPointer,
}

impl Iterator for BlockIter {
    type Item = BlockPointer;
    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let cur = self.cur;
            if cur as *mut u8 == self.end {
                return None;
            }
            let size = (*self.cur).size();
            self.cur = cur.byte_add(size);
            Some(cur)
        }
    }
}
