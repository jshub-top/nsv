

#[macro_export]
macro_rules! print_log_1 {
    ($($arg:tt)*) => {{
        println!("nsv: {}", format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! print_log_err {
    ($($arg:tt)*) => {{
        println!("nsv error: {}", format_args!($($arg)*));
    }};
}
