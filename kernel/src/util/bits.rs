use core::{
    fmt::{Debug, Display},
    mem::transmute,
};

macro_rules! get_bits {
    ($name:ident[$high:expr,$low:expr]) => {{
        ($name & ((($name - $name + 2).pow($high - $low + 1) - 1) << $low)) >> $low
    }};
}
pub(crate) use get_bits;

pub trait BeRep {
    fn _from_be(&self) -> Self;
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Be<T: BeRep>(T);
impl<T: BeRep> Be<T> {
    pub fn get(&self) -> T {
        self.0._from_be()
    }
}

impl BeRep for u32 {
    fn _from_be(&self) -> Self {
        self.to_be()
    }
}

impl BeRep for u64 {
    fn _from_be(&self) -> Self {
        self.to_be()
    }
}

impl BeRep for usize {
    fn _from_be(&self) -> Self {
        self.to_be()
    }
}

impl BeRep for *mut u8 {
    fn _from_be(&self) -> Self {
        (*self as usize).to_be() as *mut u8
    }
}

impl<T: Debug + BeRep> Debug for Be<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self.get())
    }
}

impl<T: Display + BeRep> Display for Be<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.get())
    }
}

pub fn u32_from_bytes(bytes: &[u8]) -> Option<u32> {
    if bytes.len() < 4 {
        return None;
    }
    unsafe { Some(transmute([bytes[0], bytes[1], bytes[2], bytes[3]])) }
}
