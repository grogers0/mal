#![feature(core, plugin)]
#![plugin(regex_macros)]
extern crate regex;
extern crate readline;

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
        match readline::readline("user> ") {
            Some(line) => {
                println!("{}", print(eval(read(&line))));
                readline::add_history(&line);
            },
            None => return
        }
    }
}
