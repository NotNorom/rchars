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
            println!("Enter a number! It has to be bigger than or equal to 0.");
        }
    } else {
        println!("Random character generator\n");
        println!("Usage:");
        println!("    rchars <number of characters to generate>");
    }
}
