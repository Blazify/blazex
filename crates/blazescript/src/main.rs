/*
 * Copyright 2020 to 2021 BlazifyOrg
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *    http://www.apache.org/licenses/LICENSE-2.0
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
*/

#![allow(unused_must_use)]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use bincode::{deserialize, serialize};
use blaze_vm::VM;
use blazescript::format_print;
use bzs_shared::ByteCode;
use bzsc_bytecode::ByteCodeGen;
use bzsc_lexer::Lexer;
use bzsc_parser::parser::Parser;
use cfg_if::cfg_if;
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::exit;
use std::sync::mpsc::channel;
use std::time::Duration;
use std::time::SystemTime;
use structopt::StructOpt;

/*
* Arguments Struct for CLI Argument Parsing
*/
#[derive(StructOpt, Debug)]
struct CmdParams {
    /*
     * Path to the BlazeScript or Executable
     */
    #[structopt(parse(from_os_str))]
    pub path: PathBuf,

    /*
     * Name of compiled file (Default: input_file.bze)
     */
    #[structopt(parse(from_os_str), long, short = "o")]
    pub out: Option<PathBuf>,

    /*
     * Whether there should be any logging in console (Default: false)
     */
    #[structopt(long, short = "q")]
    pub quiet: bool,

    /*
     * Whether the compiler should compile/run on file changes (Default: false)
     */
    #[structopt(long, short = "w")]
    pub watch: bool,

    /*
     * Whether the compiler should compile to llvm (Default: false)
     */
    #[structopt(long, short = "l")]
    pub llvm: bool,
}

/*
* Entry Point of the Compiler
*/
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
    let _is_llvm = cmd_params.llvm;

    /*
     * Compiling to Bytecode or Intepreting Bytecode
     */
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

            cfg_if! {
                if #[cfg(feature = "llvm-jit")] {
                    use bzsc_llvm::init_compiler;
                    if _is_llvm {
                        init_compiler(parsed.node.unwrap());

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
                                exit(1);
                            }
                        }

                        exit(0);
                    }
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

    /*
     * Triggering the compiler on file change
     */
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
