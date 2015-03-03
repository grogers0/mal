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
mod core;

fn read(input: &str) -> reader::ParseResult {
    reader::read_str(input)
}

fn eval_ast(ast: LispType, env: &mut Environment) -> LispResult {
    match ast {
        Symbol(sym) => env.get(&sym),
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

fn set_bindings(bindings: Vec<LispType>, env: &mut Environment) -> LispResult {
    let mut it = bindings.into_iter();
    loop {
        match (it.next(), it.next()) {
            (Some(Symbol(sym)), Some(val)) => {
                let val = try!(eval(val, env));
                env.set(&sym, val);
            }
            (Some(_), Some(_)) => return Err(LispError("First binding argument must be a symbol".to_string())),
            (Some(_), None) => return Err(LispError("Binding arguments must have even length".to_string())),
            (None, None) => return Ok(Nil),
            _ => unreachable!()
        }
    }
}

fn eval(ast: LispType, env: &mut Environment) -> LispResult {
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
                    let val = try!(eval(val, env));
                    env.set(&sym, val.clone());
                    Ok(val)
                },
                _ => Err(LispError("def! must be called with 2 args, a symbol and a value".to_string())),
            }
        } else if arg0 == Symbol("let*".to_string()) {
            let mut let_env = Environment::new(Some(env));
            let mut args_iter = args.into_iter();
            match (args_iter.next(), args_iter.next(), args_iter.next()) {
                (Some(List(bindings)), Some(val), None) | (Some(Vector(bindings)), Some(val), None) => {
                    try!(set_bindings(bindings, &mut let_env));
                    eval(val, &mut let_env)
                },
                _ => Err(LispError("let* must be called with a list/vector of bindings and an expression to evaluate".to_string()))
            }
        } else if arg0 == Symbol("do".to_string()) {
            let mut ret = Nil;
            for arg in args.into_iter() {
                ret = try!(eval(arg, env));
            }
            Ok(ret)
        } else if arg0 == Symbol("if".to_string()) {
            let mut args_iter = args.into_iter();
            if let Some(cond) = args_iter.next() {
                let cond = try!(eval(cond, env));
                if cond == Nil || cond == False {
                    args_iter.next(); // Skip the second parameter
                    match args_iter.next() {
                        Some(val) => eval(val, env),
                        None => Ok(Nil)
                    }
                } else {
                    match args_iter.next() {
                        Some(val) => eval(val, env),
                        None => Err(LispError("if must be called with at least two arguments".to_string()))
                    }
                }
            } else {
                Err(LispError("if must be called with at least two arguments".to_string()))
            }
        } else if arg0 == Symbol("fn*".to_string()) {
            let mut args_iter = args.into_iter();
            match (args_iter.next(), args_iter.next(), args_iter.next()) {
                (Some(List(args)), Some(val), None) | (Some(Vector(args)), Some(val), None) => {
                    Ok(Closure(args, Box::new(val)))
                },
                _ => Err(LispError("fn* must be called with a binding list and an expression".to_string()))
            }
        } else if let List(maybe_fn_def) = arg0 {
            let arg0 = try!(eval(List(maybe_fn_def), env));
            let mut resolved_args = Vec::with_capacity(1 + args.len());
            resolved_args.push(arg0);
            for arg in args {
                resolved_args.push(arg)
            }
            eval(List(resolved_args), env)
        } else {
            match try!(eval_ast(arg0, env)) {
                Func(func) => {
                    if let List(args) = try!(eval_ast(List(args), env)) {
                        func(args)
                    } else {
                        unreachable!()
                    }
                },
                Closure(bindings, expr) => {
                    let mut env = try!(Environment::with_bindings(Some(env), bindings, args));
                    eval(*expr, &mut env)
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

fn rep(input: &str, env: &mut Environment) -> String {
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
    let mut env = core::default_environment();
    loop {
        match readline::readline("user> ") {
            Some(line) => {
                println!("{}", rep(&line, &mut env));
                readline::add_history(&line);
            },
            None => return
        }
    }
}
