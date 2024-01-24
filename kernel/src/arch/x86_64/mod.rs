use crate::main;

pub mod framebuffer;
pub mod gdt;
pub mod interrupts;
pub mod qemu;

bootloader_api::entry_point!(_start);

fn _start(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    gdt::init();
    interrupts::init();
    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
        framebuffer::draw_test(framebuffer);
    }
    main();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
