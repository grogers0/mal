#![feature(plugin, collections)]
#![plugin(regex_macros)]
extern crate regex;
extern crate readline;

use std::rc::Rc;

use types::{LispType, LispError, LispResult};
use types::LispType::*;
use env::Environment;

mod reader;
mod types;
mod env;
mod core;

fn read(input: &str) -> reader::ParseResult {
    reader::read_str(input)
}

fn eval_ast(ast: LispType, env: Rc<Environment>) -> LispResult {
    match ast {
        Symbol(sym) => env.get(&sym),
        List(values) => {
            let mut evalues = Vec::with_capacity(values.len());
            for val in values.into_iter() {
                evalues.push(try!(eval(val, env.clone())))
            }
            Ok(List(evalues))
        }
        anything => Ok(anything)
    }
}

fn set_bindings(bindings: Vec<LispType>, env: Rc<Environment>) -> LispResult {
    let mut it = bindings.into_iter();
    loop {
        match (it.next(), it.next()) {
            (Some(Symbol(sym)), Some(val)) => {
                let val = try!(eval(val, env.clone()));
                env.set(&sym, val);
            }
            (Some(_), Some(_)) => return Err(LispError("First binding argument must be a symbol".to_string())),
            (Some(_), None) => return Err(LispError("Binding arguments must have even length".to_string())),
            (None, None) => return Ok(Nil),
            _ => unreachable!()
        }
    }
}

fn eval(ast: LispType, env: Rc<Environment>) -> LispResult {
    if let List(mut values) = ast {
        if values.is_empty() {
            return Err(LispError("tried to evaluate a list with no function".to_string()))
        }
        let args = values.split_off(1);
        let arg0 = values.into_iter().next().unwrap();

        if arg0 == Symbol("def!".to_string()) {
            let mut args_iter = args.into_iter();
            match (args_iter.next(), args_iter.next(), args_iter.next()) {
                (Some(Symbol(sym)), Some(val), None) => {
                    let val = try!(eval(val, env.clone()));
                    env.set(&sym, val.clone());
                    Ok(val)
                },
                _ => Err(LispError("def! must be called with 2 args, a symbol and a value".to_string())),
            }
        } else if arg0 == Symbol("let*".to_string()) {
            let let_env = Rc::new(Environment::new(Some(env.clone())));
            let mut args_iter = args.into_iter();
            if let Some(List(bindings)) = args_iter.next() {
                try!(set_bindings(bindings, let_env.clone()));
                match args_iter.next() {
                    Some(val) => eval(val, let_env),
                    None => Err(LispError("let* must be called with a second argument to evaluate".to_string()))
                }
            } else {
                Err(LispError("let* must be called with a list of bindings as the first argument".to_string()))
            }
        } else {
            match try!(eval_ast(arg0, env.clone())) {
                Func(func) => {
                    if let List(args) = try!(eval_ast(List(args), env)) {
                        func(args)
                    } else {
                        unreachable!()
                    }
                },
                misunderstood => return Err(LispError(format!("{} is not a function, cannot evaluate it", misunderstood)))
            }
        }
    } else {
        eval_ast(ast, env)
    }
}

fn print(value: LispType) -> String {
    format!("{}", value)
}

fn rep(input: &str, env: Rc<Environment>) -> String {
    match read(input) {
        Err(err) => format!("error: {:?}", err),
        Ok(ast) => {
            match eval(ast, env) {
                Err(err) => format!("error: {:?}", err),
                Ok(ast) => print(ast)
            }
        }
    }
}

fn main() {
    let env = core::default_environment();
    loop {
        match readline::readline("user> ") {
            Some(line) => {
                println!("{}", rep(&line, env.clone()));
                readline::add_history(&line);
            },
            None => return
        }
    }
}
