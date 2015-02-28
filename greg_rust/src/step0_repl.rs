extern crate readline;

fn read(input: &str) -> &str {
    input
}

fn eval(ast: &str) -> &str {
    ast
}

fn print(output: &str) -> &str {
    output
}

fn main() {
    loop {
        match readline::readline("user> ") {
            Some(line) => {
                println!("{}", print(eval(read(&line))));
                readline::add_history(&line);
            },
            None => return
        }
    }
}
