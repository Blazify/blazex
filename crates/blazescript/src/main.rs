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
use blaze_vm::{Konstants, VM};
use bzs_shared::ByteCode;
use bzsc_bytecode::ByteCodeGen;
use bzsc_lexer::Lexer;
use bzsc_parser::Parser;
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::exit;
use std::sync::mpsc::channel;
use std::time::Duration;
use std::time::SystemTime;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct CmdParams {
    #[structopt(parse(from_os_str))]
    pub path: PathBuf,

    #[structopt(long, short = "o")]
    pub out: Option<PathBuf>,

    #[structopt(long, short = "q")]
    pub quiet: bool,

    #[structopt(long, short = "w")]
    pub watch: bool,
}

fn main() {
    let cmd_params = CmdParams::from_args();
    let file_name = cmd_params.path.as_os_str().to_str().unwrap().to_string();
    let is_quiet = cmd_params.quiet;
    let out_file = if cmd_params.out.is_some() {
        if file_name.ends_with(".bze") {
            eprintln!("--out is not valid in this scope");
            exit(1)
        } else {
            let str_out = cmd_params
                .out
                .unwrap()
                .as_os_str()
                .to_str()
                .unwrap()
                .to_string();
            if str_out.ends_with(".bze") {
                str_out
            } else {
                str_out + &".bze"
            }
        }
    } else {
        file_name.clone().replace(".bzs", ".bze")
    };
    let watch = cmd_params.watch;

    let compile = || {
        let time = SystemTime::now();
        if file_name.ends_with(".bzs") {
            if !is_quiet {
                println!("----Blazescript compiler----");
                println!("Version: 0.0.1");
                println!("File: {}", file_name);
            }
            let cnt = std::fs::read_to_string(file_name.clone()).expect("could not read script");

            let name = Box::leak(file_name.to_owned().into_boxed_str());
            let content = Box::leak(cnt.to_owned().into_boxed_str());
            let lexed = Lexer::new(name, content).lex();
            let mut tokens = vec![];
            match lexed {
                Ok(lexed) => {
                    tokens.extend(lexed);
                }
                Err(error) => {
                    error.prettify();
                    if !watch {
                        exit(1);
                    }
                }
            }

            let parsed = Parser::new(tokens).parse();
            if parsed.error.is_some() || parsed.node.is_none() {
                parsed.error.unwrap().prettify();
                if !watch {
                    exit(1);
                }
            }

            let mut bytecode_gen = ByteCodeGen::new();
            bytecode_gen.compile_node(parsed.node.unwrap());

            let mut sym = HashMap::new();
            for (k, v) in &bytecode_gen.variables {
                sym.insert(*v, k.clone());
            }
            let serialized =
                serialize(&(bytecode_gen.bytecode, sym)).expect("serialization of bytecode failed");
            std::fs::write(out_file.clone(), serialized);
            if !is_quiet {
                println!("Compilation Success: Wrote to {}", out_file);
            }
            match time.elapsed() {
                Ok(elapsed) => {
                    if !is_quiet {
                        println!(
                            "Time taken for Compilation Process: {} milliseconds",
                            elapsed.as_millis()
                        );
                    }
                }
                Err(e) => {
                    eprintln!("Error: {:?}", e);
                }
            }
            if !watch {
                exit(0);
            }
        } else if file_name.ends_with(".bze") {
            if !is_quiet {
                println!("----Blaze Virtual Machine----");
                println!("Version: 0.0.1");
                println!("File: {}", file_name);
            }
            let btc_raw = std::fs::read(file_name.clone()).expect("could not read executable");
            let bytecode: (ByteCode, HashMap<u16, String>) =
                deserialize(&btc_raw[..]).expect("deserialization of executable failed");
            let mut vm = VM::new(bytecode.0, None);
            vm.run();
            println!(
                "{}",
                format_print(&vm.pop_last().borrow().clone(), bytecode.1)
            );
            match time.elapsed() {
                Ok(elapsed) => {
                    if !is_quiet {
                        println!(
                            "Time taken for Interpretation Process: {} milliseconds",
                            elapsed.as_millis()
                        );
                    }
                }
                Err(e) => {
                    eprintln!("Error: {:?}", e);
                }
            }
            if !watch {
                exit(0)
            }
        } else {
            eprintln!("Error: File name should end with .bzs(Script) or .bze(Executable)");
            if !watch {
                exit(1)
            };
        };
    };

    compile();

    let (tx, rx) = channel();

    let mut watcher = watcher(tx, Duration::from_secs(1)).unwrap();

    watcher
        .watch(file_name.clone(), RecursiveMode::Recursive)
        .unwrap();

    loop {
        match rx.recv() {
            Ok(DebouncedEvent::Write(_)) => {
                println!("\u{001b}[32;1mChange Detected!\u{001b}[0m");
                compile();
            }
            Ok(_) => {}
            Err(e) => {
                eprintln!("Watch error: {:?}", e);
                if !watch {
                    exit(1);
                }
            }
        }
    }
}

fn format_print(k: &Konstants, props: HashMap<u16, String>) -> String {
    match k {
        Konstants::None => {
            format!("None")
        }
        Konstants::Null => {
            format!("Null")
        }
        Konstants::Int(i) => {
            format!("{}", i)
        }
        Konstants::Float(i) => {
            format!("{}", i)
        }
        Konstants::String(i) => {
            format!("{}", i)
        }
        Konstants::Char(i) => {
            format!("{}", i)
        }
        Konstants::Boolean(i) => {
            format!("{}", i)
        }
        Konstants::Array(x_arr) => {
            let mut res = vec![];
            for x in &x_arr[..] {
                res.push(format_print(x, props.clone()));
            }
            res.join(", ")
        }
        Konstants::Object(x) => {
            let mut str = String::from("{\n    ");
            for (a, b) in x {
                str.push_str(
                    format!(
                        "{}: {},\n",
                        props.get(&(*a as u16)).unwrap(),
                        format_print(b, props.clone())
                    )
                    .as_str(),
                );
                str.push_str("    ");
            }
            str.push_str("\r}");
            str
        }
        Konstants::Function(x, _) => {
            let mut str = String::from("Function<(");
            let mut arr = vec![];
            for a in x {
                arr.push(props.get(a).unwrap().clone());
            }
            str.push_str(arr.join(", ").as_str());
            str.push(')');
            str.push('>');
            str
        }
    }
}
