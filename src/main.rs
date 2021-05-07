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
use blazescript::{compiler::bytecode::bytecode::ByteCodeGen, LanguageServer};
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::env::args;
use std::io::prelude::*;
use std::process::exit;
use std::time::SystemTime;

#[cxx::bridge]
mod blazevm {
    unsafe extern "C++" {
        include!("blazescript/src/blazevm/vm.h");
        fn VM();
    }
}

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
        let bytes = std::fs::read(file_name.clone()).expect("could not read executable");
        let mut z = ZlibDecoder::new(&bytes[..]);
        let mut s = String::new();
        z.read_to_string(&mut s);
        s
    };
    let file = Box::leak(file_name.to_owned().into_boxed_str());
    let content = Box::leak(cnt.to_owned().into_boxed_str());
    if !is_compile_mode {
        let btc_time = SystemTime::now();
        let btc = ByteCodeGen::from_source(file, content);
        match btc {
            Ok(b) => {
                let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
                e.write_all(
                    &b.instructions
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join("")
                        .into_bytes(),
                );
                std::fs::write(
                    file_name.clone().replace(".bzs", ".bze"),
                    e.finish().unwrap(),
                );
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
    } else {
        let inter_time = SystemTime::now();
        std::env::set_var("bze_name", file_name);
        std::env::set_var("bze_content", format!("{}", cnt));
        blazevm::VM();
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
    }
}
