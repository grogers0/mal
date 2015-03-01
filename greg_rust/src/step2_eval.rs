#![feature(core, plugin, collections)]
#![plugin(regex_macros)]
extern crate regex;
extern crate readline;

use types::{LispType, LispError, LispResult};
use types::LispType::*;
use env::Environment;

mod reader;
mod types;
mod env;

fn read(input: &str) -> reader::ParseResult {
    reader::read_str(input)
}

fn eval_ast(ast: LispType, env: &Environment) -> LispResult {
    match ast {
        Symbol(sym) => env.lookup_symbol(&sym),
        List(values) => {
            let mut evalues = Vec::with_capacity(values.len());
            for val in values.into_iter() {
                evalues.push(try!(eval(val, env)))
            }
            Ok(List(evalues))
        }
        anything => Ok(anything)
    }
}

fn eval(ast: LispType, env: &Environment) -> LispResult {
    if let List(values) = ast {
        match try!(eval_ast(List(values), env)) {
            List(mut values) => {
                if values.is_empty() {
                    return Err(LispError("tried to evaluate a list with no function".to_string()))
                }
                let args = values.split_off(1);
                if let Func(func) = values[0] {
                    func(args)
                } else {
                    return Err(LispError(format!("{} is not a function, cannot evaluate it", values[0])))
                }
            },
            _ => unreachable!()
        }
    } else {
        eval_ast(ast, env)
    }
}

fn print(value: LispType) -> String {
    format!("{}", value)
}

fn rep(input: &str) -> String {
    match read(input) {
        Err(err) => format!("error: {:?}", err),
        Ok(ast) => {
            match eval(ast, &Environment::new()) {
                Err(err) => format!("error: {:?}", err),
                Ok(ast) => print(ast)
            }
        }
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
