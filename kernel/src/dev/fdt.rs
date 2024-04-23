// garbage .1% finished FDT implementation

use alloc::format;

use crate::{
    println,
    util::bits::{u32_from_bytes, Be},
};
use core::{
    fmt::Debug,
    mem::{size_of, transmute},
    ops::Range,
    slice,
};

const MAGIC: u32 = 0xd00dfeed;

#[repr(u32)]
#[derive(Clone, Copy, Debug)]
enum Token {
    BeginNode,
    EndNode,
    Prop,
    Nop,
    End,
}

const TOKEN_SIZE: usize = size_of::<Token>();

impl Token {
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        let val = u32_from_bytes(bytes)?.to_be();
        Some(match val {
            0x00000001 => Token::BeginNode,
            0x00000002 => Token::EndNode,
            0x00000003 => Token::Prop,
            0x00000004 => Token::Nop,
            0x00000009 => Token::End,
            _ => {
                println!("Failed to parse token!");
                return None;
            }
        })
    }
}

#[repr(C)]
pub struct Header {
    pub magic: Be<u32>,
    pub totalsize: Be<u32>,
    pub off_dt_struct: Be<u32>,
    pub off_dt_strings: Be<u32>,
    pub off_mem_rsvmap: Be<u32>,
    pub version: Be<u32>,
    pub last_comp_version: Be<u32>,
    pub boot_cpuid_phys: Be<u32>,
    pub size_dt_strings: Be<u32>,
    pub size_dt_struct: Be<u32>,
}

const PROP_SIZE: usize = size_of::<RawProp>();
pub struct RawProp {
    len: Be<u32>,
    nameoff: Be<u32>,
}

pub struct Prop {
    pub name: &'static str,
    pub data: &'static [u8],
}

impl Debug for Prop {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Prop")
            .field("name", &self.name)
            .field("data_len", &self.data)
            .finish()
    }
}

impl Prop {
    pub fn full_len(&self) -> usize {
        return PROP_SIZE + self.data.len();
    }
}

pub struct FDT {
    pub header: &'static Header,
    pub nodes: &'static [u8],
    pub strings: &'static [u8],
}

impl IntoIterator for &FDT {
    type Item = Node;
    type IntoIter = NodeIter;
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            pos: self.nodes,
            strings: self.strings,
        }
    }
}

pub struct NodeIter {
    pub pos: &'static [u8],
    pub strings: &'static [u8],
}

impl Iterator for NodeIter {
    type Item = Node;
    fn next(&mut self) -> Option<Self::Item> {
        let token: Token = Token::from_bytes(self.pos)?;
        self.pos = &self.pos[4..];
        if let Token::End = token {
            return None;
        }
        let name_start = self.pos;
        'outer: loop {
            let bytes = &self.pos[..4];
            self.pos = &self.pos[4..];
            for byte in bytes {
                if *byte == 0 {
                    break 'outer;
                }
            }
        }
        let name = unsafe { transmute(&name_start[..name_start.len() - self.pos.len()]) };
        let node_start = self.pos;
        let node_data = if let Some(prop) = (PropIter {
            strings: self.strings,
            pos: self.pos,
        })
        .last()
        {
            let node_len =
                (prop.data.as_ptr() as usize + prop.data.len()) - self.pos.as_ptr() as usize;
            self.pos = &self.pos[node_len..];
            &node_start[..node_len]
        } else {
            &[]
        };
        let node = Node {
            name,
            props: node_data,
            strings: self.strings,
        };
        loop {
            if let Some(token) = Token::from_bytes(self.pos) {
                if let Token::EndNode = token {
                    self.pos = &self.pos[4..];
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        Some(node)
    }
}

impl FDT {
    pub fn from_addr(addr: *mut u8) -> Self {
        unsafe {
            let header: &Header = transmute(addr);
            if header.magic.get() != MAGIC {
                panic!("fdt magic wrong");
            }
            let data = slice::from_raw_parts(addr as *mut u8, header.totalsize.get() as usize);
            Self {
                header,
                nodes: &data[header.off_dt_struct.get() as usize..],
                strings: &data[header.off_dt_strings.get() as usize..],
            }
        }
    }
    pub fn mem_range(&self) -> FDTMemRange {
        if let Some(node) = self.into_iter().find(|n| n.name.starts_with("memory@")) {
            let prop = node.find_prop("reg");
            if let Some(prop) = prop {
                for d in prop.data.chunks(size_of::<FDTMemRange>()) {
                    let d: [u8; size_of::<FDTMemRange>()] = d.try_into().unwrap();
                    // just return first one for now
                    return unsafe { transmute(d) };
                }
            }
        }
        panic!("failed to get memory range");
    }
}

#[repr(C)]
pub struct FDTMemRange {
    pub start: Be<*mut u8>,
    pub len: Be<usize>,
}

impl FDTMemRange {
    pub fn start(&self) -> *mut u8 {
        self.start.get()
    }
    pub fn len(&self) -> usize {
        self.len.get()
    }
    pub fn end(&self) -> *mut u8 {
        unsafe { self.start().add(self.len.get()) }
    }
}

impl Debug for FDTMemRange {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}..{:?}", self.start(), self.end())
    }
}

impl Into<Range<*mut u8>> for FDTMemRange {
    fn into(self) -> Range<*mut u8> {
        Range {
            start: self.start(),
            end: self.end(),
        }
    }
}

pub struct Node {
    pub name: &'static str,
    pub strings: &'static [u8],
    pub props: &'static [u8],
}

impl Debug for Node {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let props: alloc::vec::Vec<_> = self.into_iter().map(|p| format!("{:?}", p)).collect();
        f.debug_struct("Node")
            .field("name", &self.name)
            .field("props", &props)
            .finish()
    }
}

impl IntoIterator for &Node {
    type Item = Prop;
    type IntoIter = PropIter;
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            pos: self.props,
            strings: self.strings,
        }
    }
}

pub struct PropIter {
    pub strings: &'static [u8],
    pub pos: &'static [u8],
}

impl Iterator for PropIter {
    type Item = Prop;
    fn next(&mut self) -> Option<Self::Item> {
        let token: Token = Token::from_bytes(self.pos)?;
        let Token::Prop = token else {
            return None;
        };
        self.pos = &self.pos[4..];
        let prop: &RawProp = unsafe { transmute(self.pos.as_ptr()) };
        self.pos = &self.pos[PROP_SIZE..];
        let plen = prop.len.get() as usize;
        let len = (plen + (TOKEN_SIZE - 1)) & !(TOKEN_SIZE - 1);
        let data = &self.pos[..len];
        self.pos = &self.pos[len..];
        let name_start = &self.strings[prop.nameoff.get() as usize..];
        for (i, c) in name_start.iter().enumerate() {
            if *c == 0 {
                let name: &str = unsafe { transmute(&name_start[..i]) };
                return Some(Prop { name, data });
            }
        }
        println!("failed to read prop name, not sure what to do");
        None
    }
}

impl Node {
    pub fn find_prop(&self, name: &str) -> Option<Prop> {
        self.into_iter().find(|p| p.name == name)
    }
}
