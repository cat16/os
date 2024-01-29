use core::arch::asm;

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

macro_rules! linker_static {
    ($name:ident: $type:ty, $source:expr) => {
        core::arch::global_asm!(
            concat!(".global ", stringify!($name)),
            concat!(stringify!($name), ": ", $source)
        );
        extern "C" {
            pub static $name: $type;
        }
    };
}
pub(crate) use linker_static;

pub fn wfi() {
    unsafe { asm!("wfi") }
}

pub unsafe fn mret() {
    asm!("mret");
}
