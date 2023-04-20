#![feature(iter_intersperse)]

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
        if flag == "-v" || flag == "--version" {
            println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
            return;
        }
        if flag == "--about" {
            let authors = env!("CARGO_PKG_AUTHORS");
            println!("CL Calc was made by {authors} as a project to learn how to make an interpreter. I know it is inefficient and redundant but it was a fun side project.");
            println!(
                "You can find the CL Calc repo at {}",
                env!("CARGO_PKG_REPOSITORY")
            );
            return;
        }
        if flag == "-h" || flag == "-?" || flag == "--help" {
            println!("{}\n", env!("CARGO_PKG_DESCRIPTION"));
            println!("Usage {} [PATH] [OPTIONS]\n", env!("CARGO_PKG_NAME"));
            println!("PATH: Run file of calculations");
            println!("OPTIONS:");
            println!("  -v, --version   Print version info");
            println!(
                "      --about     Print information about {}",
                env!("CARGO_PKG_NAME")
            );
            println!("  -h, -?, --help  Print this message");

            return;
        }
        if let Some(path_str) = args.get(1) {
            match path_str.parse() {
                Ok(path) => file = Some(path),
                Err(err) => {
                    eprintln!("err: {}", err);
                    return;
                }
            }
        }
    }

    let mut interpreter: Interpreter = Interpreter::default();

    if let Some(path) = file {
        match interpreter.run_file(path) {
            Ok((debug_out, res)) => {
                for out in debug_out {
                    println!("! {}", out);
                }
                println!("{}", res);
            }
            Err(err) => eprintln!("err: {}", err),
        }
        return;
    } else {
        let mut text = String::new();

        println!("Welcome To CL Calc a command line calculator tool:\nEnter \"!exit\" to exit or \"!help\" for additional help.\nRun with \"-?\" to see valid arguments.");

        loop {
            text.clear();
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
                println!("The built in functions are:\n- sqrt(x)\n- ln(x)\n- abs(x)\n- cos(x)\n- sin(x)\n- tan(x)\n- log(x)");
                println!("You can define custom functions with name(x) = expression.");
                println!(
                    "The built in constants are:\n- pi: {}\n- e: {}\nans: the result of the previous calculation",
                    std::f64::consts::PI,
                    std::f64::consts::E
                );
                println!("You can define custom constants with name = expression.");
                println!("You can enter !vars to see custom functions and constants.");
                println!("You can run CL Calc followed by a path to run a file to run a list of calculations.");
                println!("You can enter !file <path> to run a list of calculations.");
                println!("You can enter !out <path> to output all successfully run commands.");
                continue;
            }

            if text.trim() == "!vars" {
                println!("Functions:");
                for (function, (var, _)) in interpreter.funcs.iter() {
                    println!("    {}({})", function, var);
                }
                if interpreter.funcs.is_empty() {
                    println!("    None");
                }
                println!("Constants:");
                for (constant, number) in interpreter.consts.iter() {
                    println!("    {} = {}", constant, number);
                }
                if interpreter.consts.is_empty() {
                    println!("    None");
                }
                continue;
            }

            if text.trim().get(0..6) == Some("!file ") {
                match interpreter.run_file(text.trim().get(6..).unwrap_or_default().into()) {
                    Ok((debug_out, res)) => {
                        for out in debug_out {
                            println!("! {}", out);
                        }
                        println!("{}", res);
                    }
                    Err(err) => eprintln!("err: {}", err),
                }

                continue;
            }

            if text.trim().get(0..5) == Some("!out ") {

                let path: PathBuf =
                    if let Some(Ok(path)) = text.trim().get(5..).map(|arg| arg.try_into()) {
                        path
                    } else {
                        eprintln!("err: No Path");
                        continue;
                    };

                if let Err(err) = fs::write(path, interpreter.executed_lines.join("\n")) {
                    eprintln!("err: {}", err);
                }

                continue;
            }

            if text.trim().chars().next() == Some('!') {
                println!("err: Invalid Command {}", text);
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
        }
    }
}
