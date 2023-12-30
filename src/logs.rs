pub static mut VERBOSE: bool = false;


pub unsafe fn _verbose(fmt: String) {
    if VERBOSE {
        println!("{}", fmt);
    }
}

#[macro_export]
macro_rules! verbose {
    () => (
        print!("\n")
    );
    ($($arg:tt)*) => (
       unsafe { _verbose(format!($($arg)*)) };
    )
}

#[macro_export]
macro_rules! error {
    () => (
        eprint!("\n")
    );
    ($($arg:tt)*) => (
        eprintln!("{}", Stylize::red(format!($($arg)*)));
    )
}
