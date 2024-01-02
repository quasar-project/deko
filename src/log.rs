pub use colored::Colorize;

pub const LIB_NAME: &str = "deko";

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        println!("{}: {}", crate::log::LIB_NAME.cyan().bold(),
          format_args!($($arg)*)
        );
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        println!("{}: {}: {}", crate::log::LIB_NAME.yellow().bold(),
          "⚠ warning".yellow().bold(),
          format_args!($($arg)*).to_string().yellow().bold()
        );
    };
}

#[macro_export]
macro_rules! fatal_error {
    ($($arg:tt)*) => {
        println!("{}: {}: {}", crate::log::LIB_NAME.red().bold(),
          "⛔ unrecoverable error occurred".red().bold(),
          format_args!($($arg)*).to_string().red().bold().underline()
        );
    };
}


