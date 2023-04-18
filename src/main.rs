use std::io::Write;

mod token;
mod lexer;
mod interpreter;
mod parser;
mod ast;

fn main() {
    let mut text = String::new();

    loop {
        print!("calc> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut text).unwrap();
        if text.trim().is_empty() {
            text.clear();
            continue;
        }

        match interpreter::run(text.clone()) {
            Ok(result) => {
                println!("{}", result);
            },
            Err(err) => {
                println!("err: {}", err);
            },
        }
        text.clear()
    }
}


