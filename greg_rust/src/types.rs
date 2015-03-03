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
        match self {
            &Nil => out.write_str("nil"),
            &True => out.write_str("true"),
            &False => out.write_str("false"),
            &Integer(int) => int.fmt(out),
            &Symbol(ref sym) => sym.fmt(out),
            &Str(ref s) => out.write_str(s), // quotes are included
            &List(ref elems) => {
                try!(out.write_str("("));
                for (i, v) in elems.iter().enumerate() {
                    if i != 0 {
                        try!(out.write_str(" "));
                    }
                    try!(v.fmt(out));
                }
                out.write_str(")")
            },
            &Vector(ref elems) => {
                try!(out.write_str("["));
                for (i, v) in elems.iter().enumerate() {
                    if i != 0 {
                        try!(out.write_str(" "));
                    }
                    try!(v.fmt(out));
                }
                out.write_str("]")
            },
            &Func(_) => out.write_str("#<function ...>"),
            &Closure(_,_) => out.write_str("#<function ...>")
        }
    }
}
