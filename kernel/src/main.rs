#![no_std]
#![no_main]

use kernel::{framebuffer, init, exit, println, hlt_loop};
bootloader_api::entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    init();
    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
        framebuffer::draw_test(framebuffer);
    }
    for _ in 0..20000000 {}
    hlt_loop();
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    exit()
}
