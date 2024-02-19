use core::{
    mem::{size_of, transmute},
    ops::{Deref, DerefMut},
    ptr::null_mut,
};

pub const FREE_SIZE: usize = size_of::<FreeBlockInfo>() + size_of::<FreePointer>();
pub const PTR_SIZE: usize = size_of::<FreePointer>();
pub const USED_SIZE: usize = size_of::<BlockInfo>();

pub struct BlockInfo(usize);

impl BlockInfo {
    pub const fn new(prev_used: bool, size: usize) -> Self {
        Self(prev_used as usize | size)
    }
    pub fn prev_used(&self) -> bool {
        self.0 & 1 == 1
    }
    pub fn size(&self) -> usize {
        self.0 & !1
    }
}

pub struct UsedPointer(*mut BlockInfo);

impl Deref for UsedPointer {
    type Target = BlockInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { transmute(self.0) }
    }
}

impl DerefMut for UsedPointer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { transmute(self.0) }
    }
}

#[derive(Clone, Copy)]
pub struct FreeBlockInfo {
    pub size: usize,
    pub prev: FreePointer,
    pub next: FreePointer,
}

impl FreeBlockInfo {
    pub fn pointer(&mut self) -> FreePointer {
        FreePointer(self as *mut FreeBlockInfo)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct FreePointer(*mut FreeBlockInfo);

impl FreePointer {
    pub const fn null() -> Self {
        Self(null_mut())
    }
    pub unsafe fn new(addr: *mut u8, info: FreeBlockInfo) -> Self {
        let ptr = Self(addr as *mut FreeBlockInfo);
        let len = info.size;
        *ptr.0 = info;
        let end = addr.byte_add(len).byte_sub(PTR_SIZE) as *mut FreePointer;
        *end = ptr;
        Self(addr as *mut FreeBlockInfo)
    }
    pub fn to_used(mut self) -> *mut BlockInfo {
        self.prev.next = self.next;
        self.next.prev = self.prev;
        self.0 as *mut BlockInfo
    }
    pub unsafe fn insert_new(&mut self, len: usize) -> *mut BlockInfo {
        let old = self.0;
        let new = old.byte_add(len);
        *new = *old;
        self.0 = new;
        self.size = self.size - len;

        self.prev.next = *self;
        self.next.prev = *self;

        old as *mut BlockInfo
    }
}

impl Deref for FreePointer {
    type Target = FreeBlockInfo;
    fn deref(&self) -> &Self::Target {
        unsafe { transmute(self.0) }
    }
}

impl DerefMut for FreePointer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { transmute(self.0) }
    }
}

pub struct FreeBlockIter {
    end: FreePointer,
    prev: FreePointer,
}

impl Iterator for FreeBlockIter {
    // you know, I could've returned &'static mut FreeBlockInfo...
    // that feels too wrong though
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

impl FreeBlockIter {
    pub fn from(head: &mut FreeBlockInfo) -> Self {
        Self {
            end: FreePointer(head),
            prev: FreePointer(head),
        }
    }
}
