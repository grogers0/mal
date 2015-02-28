#![feature(core, old_io, plugin)]
#![plugin(regex_macros)]
extern crate regex;

use std::old_io;
use types::LispType;

mod reader;
mod types;

fn read(input: &str) -> LispType {
    reader::read_str(input).unwrap()
}

fn eval(ast: LispType) -> LispType {
    ast
}

fn print(value: LispType) -> String {
    format!("{}", value)
}

fn main() {
    loop {
        print!("user> ");
        match old_io::stdin().read_line() {
            Ok(line) => println!("{}", print(eval(read(&line)))),
            Err(_) => return
        }
    }
}
