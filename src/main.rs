use std::{io::Write, path::PathBuf, fs};

use clap::Parser;

use crate::interpreter::Interpreter;

mod ast;
mod interpreter;
mod lexer;
mod parser;
mod token;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to file with calculator commands
    #[arg(short, long)]
    file: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    let mut interpreter: Interpreter = Interpreter::default();

    if let Some(path) = args.file {
        
        let contents = fs::read_to_string(path)
        .expect("Should have been able to read the file");

        let mut out = 0.;

        for line in contents.split('\n') {
            out = match interpreter.run(line.to_owned()) {
                Ok(val) => val,
                Err(err) => {
                    println!("err: {}", err);
                    return;
                },
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
                    println!("err: {}", err);
                    //println!("backtrace: {}", err.backtrace())
                }
            }
            text.clear()
        }
    }
}
