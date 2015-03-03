use std::fmt;

use types::LispType::*;

#[derive(Debug)]
pub struct LispError(pub String);

pub type LispResult = Result<LispType, LispError>;

#[derive(Clone)]
pub enum LispType {
    Nil,
    True,
    False,
    Integer(i64),
    Symbol(String),
    Str(String),
    List(Vec<LispType>),
    Vector(Vec<LispType>),
    Func(fn(Vec<LispType>) -> LispResult),
    Closure(Vec<LispType>, Box<LispType>)
}

impl PartialEq for LispType {
    fn eq(&self, other: &LispType) -> bool {
        match (self, other) {
            (&Nil, &Nil) => true,
            (&True, &True) => true,
            (&False, &False) => true,
            (&Integer(ref a), &Integer(ref b)) if a == b => true,
            (&Symbol(ref a), &Symbol(ref b)) if a == b => true,
            (&Str(ref a), &Str(ref b)) if a == b => true,
            (&List(ref a), &List(ref b)) if a == b => true,
            // Functions are not comparable
            _ => false
        }
    }
}

impl fmt::Display for LispType {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        out.write_str(&pr_str(self, false))
    }
}

pub fn pr_str(val: &LispType, print_readably: bool) -> String {
    match val {
        &Nil => "nil".to_string(),
        &True => "true".to_string(),
        &False => "false".to_string(),
        &Integer(int) => int.to_string(),
        &Symbol(ref sym) => sym.clone(),
        &Str(ref s) => s.clone(), // quotes are included
        &List(ref elems) => {
            let mut buf = String::new();
            buf.push('(');
            for (i, v) in elems.iter().enumerate() {
                if i != 0 {
                    buf.push(' ');
                }
                buf.push_str(&pr_str(v, print_readably));
            }
            buf.push(')');
            buf
        },
        &Vector(ref elems) => {
            let mut buf = String::new();
            buf.push('[');
            for (i, v) in elems.iter().enumerate() {
                if i != 0 {
                    buf.push(' ');
                }
                buf.push_str(&pr_str(v, print_readably));
            }
            buf.push(']');
            buf
        },
        &Func(_) => "#<function ...>".to_string(),
        &Closure(_,_) => "#<function ...>".to_string()
    }
}
