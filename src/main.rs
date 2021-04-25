#![allow(unused_must_use)]
use blazescript::utils::context::Context;
use blazescript::utils::symbol_table::SymbolTable;
use blazescript::{core::interpreter::interpreter::Interpreter, Interpret};
use rustyline::{error::ReadlineError, Editor};
use std::process::exit;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    path: Option<std::path::PathBuf>,
}

fn main() {
    let global = SymbolTable::new(None);
    let mut ctx = Context::new("<Main>".to_string(), global, Box::new(None), None);

    let args = Cli::from_args();
    if args.path.is_some() {
        let content = std::fs::read_to_string(&args.path.expect("no path specified"))
            .expect("could not read file");
        let result = Interpreter::from_source(
            "CLI",
            Box::leak(content.to_owned().into_boxed_str()),
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
