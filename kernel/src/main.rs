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
use dev::fdt::FDT;

extern crate alloc;

pub mod arch;
pub mod dev;
pub mod log;
pub mod mem;
pub mod qemu;
#[cfg(test)]
mod test;
pub mod util;

pub struct StartInfo {
    mem_range: Range<*mut u8>,
    dt: FDT
}

pub fn start(info: StartInfo) -> ! {
    #[cfg(test)]
    {
        // un... un bro momento..
        test::init(info);
        test_main();
    }
    #[cfg(not(test))]
    main(info);
    qemu::exit(0)
}

pub fn main(info: StartInfo) {
    println!("we out here vibin");
    unsafe {
        ALLOCATOR.init(&info.mem_range);
    }
    for dev in info.dt.into_iter() {
        println!("{:?}", dev);
    }
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
