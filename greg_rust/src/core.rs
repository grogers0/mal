use types;
use types::{LispType, LispResult, LispError};
use types::LispType::*;
use env::Environment;

pub fn default_environment() -> Environment {
    let mut env = Environment::new(None);
    env.set("+", Func(add));
    env.set("-", Func(sub));
    env.set("*", Func(mul));
    env.set("/", Func(div));

    env.set("list", Func(mk_list));
    env.set("list?", Func(is_list));
    env.set("empty?", Func(is_empty));
    env.set("count", Func(count));

    env.set("=", Func(eq));

    env.set("<", Func(lt));
    env.set("<=", Func(le));
    env.set(">", Func(gt));
    env.set(">=", Func(ge));

    env.set("pr-str", Func(pr_str));
    env.set("str", Func(str));
    env.set("prn", Func(prn));
    env.set("println", Func(println));

    env
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

fn eq(args: Vec<LispType>) -> LispResult {
    let mut args_iter = args.into_iter();
    let arg0 = args_iter.next();
    for arg in args_iter {
        if Some(arg) != arg0 {
            return Ok(False);
        }
    }
    Ok(True)
}

fn mk_list(args: Vec<LispType>) -> LispResult { Ok(List(args)) }
fn is_list(args: Vec<LispType>) -> LispResult {
    for arg in args {
        if let List(_) = arg {
        } else {
            return Ok(False);
        }
    }
    Ok(True)
}

fn is_empty(args: Vec<LispType>) -> LispResult {
    for arg in args {
        match arg {
            List(elems) | Vector(elems) => if !elems.is_empty() { return Ok(False) },
            _ => return Err(LispError(format!("argument {} is not a list", arg)))
        }
    }
    Ok(True)
}

fn count(args: Vec<LispType>) -> LispResult {
    let mut count = 0;
    for arg in args {
        match arg {
            Nil => (),
            List(elems) | Vector(elems) => count += elems.len() as i64,
            _ => return Err(LispError(format!("argument {} is not a list", arg)))
        }
    }
    Ok(Integer(count))
}

fn binary_bool_op<F: Fn(i64,i64) -> bool>(f: F, args: Vec<LispType>) -> LispResult {
    if args.len() != 2 {
        return Err(LispError(format!("binary function called with {} arguments", args.len())))
    }

    if let Integer(a) = args[0] {
        if let Integer(b) = args[1] {
            if f(a, b) {
                Ok(True)
            } else {
                Ok(False)
            }
        } else {
            return Err(LispError(format!("illegal argument: {} to function which expects integers", args[1])))
        }
    } else {
        return Err(LispError(format!("illegal argument: {} to function which expects integers", args[1])))
    }
}

fn lt(args: Vec<LispType>) -> LispResult { binary_bool_op(|a:i64, b:i64| { a < b }, args) }
fn le(args: Vec<LispType>) -> LispResult { binary_bool_op(|a:i64, b:i64| { a <= b }, args) }
fn gt(args: Vec<LispType>) -> LispResult { binary_bool_op(|a:i64, b:i64| { a > b }, args) }
fn ge(args: Vec<LispType>) -> LispResult { binary_bool_op(|a:i64, b:i64| { a >= b }, args) }

fn pr_str(args: Vec<LispType>) -> LispResult {
    let mut buf = String::new();
    for (i, v) in args.iter().enumerate() {
        if i != 0 {
            buf.push(' ');
        }
        buf.push_str(&types::pr_str(v, true));
    }
    Ok(Str(buf))
}

fn str(args: Vec<LispType>) -> LispResult {
    let mut buf = String::new();
    for v in args.iter() {
        buf.push_str(&types::pr_str(v, false));
    }
    Ok(Str(buf))
}

fn prn(args: Vec<LispType>) -> LispResult {
    let mut buf = String::new();
    for (i, v) in args.iter().enumerate() {
        if i != 0 {
            buf.push(' ');
        }
        buf.push_str(&types::pr_str(v, true));
    }
    println!("{}", buf);
    Ok(Nil)
}

fn println(args: Vec<LispType>) -> LispResult {
    let mut buf = String::new();
    for (i, v) in args.iter().enumerate() {
        if i != 0 {
            buf.push(' ');
        }
        buf.push_str(&types::pr_str(v, false));
    }
    println!("{}", buf);
    Ok(Nil)
}

