use std::collections::HashMap;

use types::{LispType, LispError, LispResult};
use types::LispType::*;

pub struct Environment<'a> {
    symbols: HashMap<String, LispType>,
    outer_env: Option<&'a Environment<'a>>
}
impl<'a> Environment<'a> {
    pub fn set(&mut self, symbol: &str, value: LispType) {
        self.symbols.insert(symbol.to_string(), value);
    }

    pub fn get(&self, symbol: &str) -> LispResult {
        if let Some(value) = self.symbols.get(symbol) {
            Ok(value.clone())
        } else if let Some(ref env) = self.outer_env {
            env.get(symbol)
        } else {
            Err(LispError(format!("{} not found", symbol)))
        }
    }

    pub fn new(outer_env: Option<&'a Environment>) -> Environment<'a> {
        Environment { symbols: HashMap::new(), outer_env: outer_env }
    }

    pub fn with_numerics() -> Environment<'a> {
        let mut env = Environment::new(None);
        env.set("+", Func(add));
        env.set("-", Func(sub));
        env.set("*", Func(mul));
        env.set("/", Func(div));
        env
    }
}

fn binary_int_op<F: Fn(i64,i64) -> i64>(f: F, args: Vec<LispType>) -> LispResult {
    if args.len() != 2 {
        return Err(LispError(format!("binary function called with {} arguments", args.len())))
    }

    if let Integer(a) = args[0] {
        if let Integer(b) = args[1] {
            Ok(Integer(f(a, b)))
        } else {
            return Err(LispError(format!("illegal argument: {} to function which expects integers", args[1])))
        }
    } else {
        return Err(LispError(format!("illegal argument: {} to function which expects integers", args[1])))
    }
}

fn add(args: Vec<LispType>) -> LispResult { binary_int_op(|a:i64, b:i64| { a + b }, args) }
fn sub(args: Vec<LispType>) -> LispResult { binary_int_op(|a:i64, b:i64| { a - b }, args) }
fn mul(args: Vec<LispType>) -> LispResult { binary_int_op(|a:i64, b:i64| { a * b }, args) }
fn div(args: Vec<LispType>) -> LispResult { binary_int_op(|a:i64, b:i64| { a / b }, args) }

