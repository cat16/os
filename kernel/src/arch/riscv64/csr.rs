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

macro_rules! csrw {
    ($name:expr, $func:path) => {
        unsafe {
            core::arch::asm!(
                "la t0, {func}",
                concat!("csrw ", $name, ", t0"),
                func = sym $func,
            );
        }
    };
    ($name:expr, $val:expr) => {
        unsafe {
            core::arch::asm!(
                concat!("csrw ", $name, ", {val}"),
                val = in(reg) $val
            );
        }
    };
}
pub(crate) use csrw;

pub mod hartid {
    pub fn read() -> u64 {
        csrr!("mhartid")
    }
}

pub mod mtvec {
    macro_rules! init {
        ($func:path) => {
            let _: fn() -> ! = $func;
            crate::arch::csr::csrw!("mtvec", $func);
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
    pub fn read() -> u64 {
        csrr!("satp")
    }
}
