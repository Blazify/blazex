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
use std::process::exit;
use std::time::SystemTime;
use std::{collections::HashMap, env::args};

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


use structopt::StructOpt;
use std::path::PathBuf;
#[derive(StructOpt,Debug)]
// desc of the program
struct CmdParams{
    // description of symbol
    #[structopt(long)]
    pub symbol: bool,

    // description of module
    #[structopt(long)]
    pub module: Option<PathBuf>,
    // description of module
    #[structopt(long)]
    pub out : Option<PathBuf>,
    // description of workspace
    #[structopt(long)]
    pub workspace: Option<PathBuf>,
}

fn main() {
    let cmd_params = CmdParams::from_args();

    println!("{:?}", cmd_params);
        let file_name = "./blah.bze";
    // TODO integrate
   // let file_name = args().nth(1).expect("no path specified");
    let time = SystemTime::now();

    if file_name.ends_with(".bzs") {
        println!("----Blazescript compiler----");
        println!("Version: 0.0.1");
        println!("File: {}", file_name);
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
                exit(1);
            }
        }

        let parsed = Parser::new(tokens).parse();
        if parsed.error.is_some() || parsed.node.is_none() {
            parsed.error.unwrap().prettify();
            exit(1);
        }

        let mut bytecode_gen = ByteCodeGen::new();
        bytecode_gen.compile_node(parsed.node.unwrap());

        let mut sym = HashMap::new();
        for (k, v) in &bytecode_gen.variables {
            sym.insert(*v, k.clone());
        }
        let serialized =
            serialize(&(bytecode_gen.bytecode, sym)).expect("serialization of bytecode failed");
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
                eprintln!("Error: {:?}", e);
            }
        }
        exit(0);
    } else if file_name.ends_with(".bze") {
        println!("----Blaze Virtual Machine----");
        println!("Version: 0.0.1");
        println!("File: {}", file_name);
        let btc_raw = std::fs::read(file_name.clone()).expect("could not read executable");
        let bytecode: (ByteCode, HashMap<u16, String>) =
            deserialize(&btc_raw[..]).expect("deserialization of executable failed");
        let mut vm = VM::new(bytecode.0, None);
        vm.run();
        println!(
            "Result: {}",
            format_print(&vm.pop_last().borrow().clone(), bytecode.1)
        );
        match time.elapsed() {
            Ok(elapsed) => {
                println!(
                    "Time taken for Interpretation Process: {} milliseconds",
                    elapsed.as_millis()
                );
            }
            Err(e) => {
                eprintln!("Error: {:?}", e);
            }
        }
        exit(0)
    } else {
        eprintln!("Error: File name should end with .bzs(Script) or .bze(Executable)");
        exit(1);
    };
}
