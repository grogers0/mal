#![feature(core, plugin)]
#![plugin(regex_macros)]
extern crate regex;
extern crate readline;

use types::LispType;

mod reader;
mod types;

fn read(input: &str) -> reader::ParseResult {
    reader::read_str(input)
}

fn eval(ast: LispType) -> LispType {
    ast
}

fn print(value: LispType) -> String {
    format!("{}", value)
}

fn rep(input: &str) -> String {
    match read(input) {
        Ok(ast) => print(eval(ast)),
        Err(err) => format!("error: {:?}", err)
    }
}

fn main() {
    loop {
        match readline::readline("user> ") {
            Some(line) => {
                println!("{}", rep(&line));
                readline::add_history(&line);
            },
            None => return
        }
    }
}
