use std::mem::MaybeUninit;

use std::env;
use std::process::Command;

use llvm_sys::core::{
    LLVMContextCreate, LLVMContextDispose, LLVMCreateBuilderInContext,
    LLVMCreateFunctionPassManager, LLVMCreateModuleProviderForExistingModule, LLVMDisposeBuilder,
    LLVMDisposeMessage, LLVMDisposeModule, LLVMDumpModule, LLVMInitializeFunctionPassManager,
    LLVMModuleCreateWithNameInContext, LLVMSetDataLayout, LLVMSetTarget,
};
use llvm_sys::error_handling::LLVMEnablePrettyStackTrace;
use llvm_sys::target::{
    LLVMCopyStringRepOfTargetData, LLVM_InitializeAllAsmParsers, LLVM_InitializeAllAsmPrinters,
    LLVM_InitializeAllTargetInfos, LLVM_InitializeAllTargetMCs, LLVM_InitializeAllTargets,
};
use llvm_sys::target_machine::LLVMCodeGenFileType::LLVMObjectFile;
use llvm_sys::target_machine::LLVMCodeGenOptLevel::LLVMCodeGenLevelAggressive;
use llvm_sys::target_machine::LLVMCodeModel::LLVMCodeModelDefault;
use llvm_sys::target_machine::LLVMRelocMode::LLVMRelocDefault;
use llvm_sys::target_machine::{
    LLVMCreateTargetDataLayout, LLVMCreateTargetMachine, LLVMGetDefaultTargetTriple,
    LLVMGetHostCPUFeatures, LLVMGetTargetFromTriple, LLVMTargetMachineEmitToFile, LLVMTargetRef,
};
use llvm_sys::transforms::scalar::{
    LLVMAddBasicAliasAnalysisPass, LLVMAddCFGSimplificationPass, LLVMAddGVNPass,
    LLVMAddInstructionCombiningPass, LLVMAddReassociatePass,
};
use llvm_sys::transforms::util::LLVMAddPromoteMemoryToRegisterPass;

use bzxc_lexer::Lexer;
use bzxc_llvm::Compiler;
use bzxc_parser::parser::Parser;
use bzxc_shared::to_c_str;
use bzxc_type_system::TypeSystem;

pub unsafe fn compile(
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

    let context = LLVMContextCreate();
    let llvm_node = TypeSystem::new(parsed.node.unwrap(), context).llvm_node();

    let module = LLVMModuleCreateWithNameInContext(to_c_str(name).as_ptr(), context);
    let builder = LLVMCreateBuilderInContext(context);

    let fpm = LLVMCreateFunctionPassManager(LLVMCreateModuleProviderForExistingModule(module));
    LLVMAddInstructionCombiningPass(fpm);
    LLVMAddReassociatePass(fpm);
    LLVMAddGVNPass(fpm);
    LLVMAddCFGSimplificationPass(fpm);
    LLVMAddBasicAliasAnalysisPass(fpm);
    LLVMAddPromoteMemoryToRegisterPass(fpm);
    LLVMInitializeFunctionPassManager(fpm);

    LLVMEnablePrettyStackTrace();

    Compiler::init(context, builder, module, fpm, llvm_node).compile_main();
    if llvm {
        LLVMDumpModule(module);
    }

    LLVM_InitializeAllTargetInfos();
    LLVM_InitializeAllTargets();
    LLVM_InitializeAllTargetMCs();
    LLVM_InitializeAllAsmParsers();
    LLVM_InitializeAllAsmPrinters();

    let mut errors = MaybeUninit::uninit();
    let mut target: MaybeUninit<LLVMTargetRef> = MaybeUninit::uninit();
    let mut ret = LLVMGetTargetFromTriple(
        LLVMGetDefaultTargetTriple(),
        target.as_mut_ptr(),
        errors.as_mut_ptr(),
    );
    if ret == 1 {
        LLVMDisposeMessage(errors.assume_init());
    }
    let machine = LLVMCreateTargetMachine(
        target.assume_init(),
        LLVMGetDefaultTargetTriple(),
        to_c_str("generic").as_ptr(),
        LLVMGetHostCPUFeatures(),
        LLVMCodeGenLevelAggressive,
        LLVMRelocDefault,
        LLVMCodeModelDefault,
    );

    LLVMSetTarget(module, LLVMGetDefaultTargetTriple());
    let datalayout = LLVMCreateTargetDataLayout(machine);
    let datalayout_str = LLVMCopyStringRepOfTargetData(datalayout);
    LLVMSetDataLayout(module, datalayout_str);
    LLVMDisposeMessage(datalayout_str);

    ret = LLVMTargetMachineEmitToFile(
        machine,
        module,
        to_c_str(out_file.as_str()).as_ptr() as *mut _,
        LLVMObjectFile,
        errors.as_mut_ptr(),
    );
    if ret == 1 {
        LLVMDisposeMessage(errors.assume_init());
    }

    LLVMDisposeBuilder(builder);
    LLVMDisposeModule(module);
    LLVMContextDispose(context);

    let mut dir = env::current_exe().ok().unwrap();
    dir.pop();
    if dir.ends_with("bin") {
        dir.pop();
    }
    dir.push("stdlib");

    assert!(dir.is_dir());
    Command::new("clang-10")
        .args([
            out_file.clone(),
            format!("{}/libblazex.a", dir.to_str().unwrap()),
            format!("-o{}", out_file.replace(".o", ".out")),
        ])
        .status()
        .unwrap();
    println!("Compiled executable to {}", out_file.replace(".o", ".out"));

    return 0;
}
