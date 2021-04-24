#![allow(unused_must_use)]
use blazescript::utils::context::Context;
use blazescript::utils::symbol_table::SymbolTable;
use blazescript::{core::interpreter::interpreter::Interpreter, Interpret};
use rustyline::{error::ReadlineError, Editor};
use std::env::args;
use std::process::exit;

fn main() {
    let global = SymbolTable::new(None);
    let mut ctx = Context::new("<Main>".to_string(), global, Box::new(None), None);
    if Some("--no-repl".to_string()) == args().nth(1) {
        let result = Interpreter::from_source(
            "CLI",
            Box::leak(
                args()
                    .nth(2)
                    .expect("no code given")
                    .to_owned()
                    .into_boxed_str(),
            ),
            &mut ctx,
        );

        match result {
            Ok(n) => {
                println!("{}", n);
                exit(0);
            }
            Err(e) => {
                println!("{}", e);
                exit(1)
            }
        }
    }

    println!("Blazescript REPL.");
    let mut rl = Editor::<()>::new();

    loop {
        let readline = rl.readline(">> ");

        match readline {
            Ok(line) => {
                let result = Interpreter::from_source(
                    "REPL",
                    Box::leak(line.to_owned().into_boxed_str()),
                    &mut ctx,
                );
                match result {
                    Ok(n) => println!("{}", n),
                    Err(e) => println!("{}", e),
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}
