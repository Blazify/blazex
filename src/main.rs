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
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use bincode::{deserialize, serialize};
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
    let time = SystemTime::now();
    if file_name.ends_with(".bzs") {
        println!("----Blazescript compiler----");
        println!("Version: 0.0.1");
        println!("File: {}", file_name);
        let cnt = std::fs::read_to_string(file_name.clone()).expect("could not read script");

        let file = Box::leak(file_name.to_owned().into_boxed_str());
        let content = Box::leak(cnt.to_owned().into_boxed_str());

        let btc = ByteCodeGen::from_source(file, content);
        match btc {
            Ok(b) => {
                let serialized = serialize(&b).expect("serialization of bytecode failed");
                std::fs::write(file_name.clone().replace(".bzs", ".bze"), serialized);
                println!(
                    "Compilation Success: Wrote to {}",
                    file_name.clone().replace(".bzs", ".bze")
                );
                match time.elapsed() {
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
    } else if file_name.ends_with(".bze") {
        println!("----Blaze Virtual Machine----");
        println!("Version: 0.0.1");
        println!("File: {}", file_name);
        let btc_raw = std::fs::read(file_name.clone()).expect("could not read executable");
        let bytecode: ByteCode =
            deserialize(&btc_raw[..]).expect("deserialization of executable failed");
        let mut vm = VM::new(bytecode, None);
        vm.run();
        println!("Result: {:?}", vm.pop_last());
        match time.elapsed() {
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
        exit(0)
    } else {
        eprintln!("Error: File name should end with .bzs(Script) or .bze(Executable)");
        exit(1);
    };
}
