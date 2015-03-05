#![allow(dead_code)]
use std::collections::HashMap;
use std::iter::FromIterator;
use std::rc::Rc;
use std::cell::RefCell;

use types::{LispType, LispError, LispResult};
use types::LispType::*;

#[derive(Debug, Clone)]
pub struct Environment {
    symbols: RefCell<HashMap<String, LispType>>,
    outer_env: Option<Rc<Environment>>
}
impl Environment {
    pub fn set(&self, symbol: &str, value: LispType) {
        self.symbols.borrow_mut().insert(symbol.to_string(), value);
    }

    pub fn get(&self, symbol: &str) -> LispResult {
        if let Some(value) = self.symbols.borrow().get(symbol) {
            Ok(value.clone())
        } else if let Some(ref env) = self.outer_env {
            env.get(symbol)
        } else {
            Err(LispError(format!("{} not found", symbol)))
        }
    }

    pub fn new(outer_env: Option<Rc<Environment>>) -> Environment {
        Environment { symbols: RefCell::new(HashMap::new()), outer_env: outer_env }
    }

    pub fn with_bindings(outer_env: Option<Rc<Environment>>, binds: Vec<LispType>, exprs: Vec<LispType>) -> Result<Environment, LispError> {
        let mut symbols = HashMap::with_capacity(binds.len());
        let mut binds_it = binds.into_iter();
        let mut exprs_it = exprs.into_iter();

        loop {
            match binds_it.next() {
                Some(Symbol(sym)) => {
                    if sym == "&" {
                        if let Some(Symbol(more)) = binds_it.next() {
                            symbols.insert(more, List(Vec::from_iter(exprs_it)));
                            break;
                        } else {
                            return Err(LispError("binding after & must be a symbol".to_string()))
                        }
                    } else if let Some(value) = exprs_it.next() {
                        symbols.insert(sym, value);
                    } else {
                        return Err(LispError("Closure called with incorrect number of arguments".to_string()));
                    }
                },
                Some(_) => return Err(LispError("Closure with non-symbol bindings".to_string())),
                None => {
                    if let Some(_) = exprs_it.next() {
                        return Err(LispError("Closure called with incorrect number of arguments".to_string()));
                    } else {
                        break;
                    }
                }
            }
        }

        Ok(Environment { symbols: RefCell::new(symbols), outer_env: outer_env })
    }
}
