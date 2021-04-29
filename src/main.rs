#![allow(unused_must_use)]
use blazescript::utils::context::Context;
use blazescript::{
    core::interpreter::{interpreter::Interpreter, value::Value},
    LanguageServer,
};
use rustyline::{error::ReadlineError, Editor};
use std::process::exit;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    path: Option<std::path::PathBuf>,
}

fn main() {
    let mut ctx = Context::new("Global".to_string());

    let args = Cli::from_args();
    if args.path.is_some() {
        let content = std::fs::read_to_string(&args.path.clone().expect("no path specified"))
            .expect("could not read file");
        let result = Interpreter::from_source(
            Box::leak(
                args.path
                    .unwrap()
                    .into_os_string()
                    .into_string()
                    .unwrap()
                    .to_owned()
                    .into_boxed_str(),
            ),
            Box::leak(content.to_owned().into_boxed_str()),
            &mut ctx,
        );

        match result {
            Ok(n) => {
                if n == Value::Null {
                    exit(0);
                }
                println!("{}", n);
                exit(0);
            }
            Err(e) => {
                println!("{}", e.prettify());
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
                    Ok(n) => {
                        if n != Value::Null {
                            println!("{}", n)
                        }
                    }
                    Err(e) => println!("{}", e.prettify()),
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
