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
    List(Vec<LispType>),
    Func(fn(Vec<LispType>) -> Result<LispType, LispError>)
}

impl fmt::Display for LispType {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Nil => out.write_str("nil"),
            &True => out.write_str("true"),
            &False => out.write_str("false"),
            &Integer(int) => int.fmt(out),
            &Symbol(ref sym) => sym.fmt(out),
            &List(ref list) => {
                try!(out.write_str("("));
                for (i, v) in list.iter().enumerate() {
                    if i != 0 {
                        try!(out.write_str(" "));
                    }
                    try!(v.fmt(out));
                }
                out.write_str(")")
            },
            &Func(_) => out.write_str("#<function ...>")
        }
    }
}
