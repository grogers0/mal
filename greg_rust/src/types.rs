use std::fmt;

pub enum LispType {
    Integer(i64),
    Symbol(String),
    List(Vec<LispType>)
}

impl fmt::Display for LispType {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &LispType::Integer(int) => int.fmt(out),
            &LispType::Symbol(ref sym) => sym.fmt(out),
            &LispType::List(ref list) => {
                try!(out.write_str("("));
                for (i, v) in list.iter().enumerate() {
                    if i != 0 {
                        try!(out.write_str(" "));
                    }
                    try!(v.fmt(out));
                }
                out.write_str(")")
            }
        }
    }
}
