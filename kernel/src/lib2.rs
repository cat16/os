#![no_std]
#![feature(abi_x86_interrupt)]

pub mod arch;
pub mod log;
pub mod qemu;

pub fn main() -> ! {
    println!("we out here vibin");
    for _ in 0..20000000 {}
    qemu::exit();
}

pub fn exit() -> ! {
    qemu::exit();
}

pub fn hlt_loop() -> ! {
    arch::hlt_loop();
}

#[panic_handler]
pub fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    exit()
}
