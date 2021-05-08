/*
   Copyright 2021 BlazifyOrg

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
use bincode::*;
use blazescript::{
    blazevm::vm::VM,
    compiler::bytecode::bytecode::{ByteCode, ByteCodeGen},
    LanguageServer,
};
use std::env::args;
use std::process::exit;
use std::time::SystemTime;

fn main() {
    let file_name = args().nth(1).expect("no path specified");
    let is_compile_mode = if file_name.ends_with(".bze") {
        true
    } else if file_name.ends_with(".bzs") {
        false
    } else {
        eprintln!("Error: File name should end with .bzs(Script) or .bze(Executable)");
        exit(1);
    };

    let cnt = if !is_compile_mode {
        std::fs::read_to_string(file_name.clone()).expect("could not read script")
    } else {
        let btc_raw = std::fs::read(file_name.clone()).expect("could not read executable");
        let bytecode: ByteCode = bincode::deserialize(&btc_raw.clone()[..]).unwrap();
        let mut vm = VM::new(bytecode);
        vm.run();
        println!("{:?}", vm.pop_last());
        let inter_time = SystemTime::now();
        match inter_time.elapsed() {
            Ok(elapsed) => {
                println!(
                    "Time taken for Interpretation Process: {} milliseconds",
                    elapsed.as_millis()
                );
            }
            Err(e) => {
                println!("Error: {:?}", e);
            }
        }
        exit(0);
    };

    let file = Box::leak(file_name.to_owned().into_boxed_str());
    let content = Box::leak(cnt.to_owned().into_boxed_str());

    if !is_compile_mode {
        let btc_time = SystemTime::now();
        let btc = ByteCodeGen::from_source(file, content);
        match btc {
            Ok(b) => {
                let encoded = serialize(&b).unwrap();
                std::fs::write(file_name.clone().replace(".bzs", ".bze"), encoded);
                println!(
                    "Compilation Success: Wrote to {}",
                    file_name.clone().replace(".bzs", ".bze")
                );
                match btc_time.elapsed() {
                    Ok(elapsed) => {
                        println!(
                            "Time taken for Compilation Process: {} milliseconds",
                            elapsed.as_millis()
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
}
