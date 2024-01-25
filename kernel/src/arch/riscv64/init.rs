use crate::{
    arch::{csr, instructions, interrupts, wait}, main, println
};

#[no_mangle]
#[link_section = ".text.init"]
#[naked]
unsafe extern "C" fn _start() -> ! {
    core::arch::asm!(
        // set up gp & sp
        ".option push",
        ".option norelax",
        "la gp, global_pointer",
        "la sp, stack_top",
        ".option pop",
        // set up stack for each hart
        "csrr t0, mhartid",
        "slli t0, t0, 12",
        "sub sp, sp, t0",
        // continue to rest of program
        "tail {entry}",

        entry = sym entry,
        options(noreturn)
    );
}

pub fn entry() -> ! {
    let dt_addr = instructions::reg!("a1");
    let hart = csr::hartid::read();
    println!("yo from hart {hart}");
    if hart != 0 {
        wait();
    }
    interrupts::init();
    println!(
        "machine trap vector base address: 0x{:x}",
        csr::mtvec::read()
    );
    println!(
        "physical address bits: {}",
        csr::satp::read()
    );
    main(dt_addr)
}
