use core::fmt::{self, Write};

use crate::util::mutex::Mutex;
use core::arch::asm;

// --machine sifive_u
// const UART_BASE: u32 = 0x10010000;
// --machine virt
const UART_BASE: u32 = 0x10000000;
static UART: Mutex<Uart> = Mutex::new(Uart::new(UART_BASE));

struct Uart {
    base: u32,
}

impl Uart {
    pub const fn new(base: u32) -> Self {
        Self { base }
    }
}

impl fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for b in s.as_bytes() {
            while unsafe { *(self.base as *mut i32) } < 0 {}
            unsafe { *(self.base as *mut i32) = *b as i32 }
        }
        Ok(())
    }
}

pub fn exit(code: usize) -> ! {
    let data = [0x20026, code];
    unsafe {
        semihost(0x18, data.as_ptr() as *const u8);
    }
    super::wait()
}

unsafe fn semihost(call: usize, data: *const u8) {
    asm!(
        "mv a0, {call}",
        "mv a1, {data}",
        ".balign 16",
        ".option push",
        ".option norvc",
        "slli zero, zero, 0x1f",
        "ebreak",
        "srai zero, zero, 0x7",
        call = in(reg) call,
        data = in(reg) data
    )
}

pub fn _print(args: core::fmt::Arguments<'_>) {
    // NOTE: something really dumb can happen here;
    // if you evaluate an expression in a print statement, and that
    // causes an interrupt, this will be left locked...
    // Should I set up the heap before interrupts? or just avoid printing until both...?
    // or maybe force unlock if there's an interrupt?
    // or store the hart in the lock, and unlock if that hart was interrupted??
    // or just have a constant-sized buffer?
    // or create a "locked writer"?
    UART.lock().write_fmt(args).unwrap();
}
