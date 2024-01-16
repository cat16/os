use core::fmt::Arguments;

use spin::Mutex;
use uart_16550::SerialPort;
use x86_64::instructions::{interrupts, port::Port};

pub static UART: Mutex<SerialPort> = Mutex::new(unsafe { SerialPort::new(0x3F8) });

pub fn exit() {
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(0x10u32);
    }
}

#[doc(hidden)]
pub fn _print(args: Arguments<'_>) {
    use core::fmt::Write;
    interrupts::without_interrupts(|| {
        UART.lock().write_fmt(args).unwrap();
    })
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::qemu::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
