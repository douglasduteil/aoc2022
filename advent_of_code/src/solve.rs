#[macro_export]
macro_rules! solve {
    ($part:expr, $solver:ident, $input:expr) => {{
        use aoc_cli::{ANSI_BOLD, ANSI_ITALIC, ANSI_RESET};
        use std::fmt::Display;
        use std::time::Instant;

        fn print_result<T: Display>(func: impl FnOnce(&str) -> Option<T>, input: &str) {
            let timer = Instant::now();
            let result = func(input);
            let elapsed = timer.elapsed();
            match result {
                Some(result) => {
                    println!(
                        "{} {}(elapsed: {:.2?}){}",
                        result, ANSI_ITALIC, elapsed, ANSI_RESET
                    );
                }
                None => {
                    println!("not solved.")
                }
            }
        }

        println!("🎄 {}Part {}{} 🎄", ANSI_BOLD, $part, ANSI_RESET);
        print_result($solver, $input);
    }};
}
