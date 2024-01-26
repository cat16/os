// NOTE: basically none of this is safe rn, ideally it's eventually made safe / able to recover

use crate::{print, println};
use alloc::vec;
use core::mem::{size_of, transmute};

pub struct FDT {
    pub header: Header,
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
const TOKEN_SIZE: usize = size_of::<Token>();
impl Token {
    pub fn from_addr(addr: usize) -> Self {
        unsafe { transmute(from_be_32::<TOKEN_SIZE>(addr)) }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Header {
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

const HEADER_SIZE: usize = size_of::<Header>();
impl Header {
    pub fn from_addr(addr: usize) -> Self {
        unsafe { transmute(from_be_32::<HEADER_SIZE>(addr)) }
    }
}

const PROP_SIZE: usize = size_of::<Prop>();
#[derive(Debug)]
pub struct Prop {
    len: u32,
    nameoff: u32,
}

impl Prop {
    pub fn from_addr(addr: usize) -> Self {
        unsafe { transmute(from_be_32::<PROP_SIZE>(addr)) }
    }
}

impl FDT {
    pub fn new(addr: usize) -> Self {
        let header = Header::from_addr(addr);
        if header.magic != MAGIC {
            panic!("FDT magic incorrect");
        }
        if header.version != 17 {
            panic!("FDT version not implemented {}", header.version);
        }
        let dt_structs = addr + header.off_dt_struct as usize;
        let first_node: Token = Token::from_addr(dt_structs);
        println!("{first_node:?}");
        let a = vec![1, 2];
        println!("arst{a:?}");
        Self { header }
    }
}

pub unsafe fn from_be_32<const S: usize>(addr: usize) -> [u8; S] {
    let mut data = *(addr as *mut [u8; S]);
    for slice in data.chunks_mut(4) {
        slice.reverse();
    }
    data
}

pub fn print_fdt(addr: usize) {
    let header = Header::from_addr(addr);
    let str_addr = header.off_dt_strings as usize + addr;
    let mut addr = header.off_dt_struct as usize + addr;
    loop {
        let token: Token = Token::from_addr(addr);
        addr += TOKEN_SIZE;
        if let Token::End = token {
            break;
        }
        if let Token::EndNode = token {
            continue;
        }
        print!("name: ");
        'outer: loop {
            let bytes = unsafe { *(addr as *mut [u8; TOKEN_SIZE]) };
            addr += TOKEN_SIZE;
            for byte in bytes {
                if byte == 0 {
                    break 'outer;
                }
                let c = byte as char;
                print!("{}", c);
            }
        }
        println!();
        print_props(str_addr, &mut addr)
    }
}

pub fn print_props(str_addr: usize, addr: &mut usize) {
    loop {
        let token: Token = Token::from_addr(*addr);
        let Token::Prop = token else {
            break;
        };
        *addr += TOKEN_SIZE;
        let prop: Prop = Prop::from_addr(*addr);
        let mut name_addr = str_addr + prop.nameoff as usize;
        print!("    ");
        loop {
            let byte = unsafe { *(name_addr as *mut u8) };
            name_addr += 1;
            if byte == 0 {
                break;
            }
            let c = byte as char;
            print!("{}", c);
        }
        println!(": {prop:?}");
        let aligned_len = (prop.len as usize + (TOKEN_SIZE - 1)) & !(TOKEN_SIZE - 1);
        *addr += PROP_SIZE + aligned_len;
    }
}
