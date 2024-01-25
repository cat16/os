#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(naked_functions)]
#![feature(fn_align)]

extern crate alloc;

pub mod allocator;
pub mod arch;
pub mod fdt;
pub mod log;
pub mod qemu;

pub fn main(dt_addr: u64) -> ! {
    println!("we out here vibin");
    allocator::init_heap();
    let fdt = fdt::FDT::new(dt_addr);
    // for _ in 0..40000000 {}
    let x = unsafe { *(0xdeadbeef as *mut u8) };
    println!("we got {x}");
    qemu::exit();
}

#[panic_handler]
pub fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    qemu::exit()
}
