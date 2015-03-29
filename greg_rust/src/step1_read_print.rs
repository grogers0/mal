#![feature(plugin)]
#![plugin(regex_macros)]
extern crate regex;
extern crate readline;

use std::str;
use std::ffi::CString;

use types::LispType;

mod reader;
mod types;
mod env;

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
    let prompt = &CString::new("user> ").unwrap();
    loop {
        match readline::readline(prompt) {
            Ok(line) => {
                println!("{}", rep(str::from_utf8(line.to_bytes()).unwrap()));
                readline::add_history(&line);
            },
            Err(_) => return
        }
    }
}
