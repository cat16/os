macro_rules! csrr {
    ($name:expr) => {{
        let mut out: u64;
        unsafe {
            core::arch::asm!(
                concat!("csrr {out}, ", $name),
                out = out(reg) out,
            );
        }
        out
    }};
}

macro_rules! csrw_fn {
    ($name:expr, $func:path) => {
        unsafe {
            core::arch::asm!(
                "la t0, {func}",
                concat!("csrw ", $name, ", t0"),
                func = sym $func,
            );
        }
    };
}
pub(crate) use csrw_fn;

macro_rules! csrw {
    ($name:expr, $val:expr) => {
        unsafe {
            core::arch::asm!(
                concat!("csrw ", $name, ", {val}"),
                val = in(reg) $val
            );
        }
    };
}

macro_rules! bits {
    ($name:ident[$high:expr,$low:expr]) => {{
        ($name & ((2u64.pow($high - $low + 1) - 1) << $low)) >> $low
    }};
}

pub mod hartid {
    pub fn read() -> u64 {
        csrr!("mhartid")
    }
}

pub mod mtvec {
    macro_rules! init {
        ($func:path) => {
            let _: fn() -> ! = $func;
            crate::arch::csr::csrw_fn!("mtvec", $func);
        };
    }
    pub(crate) use init;
    pub fn read() -> u64 {
        csrr!("mtvec")
    }
}

pub mod mcause {
    pub fn read() -> u64 {
        csrr!("mcause")
    }
}

pub mod satp {
    use core::mem::transmute;

    use crate::arch::paging::Table;

    #[derive(Debug)]
    #[repr(u64)]
    pub enum Mode {
        Bare = 0,
        Reserved1 = 1,
        Reserved2 = 2,
        Reserved3 = 3,
        Reserved4 = 4,
        Reserved5 = 5,
        Reserved6 = 6,
        Reserved7 = 7,
        Sv39 = 8,
        Sv48 = 9,
        Sv57 = 10,
        Sv64 = 11,
        Reserved8 = 12,
        Reserved9 = 13,
        Custom1 = 14,
        Custom2 = 15,
    }
    #[derive(Debug)]
    pub struct Satp {
        pub mode: Mode,
        pub asid: u64,
        pub ppn: *mut Table,
    }
    pub fn read() -> Satp {
        let satp = csrr!("satp");
        let mode = unsafe { transmute(bits!(satp[63,60])) };
        let asid = bits!(satp[59, 44]);
        let ppn = unsafe { transmute(bits!(satp[43, 0]) << 12) };
        Satp { mode, asid, ppn }
    }
    pub fn write(satp: Satp) {
        let val = (satp.mode as u64) << 60 | satp.asid << 44 | (satp.ppn as u64 >> 12);
        csrw!("satp", val);
    }
}
