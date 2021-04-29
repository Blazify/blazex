#![allow(unused_must_use)]
use blazescript::utils::context::Context;
use blazescript::{
    core::interpreter::{interpreter::Interpreter, value::Value},
    LanguageServer,
};
use std::env::args;
use std::process::exit;

fn main() {
    let mut ctx = Context::new("Global".to_string());

    let content = std::fs::read_to_string(args().nth(1).expect("no path specified"))
        .expect("could not read file");
    let result = Interpreter::from_source(
        Box::leak(args().nth(1).unwrap().to_owned().into_boxed_str()),
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
