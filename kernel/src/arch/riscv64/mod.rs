use crate::main;

pub mod qemu;

#[no_mangle]
#[link_section = ".text.init"]
pub extern "C" fn _start() -> ! {
    unsafe {
        core::arch::asm!(
            "csrr    t0, mhartid",
            "bnez    t0, {_start}",
            _start = sym _start
        );
        core::arch::asm!(
            ".option push",
            ".option norelax",

            "la gp, global_pointer",
            "la sp, stack_top",

            "tail {entry}",
            entry = sym entry,
            options(noreturn)
        );
    }
}

extern "C" fn entry() -> ! {
    main()
}

pub fn hlt_loop() -> ! {
    loop {}
}
