use std::fmt;
use std::rc::Rc;

use types::LispType::*;
use env::Environment;

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
    Keyword(String),
    List(Vec<LispType>),
    Vector(Vec<LispType>),
    Func(fn(Vec<LispType>) -> LispResult),
    Closure(Vec<LispType>, Box<LispType>, Rc<Environment>)
}

impl PartialEq for LispType {
    fn eq(&self, other: &LispType) -> bool {
        match (self, other) {
            (&Nil, &Nil) => true,
            (&True, &True) => true,
            (&False, &False) => true,
            (&Integer(ref a), &Integer(ref b)) => a == b,
            (&Symbol(ref a), &Symbol(ref b)) => a == b,
            (&Str(ref a), &Str(ref b)) => a == b,
            (&Keyword(ref a), &Keyword(ref b)) => a == b,
            (&List(ref a), &List(ref b)) => a == b,
            (&List(ref a), &Vector(ref b)) => a == b,
            (&Vector(ref a), &Vector(ref b)) => a == b,
            (&Vector(ref a), &List(ref b)) => a == b,
            (&Func(_), &Func(_)) => false,
            (&Closure(_,_,_), &Closure(_,_,_)) => false,
            _ => false
        }
    }
}

impl fmt::Display for LispType {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        out.write_str(&pr_str(self, false))
    }
}

impl fmt::Debug for LispType {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        out.write_str(&pr_str(self, true))
    }
}

pub fn pr_str(val: &LispType, print_readably: bool) -> String {
    match val {
        &Nil => "nil".to_string(),
        &True => "true".to_string(),
        &False => "false".to_string(),
        &Integer(int) => int.to_string(),
        &Symbol(ref sym) => sym.clone(),
        &Str(ref s) => if print_readably {
            let mut buf = String::new();
            buf.push('"');
            for c in s.chars() {
                match c {
                    '"' => buf.push_str("\\\""),
                    '\\' => buf.push_str("\\\\"),
                    '\r' => buf.push_str("\\r"),
                    '\n' => buf.push_str("\\n"),
                    '\t' => buf.push_str("\\t"),
                    c => buf.push(c)
                }
            }
            buf.push('"');
            buf
        } else {
            s.clone()
        },
        &Keyword(ref s) => s.clone(),
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
        &Closure(_,_,_) => "#<function ...>".to_string()
    }
}
