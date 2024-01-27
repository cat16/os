// NOTE: basically none of this is safe rn, ideally it's eventually made safe / able to recover

use crate::{print, println};
use core::{
    mem::{size_of, transmute},
    slice,
};

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
    pub fn from_bytes(data: &[u8]) -> Self {
        unsafe { transmute(be_32::<TOKEN_SIZE>(data)) }
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
    pub fn from_bytes(data: &[u8]) -> Self {
        unsafe { transmute(be_32::<PROP_SIZE>(data)) }
    }
}

pub struct FDT {
    pub header: Header,
}

impl FDT {
    pub fn new(addr: usize) -> Self {
        let header = Header::from_addr(addr);
        Self { header }
    }
}

pub struct RawFDT {
    pub header: Header,
    pub nodes: &'static [u8],
    pub strings: &'static [u8],
}

impl RawFDT {
    pub fn new(addr: usize) -> Self {
        let header = Header::from_addr(addr);
        let data = unsafe { slice::from_raw_parts(addr as *mut u8, header.totalsize as usize) };
        Self {
            header,
            nodes: &data[header.off_dt_struct as usize..],
            strings: &data[header.off_dt_strings as usize..],
        }
    }
    pub fn print_all(&self) {
        let mut pos = self.nodes;
        loop {
            let token: Token = Token::from_bytes(pos);
            pos = &pos[4..];
            if let Token::End = token {
                break;
            }
            if let Token::EndNode = token {
                continue;
            }
            print!("name: ");
            'outer: loop {
                let bytes = &pos[..4];
                pos = &pos[4..];
                for byte in bytes {
                    if *byte == 0 {
                        break 'outer;
                    }
                    let c = *byte as char;
                    print!("{}", c);
                }
            }
            println!();
            let node = RawNode {
                data: pos,
                strings: self.strings,
            };
            pos = node.print_props();
        }
    }

    pub fn find_node(&self, str: &str) -> Option<RawNode> {
        let mut pos = self.nodes;
        loop {
            let token: Token = Token::from_bytes(pos);
            pos = &pos[4..];
            if let Token::End = token {
                break None;
            }
            if let Token::EndNode = token {
                continue;
            }
            let mut i = 0;
            let check = str.as_bytes();
            let mut failed = false;
            'outer: loop {
                let bytes = &pos[..4];
                pos = &pos[4..];
                for byte in bytes {
                    if *byte == 0 {
                        break 'outer;
                    }
                    if i < check.len() && check[i] != *byte {
                        failed = true;
                    }
                    i += 1;
                }
            }
            let node = RawNode {
                data: pos,
                strings: self.strings,
            };
            if !failed && i >= check.len() {
                return Some(node);
            }
            pos = node.pass_props();
        }
    }
}

pub struct RawNode {
    pub strings: &'static [u8],
    pub data: &'static [u8],
}

impl RawNode {
    pub fn get_prop(&self, str: &str) -> Option<&'static [u8]> {
        let mut pos = self.data;
        loop {
            let token: Token = Token::from_bytes(pos);
            let Token::Prop = token else {
                break None;
            };
            pos = &pos[4..];
            let prop: Prop = Prop::from_bytes(pos);
            let name_bytes = &self.strings[prop.nameoff as usize..];
            let mut i = 0;
            let mut failed = false;
            let check = str.as_bytes();
            for byte in name_bytes {
                if *byte == 0 {
                    break;
                }
                if i < check.len() && check[i] != *byte {
                    failed = true;
                }
                i += 1;
            }
            pos = &pos[PROP_SIZE..];
            if !failed && i >= check.len() {
                return Some(&pos[..prop.len as usize]);
            }
            let len = (prop.len as usize + (TOKEN_SIZE - 1)) & !(TOKEN_SIZE - 1);
            pos = &pos[len..];
        }
    }
    pub fn print_props(&self) -> &'static [u8] {
        let mut pos = self.data;
        loop {
            let token: Token = Token::from_bytes(pos);
            let Token::Prop = token else {
                break;
            };
            pos = &pos[4..];
            let prop: Prop = Prop::from_bytes(pos);
            let name_bytes = &self.strings[prop.nameoff as usize..];
            print!("    ");
            for byte in name_bytes {
                if *byte == 0 {
                    break;
                }
                let c = *byte as char;
                print!("{}", c);
            }
            println!(": {prop:?}");
            let aligned_len = (prop.len as usize + (TOKEN_SIZE - 1)) & !(TOKEN_SIZE - 1);
            pos = &pos[PROP_SIZE + aligned_len..];
        }
        pos
    }
    pub fn pass_props(&self) -> &'static [u8] {
        let mut pos = self.data;
        loop {
            let token: Token = Token::from_bytes(pos);
            let Token::Prop = token else {
                break;
            };
            pos = &pos[4..];
            let prop: Prop = Prop::from_bytes(pos);
            let aligned_len = (prop.len as usize + (TOKEN_SIZE - 1)) & !(TOKEN_SIZE - 1);
            pos = &pos[PROP_SIZE + aligned_len..];
        }
        pos
    }
}

pub fn print_mem_layout(addr: usize) {
    let fdt = RawFDT::new(addr);
    fdt.print_all();
    if let Some(node) = fdt.find_node("memory") {
        println!("mem:");
        let data = node.get_prop("reg");
        if let Some(data) = data {
            for d in data.chunks(8) {
                let mut arr: [u8; 8] = d.try_into().unwrap();
                let num: u64 = unsafe { transmute(arr) };
                println!("0x{:x}", num.to_be());
            }
        }
    }
}

pub unsafe fn from_be_32<const S: usize>(addr: usize) -> [u8; S] {
    let mut data = *(addr as *mut [u8; S]);
    for slice in data.chunks_mut(4) {
        slice.reverse();
    }
    data
}

pub unsafe fn be_32<const S: usize>(data: &[u8]) -> [u8; S] {
    let data: &[u8; S] = data[..S].try_into().unwrap();
    let mut data = (*data).clone();
    for slice in data.chunks_mut(4) {
        slice.reverse();
    }
    data
}
