use core::fmt::Arguments;

use uart_16550::SerialPort;
use x86_64::instructions::{interrupts, port::Port};

use super::hlt_loop;

pub static UART: Mutex<SerialPort> = Mutex::new(unsafe { SerialPort::new(0x3F8) });

pub fn exit() -> ! {
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(0x10u32);
    }
    hlt_loop()
}

#[doc(hidden)]
pub fn _print(args: Arguments<'_>) {
    use core::fmt::Write;
    interrupts::without_interrupts(|| {
        UART.lock().write_fmt(args).unwrap();
    })
}
