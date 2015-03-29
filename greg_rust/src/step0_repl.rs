extern crate readline;

use std::str;
use std::ffi::CString;

fn read(input: &str) -> &str {
    input
}

fn eval(ast: &str) -> &str {
    ast
}

fn print(output: &str) -> &str {
    output
}

fn rep(input: &str) -> &str {
    print(eval(read(input)))
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
