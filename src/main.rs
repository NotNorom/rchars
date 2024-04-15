use std::process::ExitCode;

use rand::{
    distributions::{Alphanumeric, Distribution},
    thread_rng,
};

const HELP_TEXT: &'static str = r#"Random character generator

Usage:
    rchars <number of characters to generate>"#;

fn main() -> ExitCode {
    let Some(first_argument) = std::env::args().nth(1) else {
        println!("{HELP_TEXT}");
        return ExitCode::FAILURE;
    };

    let Ok(count) = first_argument.parse::<usize>() else {
        println!("Enter a number! It has to be bigger than or equal to 0.");
        return ExitCode::FAILURE;
    };

    Alphanumeric
        .sample_iter(thread_rng())
        .take(count)
        .for_each(|elem| print!("{}", elem as char));
    println!("");

    ExitCode::SUCCESS
}
