use core::arch::asm;

pub fn wfi() {
    unsafe { asm!("wfi") }
}

macro_rules! reg {
    ($name:expr) => {{
        let out: u64;
        unsafe {
            core::arch::asm!(concat!("mv {out}, ", $name), out = out(reg) out);
        }
        out
    }};
}
pub(crate) use reg;
