/*
   Copyright 2021 Blazify

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

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
