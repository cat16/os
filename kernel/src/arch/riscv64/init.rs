use crate::{
    arch::{asm, csr, interrupts, paging, wait}, main, println
};

#[no_mangle]
#[link_section = ".text.init"]
#[naked]
unsafe extern "C" fn _start() -> ! {
    core::arch::asm!(
        // set up gp & sp
        ".option push",
        ".option norelax",
        "la gp, _global_pointer",
        "la sp, _stack_end",
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
    let dt_addr = asm::reg!("a1") as usize;
    let hart = csr::hartid::read();
    println!("yo from hart {hart}");
    if hart != 0 {
        wait();
    }
    interrupts::init();
    paging::init(dt_addr);
    println!(
        "machine trap vector base address: 0x{:x}",
        csr::mtvec::read()
    );
    main(dt_addr)
}
