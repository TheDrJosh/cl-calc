use std::io::Write;

use interpreter::Interpreter;

mod token;
mod lexer;
mod interpreter;




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

        let mut inter = match Interpreter::new(text.clone()) {
            Ok(inter) => {
                inter
            },
            Err(err) => {
                println!("{}", err);
                text.clear();
                continue;
            },
        };
        match inter.calc() {
            Ok(result) => {
                println!("{}", result);
            },
            Err(err) => {
                println!("{}", err);
            },
        }
        text.clear()
    }
}


