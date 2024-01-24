#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

pub mod arch;
pub mod log;
pub mod qemu;

pub fn main() -> ! {
    println!("we out here vibin");
    for _ in 0..20000000 {}
    qemu::exit();
}

#[panic_handler]
pub fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    qemu::exit()
}