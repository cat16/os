#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(naked_functions)]
#![feature(fn_align)]

use fdt::FDT;

extern crate alloc;

pub mod allocator;
pub mod arch;
pub mod fdt;
pub mod log;
pub mod qemu;
pub mod util;

pub fn main(dt_addr: *mut FDT) -> ! {
    println!("we out here vibin");
    allocator::init_heap();
    let mut test = alloc::vec![1, 2, 3];
    test.push(3);
    println!("{:?}", test);
    // for _ in 0..40000000 {}
    // let x = unsafe { *(0x10000000000 as *mut u8) };
    // println!("we got {x}");
    qemu::exit();
}

#[panic_handler]
pub fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    qemu::exit()
}
