use bzxc_lexer::Lexer;
use bzxc_llvm::Compiler;
use bzxc_llvm::Function;
use bzxc_llvm::Prototype;
use bzxc_parser::parser::Parser;
use inkwell::support::enable_llvm_pretty_stack_trace;
use inkwell::{
    context::Context,
    execution_engine::JitFunction,
    module::{Linkage, Module},
    passes::PassManager,
    targets::{CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine},
    AddressSpace, OptimizationLevel,
};
use std::path::Path;
use std::time::SystemTime;

pub fn compile(
    file_name: String,
    cnt: String,
    is_quiet: bool,
    watch: bool,
    out_file: String,
    llvm: bool,
    jit_: bool,
) -> i32 {
    let time = SystemTime::now();
    if !is_quiet {
        println!("----BlazeX compiler----");
        println!("Version: 0.0.1");
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

    let func = Function {
        body: parsed.node.unwrap(),
        prototype: Prototype {
            name: Some(String::from("main")),
            args: vec![],
            ret_type: context.i128_type().into(),
        },
    };

    if !jit_ {
        module.add_function(
            "printf",
            context.i32_type().fn_type(
                &[context.i8_type().ptr_type(AddressSpace::Generic).into()],
                true,
            ),
            Some(Linkage::External),
        );
    }

    match Compiler::init(&context, &builder, &module, &fpm, func).compile_main() {
        Ok(_) => {
            if llvm {
                println!("LLVM IR:\n{}", module.print_to_string().to_string());
            }

            if jit_ {
                jit(module.clone());
                if !watch {
                    return 1;
                }
            };

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
                        println!("Wrote object file to {}", out_file);
                    }
                }
                Err(e) => {
                    eprintln!("{}", e.to_string());
                    return 1;
                }
            }
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
            return 0;
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
            if !watch {
                return 1;
            }
        }
    }
    return 0;
}

fn jit<'ctx>(module: Module<'ctx>) {
    let jit_engine = module
        .create_jit_execution_engine(OptimizationLevel::Aggressive)
        .unwrap();

    unsafe {
        let main: JitFunction<unsafe extern "C" fn() -> i128> =
            jit_engine.get_function("main").unwrap();
        main.call();
    }
}
