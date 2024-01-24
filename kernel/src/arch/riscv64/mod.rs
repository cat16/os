use crate::{main, println};

pub mod qemu;

#[no_mangle]
#[link_section = ".text.init"]
#[naked]
unsafe extern "C" fn _start() -> ! {
    core::arch::asm!(
        ".option push",
        ".option norelax",
        "la gp, global_pointer",
        "la sp, stack_top",
        ".option pop",

        "csrr a0, mhartid",
        "slli t0, a0, 12",
        "sub sp, sp, t0",
        "tail {entry}",

        entry = sym entry,
        options(noreturn)
    );
}

fn get_hartid() -> u64 {
    let mut hart: u64;
    unsafe {
        core::arch::asm!(
            "csrr {hart}, mhartid",
            hart = out(reg) hart
        );
    }
    return hart
}

fn entry() -> ! {
    let hart = get_hartid();
    println!("yo from hart {hart}");
    if hart != 0 {
        loop {}
    }
    main()
}

pub fn hlt_loop() -> ! {
    loop {}
}
