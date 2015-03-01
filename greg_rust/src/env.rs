use std::collections::HashMap;

use types::{LispType, LispError, LispResult};
use types::LispType::*;

pub struct Environment {
    symbols: HashMap<String, LispType>
}
impl Environment {
    pub fn lookup_symbol(&self, symbol: &str) -> LispResult {
        if let Some(value) = self.symbols.get(symbol) {
            Ok(value.clone())
        } else {
            Err(LispError(format!("{} not found", symbol)))
        }
    }

    pub fn new() -> Environment {
        let mut symbols = HashMap::new();
        symbols.insert("+".to_string(), Func(add));
        symbols.insert("-".to_string(), Func(sub));
        symbols.insert("*".to_string(), Func(mul));
        symbols.insert("/".to_string(), Func(div));

        Environment { symbols: symbols }
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

