use std::{env, fs};

use stack_bad::Interpreter;

fn main() {
    let file = env::args().nth(1).unwrap();
    let src = fs::read_to_string(file).unwrap();
    Interpreter::new().run(&src);
}
