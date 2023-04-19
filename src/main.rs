use std::{env, fs, io::Write, path::PathBuf};

use crate::interpreter::Interpreter;

mod ast;
mod interpreter;
mod lexer;
mod parser;
mod token;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut file: Option<PathBuf> = None;
    if let Some(flag) = args.get(1) {
        if flag == "-f" || flag == "--file" {
            if let Some(path_str) = args.get(2) {
                match path_str.parse() {
                    Ok(path) => file = Some(path),
                    Err(err) => {
                        eprintln!("err: {}", err);
                        return;
                    }
                }
            }
        }
        if flag == "-v" || flag == "--version" {
            println!("version: {}", env!("CARGO_PKG_VERSION"));
            return;
        }
        if flag == "--author" {
            println!("author: {}", env!("CARGO_PKG_AUTHORS"));
            return;
        }
        if flag == "--about" {
            println!("{}", env!("CARGO_PKG_DESCRIPTION"));
            println!("You can find CL Calc at {}", env!("CARGO_PKG_REPOSITORY"));
            return;
        }
        if flag == "-h" || flag == "-?" ||flag == "--help" {
            todo!();
            return;
        }
    }


    let mut interpreter: Interpreter = Interpreter::default();

    if let Some(path) = file {
        let contents = fs::read_to_string(path).expect("Should have been able to read the file");

        let mut out = 0.;

        for line in contents.split('\n') {
            let mut line = line.to_owned();
            let do_out = line.chars().next() == Some('!');
            if do_out {
                line.remove(0);
            }
            out = match interpreter.run(line) {
                Ok(val) => val,
                Err(err) => {
                    eprintln!("err: {}", err);
                    return;
                }
            };
            if do_out {
                println!("!{}", out);
            }
        }
        println!("{}", out);
    } else {
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
                println!("You can define custom functions with name(x) = exprection.");
                println!(
                    "Built in constants:\n- pi: {}\n- e: {}",
                    std::f64::consts::PI,
                    std::f64::consts::E
                );
                println!("You can define custom constants with name = exprection.");
                println!("You can enter !vars to see custom functions and constants.");
                println!("You can run CL Calc with -f or --file followed by a path to run a file to run a list of calculations.");
                continue;
            }

            if text.trim() == "!vars" {
                println!("Functions:");
                for (function, (var, _)) in interpreter.funcs.iter() {
                    println!("    {}({})", function, var);
                }
                println!("Constants:");
                for (constant, number) in interpreter.consts.iter() {
                    println!("    {} = {}", constant, number);
                }
                continue;
            }

            match interpreter.run(text.clone()) {
                Ok(result) => {
                    println!("{}", result);
                }
                Err(err) => {
                    eprintln!("err: {}", err);
                }
            }
            text.clear()
        }
    }
}
