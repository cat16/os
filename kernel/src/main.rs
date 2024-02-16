#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(naked_functions)]
#![feature(fn_align)]

use core::ops::Range;

use fdt::FDT;

extern crate alloc;

pub mod allocator;
pub mod arch;
pub mod fdt;
pub mod log;
pub mod qemu;
pub mod util;
pub mod dev;

pub fn main(heap_mem: Range<*mut u8>, fdt: FDT) -> ! {
    println!("we out here vibin");
    println!("memory range: {:?}", fdt.mem_range());
    println!("heap range: {:?}", heap_mem);
    for node in &fdt {

    }
    allocator::init_heap(heap_mem);
    let mut test = alloc::vec![1, 2, 3];
    test.push(3);
    println!("{:?}", test);
    unsafe {
        let x = *(0x3000 as *const u8);
        println!("{}", x);
    }
    for i in 0..10000 {
        let test2: alloc::vec::Vec<i32> = alloc::vec::Vec::with_capacity(10_000_000);
        println!("{}", i);
    }
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
