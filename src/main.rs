use std::{env, fs};

use crate::lexer::tokenize;

mod lexer;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut filename = "test.lg";
    if args.len() >= 2 {
        filename = &args[2];
    }
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    println!("{}", contents);
    tokenize(contents);
}
