use bzxc_shared::{try_any_to_basic, Error, Node};
use inkwell::{
    module::Linkage,
    types::{AnyTypeEnum, BasicTypeEnum},
    values::{BasicValue, BasicValueEnum, FunctionValue},
    AddressSpace,
};
use rand::{distributions::Alphanumeric, Rng};

use crate::{Compiler, Function, Prototype};

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub(crate) fn compile_prototype(
        &self,
        proto: &'a Prototype<'ctx>,
    ) -> Result<FunctionValue<'ctx>, Error> {
        let ret_type = proto.ret_type;
        let args_types = proto
            .args
            .iter()
            .map(|x| x.1)
            .collect::<Vec<BasicTypeEnum>>();
        let args_types = args_types.as_slice();

        let fn_type = match ret_type {
            AnyTypeEnum::ArrayType(x) => x.fn_type(args_types, false),
            AnyTypeEnum::FloatType(x) => x.fn_type(args_types, false),
            AnyTypeEnum::FunctionType(x) => {
                x.ptr_type(AddressSpace::Generic).fn_type(args_types, false)
            }
            AnyTypeEnum::IntType(x) => x.fn_type(args_types, false),
            AnyTypeEnum::PointerType(x) => x.fn_type(args_types, false),
            AnyTypeEnum::StructType(x) => x.fn_type(args_types, false),
            AnyTypeEnum::VectorType(x) => x.fn_type(args_types, false),
            AnyTypeEnum::VoidType(x) => x.fn_type(args_types, false),
        };
        let fn_val = self.module.add_function(
            proto
                .name
                .as_ref()
                .unwrap_or(
                    &rand::thread_rng()
                        .sample_iter(&Alphanumeric)
                        .take(20)
                        .map(char::from)
                        .collect(),
                )
                .as_str(),
            fn_type,
            None,
        );

        for (i, arg) in fn_val.get_param_iter().enumerate() {
            arg.set_name(proto.args[i].0.as_str());
        }

        Ok(fn_val)
    }

    pub(crate) fn compile_fn(
        &mut self,
        func: Function<'ctx>,
    ) -> Result<FunctionValue<'ctx>, Error> {
        let parent = self.fn_value_opt.clone();

        let proto = &func.prototype;
        let function = self.compile_prototype(&proto)?;

        let parental_block = self.builder.get_insert_block();

        let entry = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry);
        self.fn_value_opt = Some(function);

        self.variables.reserve(proto.args.len());

        for (i, arg) in function.get_param_iter().enumerate() {
            let arg_name = proto.args[i].0.as_str();
            let alloca = self.create_entry_block_alloca(arg_name, arg.get_type());

            self.builder.build_store(alloca, arg);

            self.variables
                .insert(proto.args[i].0.clone(), (alloca, false));
        }

        let body = self.compile_node(func.body.clone())?;

        if let AnyTypeEnum::VoidType(_) = func.prototype.ret_type {
            self.builder.build_return(None);
        } else {
            self.builder.build_return(Some(&body));
        }

        if parental_block.is_some() {
            self.builder.position_at_end(parental_block.unwrap());
        }

        self.fn_value_opt = parent;

        if function.verify(true) {
            self.fpm.run_on(&function);

            Ok(function)
        } else {
            println!("{}", self.module.print_to_string().to_string());
            unsafe {
                function.delete();
            }

            Err(self.error(func.body.get_pos(), "Invalid generated function"))
        }
    }

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
