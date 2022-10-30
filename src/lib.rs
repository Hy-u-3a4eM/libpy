//! This is my first crate. Its task is to facilitate the programmer's work by allowing the use of the well-known function "input()".
//!
//! This works the same way as in python, only you need to write '!', since this is a macro: "input!()" or "input!("Enter your name: ")".

pub mod std {
    /// Outputs (or not) text and awaiting input.
    #[macro_export]
    macro_rules! input {
        () => {{
            use std::io::{stdin, self, Write};

            let mut buf = String::new();
            stdin().read_line(&mut buf).unwrap();
            buf
        }};

        ($arg:tt) => {{
            use std::io::{stdin, self, Write};

            let text: &str = $arg;

            print!("{}", text);
            io::stdout().flush().expect("flush error");

            let mut buf = String::new();
            stdin().read_line(&mut buf).unwrap();
            buf
        }};
    }

    /// Parses to int.
    #[macro_export]
    macro_rules! int {
        () => {
            0
        };

        ($arg:expr) => {{
            $arg.trim().parse::<isize>().unwrap()
        }};
    }
}
