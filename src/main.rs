use chumsky::prelude::*;
use entropyc::{interpreter::Interpreter, parser::program_parser};

fn main() {
    let filename = std::env::args().nth(1).expect("Expect file path");
    let content = std::fs::read_to_string(filename).expect("file read failed");

    let ast = match program_parser().parse(content) {
        Ok(program) => program,
        Err(e) => {
            eprintln!("Invalid syntax: {:?}", e);
            panic!();
        }
    };

    let mut interpreter = Interpreter::default();
    interpreter.execute(&ast);
}
