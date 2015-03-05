use std::result::Result;

use types::LispType;
use types::LispType::*;

#[derive(Debug)]
pub struct ParseError(String);
pub type ParseResult = Result<LispType, ParseError>;

struct Reader {
    tokens: Vec<String>,
    position: usize
}

impl Reader {
    fn new(tokens: Vec<String>) -> Reader {
        Reader { tokens: tokens, position: 0 }
    }

    fn peek(&self) -> Result<&str, ParseError> {
        if self.position < self.tokens.len() {
            Ok(&self.tokens[self.position])
        } else {
            Err(ParseError("expected more input but got EOF".to_string()))
        }
    }

    fn next(&mut self) -> Result<&str, ParseError> {
        if self.position < self.tokens.len() {
            let token = &self.tokens[self.position];
            self.position += 1;
            Ok(token)
        } else {
            Err(ParseError("expected more input but got EOF".to_string()))
        }
    }
}

fn tokenize(input: &str) -> Vec<String> {
    let re = regex!(r#"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"|;.*|[^\t\n\v\f\r \[\]{}('"`,;)]+)"#);
    let mut tokens = Vec::new();

    for cap in re.captures_iter(input) {
        tokens.push(cap.at(1).unwrap().to_string())
    }

    tokens
}

pub fn read_str(input: &str) -> ParseResult {
    let tokens = tokenize(input);
    let mut reader = Reader::new(tokens);
    read_form(&mut reader)
}

fn read_form(reader: &mut Reader) -> ParseResult {
    match try!(reader.peek()) {
        "(" => read_list(reader),
        "[" => read_vector(reader),
        _ => read_atom(reader)
    }
}

fn read_list(reader: &mut Reader) -> ParseResult {
    reader.next().unwrap(); // skip the opening "("

    let mut elems = Vec::new();
    while try!(reader.peek()) != ")" {
        elems.push(try!(read_form(reader)));
    }

    reader.next().unwrap(); // skip the trailing ")"

    Ok(List(elems))
}

fn read_vector(reader: &mut Reader) -> ParseResult {
    reader.next().unwrap(); // skip the opening "["

    let mut elems = Vec::new();
    while try!(reader.peek()) != "]" {
        elems.push(try!(read_form(reader)));
    }

    reader.next().unwrap(); // skip the trailing "]"

    Ok(Vector(elems))
}

fn read_atom(reader: &mut Reader) -> ParseResult {
    let token = try!(reader.next());
    if token == "nil" {
        Ok(Nil)
    } else if token == "true" {
        Ok(True)
    } else if token == "false" {
        Ok(False)
    } else if let Ok(int) = token.parse::<i64>() {
        Ok(Integer(int))
    } else if let Some('"') = token.chars().next() {
        let mut chars = token.chars();
        chars.next(); // skip leading "
        let mut buf = String::new();
        loop {
            match chars.next() {
                Some('\\') => match chars.next() {
                    Some('"') => buf.push('"'),
                    Some('\\') => buf.push('\\'),
                    Some('r') => buf.push('\r'),
                    Some('n') => buf.push('\n'),
                    Some('t') => buf.push('\t'),
                    Some(c) => { buf.push('\\'); buf.push(c); },
                    None => unreachable!()
                },
                Some(c) => buf.push(c),
                None => break
            }
        }
        buf.pop(); // remove trailing "
        Ok(Str(buf))
    } else if let Some(':') = token.chars().next() {
        Ok(Keyword(token.to_string()))
    } else {
        Ok(Symbol(token.to_string()))
    }
}
