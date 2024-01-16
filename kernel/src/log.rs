use core::fmt::Arguments;


#[doc(hidden)]
pub fn _log(args: Arguments<'_>) {
    use core::fmt::Write;
    interrupts::without_interrupts(|| {
        UART.lock().write_fmt(args).unwrap();
    })
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => ($crate::_log(format_args!($($arg)*)));
}

