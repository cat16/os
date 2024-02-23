#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(naked_functions)]
#![feature(fn_align)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

use crate::allocator::ALLOCATOR;
use core::ops::Range;
use fdt::FDT;

extern crate alloc;

pub mod allocator;
pub mod arch;
pub mod dev;
pub mod fdt;
pub mod heap;
pub mod log;
pub mod qemu;
#[cfg(test)]
mod test;
pub mod util;

pub fn start(heap_mem: Range<*mut u8>, fdt: FDT) -> ! {
    #[cfg(test)]
    test_main();
    #[cfg(not(test))]
    main(heap_mem, fdt);
    qemu::exit(0)
}

pub fn main(heap_mem: Range<*mut u8>, fdt: FDT) {
    println!("we out here vibin");
    println!("memory range: {:?}", fdt.mem_range());
    for node in &fdt {}
    unsafe {
        ALLOCATOR.init(heap_mem);
    }
    ALLOCATOR.print();
    println!("----------- vec test:");
    let mut test = alloc::vec![1, 2, 3];
    test.push(3);
    let test2 = alloc::vec![-1, -2, -3, -4];
    ALLOCATOR.print();
    println!("{:?}", test);
    println!("{:?}", test2);
    drop(test2);
    drop(test);
    ALLOCATOR.print();
    println!("----------- vec vec test:");
    let mut test = alloc::vec::Vec::new();
    for i in 0..4 {
        let n = i * 4;
        test.push(alloc::vec![n, n + 1, n + 2, n + 3]);
    }
    ALLOCATOR.print();
    println!("{:?}", test);
    drop(test);
    ALLOCATOR.print();
    println!("----------- dealloc test:");
    for i in 0..1000 {
        let test2: alloc::vec::Vec<i32> = alloc::vec::Vec::with_capacity(10_000_0000);
    }
    ALLOCATOR.print();
    // for _ in 0..40000000 {}
    // let x = unsafe { *(0x10000000000 as *mut u8) };
    // println!("we got {x}");
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
