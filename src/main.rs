#![allow(unused_must_use)]
use blazescript::{
    core::{
        bytecode::bytecode::ByteCodeGen,
        interpreter::{interpreter::Interpreter, value::Value},
        vm::vm::VM,
    },
    LanguageServer,
};
use std::env::args;
use std::process::exit;

fn main() {
    let cnt = std::fs::read_to_string(args().nth(1).expect("no path specified"))
        .expect("could not read file");
    let file = Box::leak(args().nth(1).unwrap().to_owned().into_boxed_str());
    let content = Box::leak(cnt.to_owned().into_boxed_str());
    let mode = args().nth(2);
    if mode.is_some() {
        if mode.unwrap() == "bytecode" {
            let btc = ByteCodeGen::from_source(file, content);
            match btc {
                Ok(b) => {
                    println!(
                        "Source Code: {:?}\nInstructions: {:?}\nConstants: {:?}",
                        content, b.instructions, b.constants
                    );
                    let mut vm = VM::new(b);
                    vm.run();

                    println!("Result: {:?}", vm.pop_last());
                    exit(0);
                }
                Err(_) => {}
            }
        }
    }
    let result = Interpreter::from_source(file, content);

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
