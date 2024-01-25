use core::mem::transmute;

use alloc::vec;

use crate::println;

pub struct FDT {
    pub header: FDTHeader,
}

const MAGIC: u32 = 0xd00dfeed;

#[repr(u32)]
#[derive(Clone, Copy, Debug)]
enum Token {
    BeginNode = 0x00000001,
    EndNode = 0x00000002,
    Prop = 0x00000003,
    Nop = 0x00000004,
    End = 0x00000009,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct FDTHeader {
    pub magic: u32,
    pub totalsize: u32,
    pub off_dt_struct: u32,
    pub off_dt_strings: u32,
    pub off_mem_rsvmap: u32,
    pub version: u32,
    pub last_comp_version: u32,
    pub boot_cpuid_phys: u32,
    pub size_dt_strings: u32,
    pub size_dt_struct: u32,
}

const HEADER_SIZE: usize = core::mem::size_of::<FDTHeader>();

impl FDT {
    pub fn new(addr: u64) -> Self {
        let header: FDTHeader = unsafe { transmute(from_be_32::<HEADER_SIZE>(addr)) };
        if header.magic != MAGIC {
            panic!("FDT magic incorrect");
        }
        if header.version != 17 {
            panic!("FDT version not implemented {}", header.version);
        }
        let dt_structs = addr + header.off_dt_struct as u64;
        let first_node: Token = unsafe { transmute(from_be_32::<4>(dt_structs)) };
        println!("{first_node:?}");
        let a = vec![1, 2];
        println!("arst{a:?}");
        Self { header }
    }
}

pub unsafe fn from_be_32<const S: usize>(addr: u64) -> [u8; S] {
    let mut data = *(addr as *mut [u8; S]);
    for slice in data.chunks_mut(4) {
        slice.reverse();
    }
    data
}
