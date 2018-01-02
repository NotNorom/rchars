extern crate rand;
use rand::{thread_rng, Rng};

use std::env;

fn main() {
    if let Some(first_argument) = env::args().nth(1) {
        if let Ok(count) = first_argument.parse::<usize>() {
            let mut rng = thread_rng();
            let gen = rng.gen_ascii_chars();
            let result = gen.take(count);
            for elem in result {
                print!("{}", elem);
            }
            println!("");
        } else {
            println!("Please enter a number > 0.");
        }
    }
}
