use std::io::Write;

mod token;
mod lexer;
mod interpreter;
mod parser;
mod ast;
mod program_state;

fn main() {
    let mut text = String::new();

    println!("Welcome To CL Calc a command line calculator tool:\nenter \"!exit\" to exit or \"!help\" for additional help.");

    loop {
        print!("calc> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut text).unwrap();
        if text.trim().is_empty() {
            text.clear();
            continue;
        }

        if text.trim() == "!exit" {
            break;
        }

        if text.trim() == "!help" {
            println!("Built in functions:\n- sqrt(x)\n- ln(x)\n- abs(x)\n- cos(x)\n- sin(x)\n- tan(x)\n- log(x)");
            println!("You can define custom functions with name(x)=exprection.");
            println!("Built in constants:\n- pi: {}\n- e: {}", std::f64::consts::PI, std::f64::consts::E);
            println!("You can define custom constants with name=exprection.");
            println!("You can enter !vars to see custom functions and constants");
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


