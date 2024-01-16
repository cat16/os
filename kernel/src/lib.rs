#![no_std]
#![feature(abi_x86_interrupt)]

pub mod framebuffer;
pub mod gdt;
pub mod interrupts;
pub mod qemu;
pub mod log;

pub fn init() {
    gdt::init();
    interrupts::init();
}

pub fn exit() -> ! {
    qemu::exit();
    hlt_loop()
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

