use types::LispType;
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

    fn peek(&self) -> &str {
        &self.tokens[self.position]
    }

    fn next(&mut self) -> &str {
        let token = &self.tokens[self.position];
        self.position += 1;
        token
    }
}

fn tokenize(input: &str) -> Vec<String> {
    let re = regex!(r#"[\s,]*([()]|\w+)"#);
    let mut tokens = Vec::new();

    for cap in re.captures_iter(input) {
        tokens.push(cap.at(1).unwrap().to_string())
    }

    tokens
}

pub fn read_str(input: &str) -> ParseResult {
    let mut reader = Reader::new(tokenize(input));
    read_form(&mut reader)
}

fn read_form(reader: &mut Reader) -> ParseResult {
    match reader.peek() {
        "(" => read_list(reader),
        _ => read_atom(reader)
    }
}

fn read_list(reader: &mut Reader) -> ParseResult {
    reader.next(); // skip the opening "("

    let mut list = Vec::new();
    while reader.peek() != ")" {
        list.push(try!(read_form(reader)));
    }

    reader.next(); // skip the trailing ")"

    Ok(LispType::List(list))
}

fn read_atom(reader: &mut Reader) -> ParseResult {
    let token = reader.next();
    if let Ok(int) = token.parse::<i64>() {
        Ok(LispType::Integer(int))
    } else {
        Ok(LispType::Symbol(token.to_string()))
    }
}
