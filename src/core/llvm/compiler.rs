use crate::core::llvm::compiler_result::CompilerResult;
use crate::core::parser::nodes::Node;
use crate::utils::constants::{DynType, Tokens};
use crate::utils::error::Error;
use crate::Compile;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::JitFunction;
use inkwell::types::IntType;
use inkwell::values::AnyValue;
use inkwell::OptimizationLevel;

pub struct Jit;

impl Compile for Jit {
    fn from_ast(node: &Node) -> Result<i64, String> {
        let context = Context::create();
        let module = context.create_module("<main>");
        let builder = context.create_builder();
        let execution_engine = module
            .create_jit_execution_engine(OptimizationLevel::None)
            .unwrap();
        let i64_type = context.i64_type();
        let bool_type = context.bool_type();
        let fn_type = i64_type.fn_type(&[], false);
        let function = module.add_function("jit", fn_type, None);
        let basic_block = context.append_basic_block(function, "entry");
        builder.position_at_end(basic_block);

        let recursive_builder = RecursiveBuilder::new(i64_type, bool_type, &builder);
        let return_value = recursive_builder.build(node);
        if return_value.error.clone().is_some() {
            return Err(return_value.error.unwrap().prettify());
        }
        builder.build_return(Some(&return_value.value.unwrap()));
        println!(
            "Generated LLVM IR: {}",
            function.print_to_string().to_string()
        );

        unsafe {
            let jit_function: JitFunction<unsafe extern "C" fn() -> i64> =
                execution_engine.get_function("jit").unwrap();
            Ok(jit_function.call())
        }
    }
}

struct RecursiveBuilder<'a> {
    i64_type: IntType<'a>,
    bool_type: IntType<'a>,
    builder: &'a Builder<'a>,
}

impl<'a> RecursiveBuilder<'a> {
    pub fn new(i64_type: IntType<'a>, bool_type: IntType<'a>, builder: &'a Builder) -> Self {
        Self {
            i64_type,
            bool_type,
            builder,
        }
    }

    pub fn build(&self, node: &Node) -> CompilerResult {
        let res = CompilerResult::new();
        match node {
            Node::NumberNode {
                token,
                pos_start,
                pos_end,
            } => match token.value {
                DynType::Int(i) => res.success(self.i64_type.const_int(i as u64, true)),
                _ => res.failure(Error::new(
                    "Compilation Error",
                    pos_start.clone(),
                    pos_end.clone(),
                    "Unexpected token, expected a token of type int or float.",
                )),
            },
            Node::UnaryNode {
                node,
                op_token,
                pos_start,
                pos_end,
            } => {
                let child = res.register(self.build(node));
                if res.error.is_some() {
                    return res;
                }
                match op_token.r#type {
                    Tokens::Plus => res.success(child.unwrap()),
                    Tokens::Minus => res.success(child.unwrap().const_neg()),
                    _ => res.failure(Error::new(
                        "Compilation Error",
                        pos_start.clone(),
                        pos_end.clone(),
                        "Unexpected token, expected a plus or minus.",
                    )),
                }
            }
            Node::BinOpNode {
                left,
                right,
                op_token,
                pos_start,
                pos_end,
            } => {
                let built_left = res.register(self.build(left));
                if res.error.is_some() {
                    return res;
                }
                let built_right = res.register(self.build(right));
                if res.error.is_some() {
                    return res;
                }

                match op_token.r#type {
                    Tokens::Plus => res.success(self.builder.build_int_add(
                        built_left.unwrap(),
                        built_right.unwrap(),
                        "plus_temp",
                    )),
                    Tokens::Minus => res.success(self.builder.build_int_sub(
                        built_left.unwrap(),
                        built_right.unwrap(),
                        "minus_temp",
                    )),
                    Tokens::Multiply => res.success(self.builder.build_int_mul(
                        built_left.unwrap(),
                        built_right.unwrap(),
                        "multiply_temp",
                    )),
                    Tokens::Divide => res.success(self.builder.build_int_signed_div(
                        built_left.unwrap(),
                        built_right.unwrap(),
                        "divide_signed_temp",
                    )),
                    _ => res.failure(Error::new(
                        "Compilation Error",
                        pos_start.clone(),
                        pos_end.clone(),
                        "Unexpected token, expected plus, minus, multiply or divide.",
                    )),
                }
            }
            Node::BooleanNode {
                token,
                pos_start,
                pos_end,
            } => match token.value {
                DynType::Boolean(b) => res.success(self.bool_type.const_int(u64::from(b), true)),
                _ => res.failure(Error::new(
                    "Compilation Error",
                    pos_start.clone(),
                    pos_end.clone(),
                    "Unexpected token, expected a token of type boolean.",
                )),
            },
            _ => panic!("Not implemented"),
        }
    }
}
