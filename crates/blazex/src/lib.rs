#![allow(dead_code, unused_variables, unused_imports)]

use std::env;
use bzxc_lexer::Lexer;
use bzxc_llvm::Compiler;
use bzxc_llvm_wrapper::support::enable_llvm_pretty_stack_trace;
use bzxc_llvm_wrapper::{
    context::Context,
    execution_engine::JitFunction,
    module::Module,
    passes::PassManager,
    targets::{CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine},
    OptimizationLevel,
};
use bzxc_parser::parser::Parser;
use bzxc_type_system::TypeSystem;
use std::path::Path;
use std::process::Command;

pub fn compile(
    file_name: String,
    cnt: String,
    is_quiet: bool,
    watch: bool,
    out_file: String,
    llvm: bool,
) -> i32 {
    if !is_quiet {
        println!("----BlazeX compiler----");
        println!("Version: {}", env!("CARGO_PKG_VERSION"));
        println!("File: {}", file_name);
    }

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
                return 1;
            }
        }
    }

    let parsed = Parser::new(tokens).parse();
    if parsed.error.is_some() || parsed.node.is_none() {
        parsed.error.unwrap().prettify();
        if !watch {
            return 1;
        }
    }

    let context = Context::create();
    let llvm_node = TypeSystem::new(parsed.node.unwrap(), &context).llvm_node();

    let module = context.create_module(name);
    let builder = context.create_builder();
    let fpm = PassManager::create(&module);

    fpm.add_instruction_combining_pass();
    fpm.add_reassociate_pass();
    fpm.add_gvn_pass();
    fpm.add_cfg_simplification_pass();
    fpm.add_basic_alias_analysis_pass();
    fpm.add_promote_memory_to_register_pass();
    fpm.add_reassociate_pass();
    fpm.initialize();
    enable_llvm_pretty_stack_trace();

    Compiler::init(&context, &builder, &module, &fpm, llvm_node).compile_main();
    if llvm {
        println!("LLVM IR:\n{}", module.print_to_string().to_string());
    }

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

    match target_machine.write_to_file(&module, FileType::Object, &path) {
        Ok(_) => {
            if !is_quiet {
                let out_dir = env::var("OUT_DIR").unwrap();

                Command::new("clang-10")
                    .args([out_file.clone(), format!("{}/libblazex.a", out_dir), format!("-o{}", out_file.replace(".o", ".out"))])
                    .status()
                    .unwrap();
                println!("Compiled executable to {}", out_file.replace(".o", ".out"));
            }
        }
        Err(e) => {
            eprintln!("{}", e.to_string());
            return 1;
        }
    }

    return 0;
}
