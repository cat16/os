#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(naked_functions)]
#![feature(fn_align)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

use crate::mem::alloc::ALLOCATOR;
use core::ops::Range;
use fdt::{DT, FDT};

extern crate alloc;

pub mod arch;
pub mod dev;
pub mod fdt;
pub mod log;
pub mod mem;
pub mod qemu;
#[cfg(test)]
mod test;
pub mod util;

pub fn start(heap_mem: Range<*mut u8>, fdt: FDT) -> ! {
    DT.init(fdt);
    unsafe {
        ALLOCATOR.init(heap_mem);
    }
    #[cfg(test)]
    test_main();
    #[cfg(not(test))]
    main();
    qemu::exit(0)
}

pub fn main() {
    println!("we out here vibin");
}

#[panic_handler]
pub fn panic(info: &core::panic::PanicInfo) -> ! {
    #[cfg(test)]
    crate::test::test_panic(info);
    #[cfg(not(test))]
    main_panic(info);
}

pub fn main_panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    qemu::exit(1);
}
