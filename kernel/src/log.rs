use core::fmt::Arguments;


#[doc(hidden)]
pub fn _log(args: Arguments<'_>) {
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => ($crate::_log(format_args!($($arg)*)));
}

