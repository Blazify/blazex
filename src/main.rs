#![allow(unused_must_use)]
use blazescript::{
    core::{bytecode::bytecode::ByteCodeGen, vm::vm::VM},
    LanguageServer,
};
use std::env::args;
use std::process::exit;
use std::time::SystemTime;

fn main() {
    let cnt = std::fs::read_to_string(args().nth(1).expect("no path specified"))
        .expect("could not read file");
    let file = Box::leak(args().nth(1).unwrap().to_owned().into_boxed_str());
    let content = Box::leak(cnt.to_owned().into_boxed_str());
    let btc_time = SystemTime::now();
    let btc = ByteCodeGen::from_source(file, content);
    match btc {
        Ok(b) => {
            let mut vm = VM::new(b);
            vm.run();
            match btc_time.elapsed() {
                Ok(elapsed) => {
                    println!(
                        "Time taken for Lexer & Parser & Bytecode & VM: {} nanoseconds",
                        elapsed.as_nanos()
                    );
                }
                Err(e) => {
                    println!("Error: {:?}", e);
                }
            }
            exit(0);
        }
        Err(_) => {}
    }
}
