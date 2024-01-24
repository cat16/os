const UART_BASE: u32 = 0x10010000;
const UART_REG_TXFIFO: *mut i32 = (UART_BASE + 0) as *mut i32;

pub fn exit() -> ! {
    unsafe {
        core::arch::asm!(
            "li t0, 0x20026",
            "sw t0, 0(sp)",
            "move a1, sp",
            "li a0, 0x18",
            ".balign 16",
            ".option push",
            ".option norvc",
            "slli zero, zero, 0x1f",
            "ebreak",
            "srai zero, zero, 0x7",
            options(noreturn)
        );
    }
}

pub fn _print(args: core::fmt::Arguments<'_>) {
    let msg = args.as_str().expect("bruh");
    for b in msg.as_bytes() {
        while unsafe { *UART_REG_TXFIFO } < 0 {}
        unsafe { *UART_REG_TXFIFO = *b as i32 }
    }
}
