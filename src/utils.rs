#[macro_export]
macro_rules! error_and_exit {
    ($($arg:tt)*) => {{
        log::error!($($arg)*);
        std::process::exit(1);
    }};
}
