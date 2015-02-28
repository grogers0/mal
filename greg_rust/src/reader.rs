use types::LispType;
use types::LispType::*;
use std::result::Result;

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
        _ => read_atom(reader)
    }
}

fn read_list(reader: &mut Reader) -> ParseResult {
    reader.next().unwrap(); // skip the opening "("

    let mut list = Vec::new();
    while try!(reader.peek()) != ")" {
        list.push(try!(read_form(reader)));
    }

    reader.next().unwrap(); // skip the trailing ")"

    Ok(List(list))
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
    } else {
        Ok(Symbol(token.to_string()))
    }
}
