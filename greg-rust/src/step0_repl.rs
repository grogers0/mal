#![feature(old_io)]
use std::old_io;

fn read(input: String) -> String {
    input
}

fn eval(ast: String) -> String {
    ast
}

fn print(output: String) -> String {
    output
}

fn main() {
    loop {
        print!("prompt> ");
        match old_io::stdin().read_line() {
            Ok(line) => println!("{}", print(eval(read(line)))),
            Err(_) => return
        }
    }
}
