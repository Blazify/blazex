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

#![allow(dead_code)]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use bzc_lexer::Lexer;
use bzc_llvm::Compiler;
use bzc_llvm::Function;
use bzc_llvm::Prototype;
use bzc_parser::parser::Parser;
use inkwell::context::Context;
use inkwell::execution_engine::JitFunction;
use inkwell::module::Linkage;
use inkwell::module::Module;
use inkwell::passes::PassManager;
use inkwell::targets::CodeModel;
use inkwell::targets::FileType;
use inkwell::targets::InitializationConfig;
use inkwell::targets::RelocMode;
use inkwell::targets::Target;
use inkwell::targets::TargetMachine;
use inkwell::types::BasicTypeEnum;
use inkwell::AddressSpace;
use inkwell::OptimizationLevel;
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::path::Path;
use std::path::PathBuf;
use std::process::exit;
use std::sync::mpsc::channel;
use std::time::Duration;
use std::time::SystemTime;
use structopt::StructOpt;

/*
* Arguments Struct for CLI Argument Parsing
*/
#[derive(StructOpt)]
struct CmdParams {
    /*
     * Path to the Blaze Source code
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
}

/*
* Entry Point of the Compiler
*/
fn main() {
    let cmd_params = CmdParams::from_args();
    let file_name = cmd_params.path.as_os_str().to_str().unwrap().to_string();
    let is_quiet = cmd_params.quiet;
    let out_file = if let Some(out) = cmd_params.out {
        if out.ends_with(".o") {
            out.as_os_str().to_str().unwrap().to_string()
        } else {
            out.as_os_str().to_str().unwrap().to_string() + ".o"
        }
    } else {
        file_name.clone().replace(".bz", ".o")
    };
    let watch = cmd_params.watch;

    /*
     * Compiling to Bytecode or Intepreting Bytecode
     */
    let compile = || {
        let time = SystemTime::now();
        if !is_quiet {
            println!("----Blaze compiler----");
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

        let context = Context::create();
        let module = context.create_module(name);
        let builder = context.create_builder();

        let fpm = PassManager::create(&module);

        fpm.add_instruction_combining_pass();
        fpm.add_reassociate_pass();
        fpm.add_gvn_pass();
        fpm.add_cfg_simplification_pass();
        fpm.add_basic_alias_analysis_pass();
        fpm.add_promote_memory_to_register_pass();
        fpm.add_instruction_combining_pass();
        fpm.add_reassociate_pass();

        fpm.initialize();

        let func = Function {
            body: parsed.node.unwrap(),
            prototype: Prototype {
                name: Some(String::from("main")),
                args: vec![],
                ret_type: context.i128_type().into(),
            },
        };

        let str_type = context.i8_type().ptr_type(AddressSpace::Generic);
        let i32_type = context.i32_type();
        let printf_type = i32_type.fn_type(&[BasicTypeEnum::PointerType(str_type)], true);
        module.add_function("printf", printf_type, Some(Linkage::External));

        match Compiler::compile(&context, &builder, &module, &fpm, func) {
            Ok(_) => {
                println!("LLVM IR:\n{}", module.print_to_string().to_string());

                /*
                 * Uncomment if you want to test what the output is...
                 */
                // jit(module.clone());

                let path = Path::new(&out_file);

                Target::initialize_all(&InitializationConfig::default());
                let target = Target::from_name("x86-64").unwrap();
                let target_machine = target
                    .create_target_machine(
                        &TargetMachine::get_default_triple(),
                        "x86-64",
                        TargetMachine::get_host_cpu_features().to_string().as_str(),
                        OptimizationLevel::Aggressive,
                        RelocMode::Default,
                        CodeModel::Default,
                    )
                    .unwrap();

                target_machine
                    .write_to_file(&module, FileType::Object, &path)
                    .ok();

                println!("Wrote object file to {}", out_file);
            }
            Err(err) => {
                err.prettify();
            }
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
                if !watch {
                    exit(1);
                }
            }
        }

        if !watch {
            exit(0);
        }
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

fn jit<'ctx>(module: Module<'ctx>) {
    let jit_engine = module
        .create_jit_execution_engine(OptimizationLevel::Aggressive)
        .unwrap();

    unsafe {
        let main: JitFunction<unsafe extern "C" fn() -> i128> =
            jit_engine.get_function("main").unwrap();
        println!("{}", main.call());
    }
}
