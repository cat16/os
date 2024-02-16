use core::{arch::asm, ops::Range};

use crate::{
    arch::{asm, csr, interrupts, paging, wait},
    fdt::FDT,
    main,
};

#[no_mangle]
#[link_section = ".text.init"]
#[naked]
unsafe extern "C" fn _start() -> ! {
    asm!(
        // disable interrupts
        "csrw mie, zero",
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
        // "tail {entry}",
        // ok so this should be done in rust
        "li	t0, 0b11 << 11",
        "csrw mstatus, t0",
        "la t0, {init}",
        "csrw mepc, t0",
        "mret",

        init = sym init,
        options(noreturn)
    );
}

pub unsafe fn to_supervisor() {
    asm!(
        "li t0, (1 << 8) | (1 << 5)",
        "csrw sstatus, t0",
        "li t0, (7 << 0) | (1 << 3)",
        "csrw pmpcfg0, t0",
        "li t0, 0xffffffffffff",
        "csrw pmpaddr0, t0",
        "li	t2, (1 << 1) | (1 << 5) | (1 << 9)",
        "csrw mideleg, t2",
        "csrw sie, t2",
        "mv t0, ra",
        "csrw sepc, t0",
        "sfence.vma",
        "sret",
    );
}

pub unsafe fn init() -> ! {
    let dt_addr = asm::reg!("a1") as *mut u8;
    let hart = csr::hartid::read();
    if hart != 0 {
        wait();
    }
    interrupts::init();
    let fdt = FDT::from_addr(dt_addr);
    let raw_mem_range = fdt.mem_range();
    let heap_start = paging::init(raw_mem_range.end());
    let heap_mem = Range {
        start: heap_start,
        end: raw_mem_range.end(),
    };
    to_supervisor();
    main(heap_mem, fdt);
}
