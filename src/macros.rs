#[macro_export]
macro_rules! c_str {
    ($string:expr) => {
        concat!($string, "\0")
    };
    ($fmt:expr, $($arg:tt)*) => (format!(concat!($fmt, "\0"), $($arg)*));
}

#[macro_export]
macro_rules! win_str_ptr {
    ($string:expr) => {
        concat!($string, "\0").as_ptr() as *mut u8
    };
    ($fmt:expr, $($arg:tt)*) => (format!(concat!($fmt, "\0"), $($arg)*).as_ptr() as *mut u8);
}

#[macro_export]
macro_rules! wide_win_str_ptr {
    ($string:expr) => {
        concat!($string, "\0").as_ptr() as *mut u16
    };
    ($fmt:expr, $($arg:tt)*) => (format!(concat!($fmt, "\0"), $($arg)*).as_ptr() as *mut u16);
}
