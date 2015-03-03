#![allow(dead_code)]

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

    pub fn with_bindings(outer_env: Option<&'a Environment>, binds: Vec<LispType>, exprs: Vec<LispType>) -> Result<Environment<'a>, LispError> {
        let mut symbols = HashMap::with_capacity(binds.len());
        if binds.len() != exprs.len() {
            return Err(LispError("incorrect number of arguments passed to a closure".to_string()))
        }
        for (bind, value) in binds.into_iter().zip(exprs.into_iter()) {
            if let Symbol(sym) = bind {
                symbols.insert(sym, value);
            } else {
                return Err(LispError("function binding is not a symbol".to_string()))
            }
        }

        Ok(Environment { symbols: symbols, outer_env: outer_env })
    }
}
