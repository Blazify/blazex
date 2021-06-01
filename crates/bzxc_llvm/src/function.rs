use bzxc_shared::{Error, Node};
use inkwell::{
    module::Linkage,
    types::{AnyTypeEnum, BasicTypeEnum},
    values::BasicValueEnum,
    AddressSpace,
};

use crate::{try_any_to_basic, Compiler};

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub(crate) fn fun_decl(&mut self, node: Node) -> Result<BasicValueEnum<'ctx>, Error> {
        match node.clone() {
            Node::FunDef { .. } => {
                let func = self.to_func_with_proto(node.clone())?;
                let fun = self.compile_fn(func)?;

                Ok(fun.as_global_value().as_pointer_value().into())
            }
            _ => panic!(),
        }
    }

    pub(crate) fn fun_call(&mut self, node: Node) -> Result<BasicValueEnum<'ctx>, Error> {
        match node.clone() {
            Node::CallNode { node_to_call, args } => {
                let mut compiled_args = Vec::with_capacity(args.len());

                for arg in args {
                    compiled_args.push(self.compile_node(arg)?);
                }

                let func = self.compile_node(*node_to_call)?;
                if !func.is_pointer_value() {
                    return Err(self.error(
                        node.get_pos(),
                        "Expected a Function pointer found something else",
                    ));
                }

                Ok(self
                    .builder
                    .build_call(func.into_pointer_value(), &compiled_args[..], "tmpcall")
                    .try_as_basic_value()
                    .left_or(self.context.i128_type().const_int(0, false).into()))
            }
            _ => panic!(),
        }
    }

    pub(crate) fn fun_extern(&self, node: Node) -> Result<BasicValueEnum<'ctx>, Error> {
        match node {
            Node::ExternNode {
                name,
                arg_tokens,
                return_type,
                var_args,
            } => {
                let args_types = &arg_tokens
                    .iter()
                    .map(|x| try_any_to_basic(x.to_llvm_type(&self.context)))
                    .collect::<Vec<BasicTypeEnum>>()[..];
                Ok(self
                    .module
                    .add_function(
                        &name.value.into_string(),
                        match return_type.to_llvm_type(&self.context) {
                            AnyTypeEnum::ArrayType(x) => x.fn_type(args_types, var_args),
                            AnyTypeEnum::FloatType(x) => x.fn_type(args_types, var_args),
                            AnyTypeEnum::FunctionType(x) => x
                                .ptr_type(AddressSpace::Generic)
                                .fn_type(args_types, var_args),
                            AnyTypeEnum::IntType(x) => x.fn_type(args_types, var_args),
                            AnyTypeEnum::PointerType(x) => x.fn_type(args_types, var_args),
                            AnyTypeEnum::StructType(x) => x.fn_type(args_types, var_args),
                            AnyTypeEnum::VectorType(x) => x.fn_type(args_types, var_args),
                            AnyTypeEnum::VoidType(x) => x.fn_type(args_types, var_args),
                        },
                        Some(Linkage::External),
                    )
                    .as_global_value()
                    .as_pointer_value()
                    .into())
            }
            _ => panic!(),
        }
    }

    pub(crate) fn ret(&self, node: Node) -> Result<BasicValueEnum<'ctx>, Error> {
        match node {
            Node::ReturnNode { .. } => Err(self.error(node.get_pos(), "Node can't be compiled")),
            _ => panic!(),
        }
    }
}
