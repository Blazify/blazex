use bzxc_shared::{any_fn_type, try_any_to_basic, Error, Node, Position, Token, Type};
use inkwell::{
    module::Linkage,
    types::{AnyTypeEnum, BasicTypeEnum},
    values::{BasicValue, BasicValueEnum, FunctionValue},
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

        let fn_type = any_fn_type(ret_type, args_types, false);
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
            println!(
                "Invalid LLVM IR:\n{}",
                self.module.print_to_string().to_string()
            );
            unsafe {
                function.delete();
            }

            Err(self.error(func.body.get_pos(), "Invalid generated function"))
        }
    }

    pub(crate) fn fun_decl(
        &mut self,
        arg_tokens: Vec<(Token, Type)>,
        body_node: Node,
        name: Option<Token>,
        return_type: Type,
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        let func = self.to_func_with_proto(arg_tokens, body_node, name, return_type)?;
        let fun = self.compile_fn(func)?;

        Ok(fun.as_global_value().as_pointer_value().into())
    }

    pub(crate) fn fun_call(
        &mut self,
        node_to_call: Node,
        args: Vec<Node>,
        pos: (Position, Position),
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        let mut compiled_args = Vec::with_capacity(args.len());

        for arg in args {
            compiled_args.push(self.compile_node(arg)?);
        }

        let func = self.compile_node(node_to_call)?;
        if !func.is_pointer_value() {
            return Err(self.error(pos, "Expected a Function pointer found something else"));
        }

        Ok(self
            .builder
            .build_call(func.into_pointer_value(), &compiled_args[..], "tmpcall")
            .try_as_basic_value()
            .left_or(self.context.i128_type().const_int(0, false).into()))
    }

    pub(crate) fn fun_extern(
        &self,
        name: Token,
        arg_tokens: Vec<Type>,
        return_type: Type,
        var_args: bool,
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        let args_types = &arg_tokens
            .iter()
            .map(|x| try_any_to_basic(x.to_llvm_type(&self.context)))
            .collect::<Vec<BasicTypeEnum>>()[..];
        Ok(self
            .module
            .add_function(
                &name.value.into_string(),
                any_fn_type(return_type.to_llvm_type(self.context), args_types, var_args),
                Some(Linkage::External),
            )
            .as_global_value()
            .as_pointer_value()
            .into())
    }

    pub(crate) fn ret(
        &self,
        node: Option<Node>,
        pos: (Position, Position),
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        Err(self.error(pos, "Node can't be compiled"))
    }

    pub(crate) fn to_func_with_proto(
        &self,
        arg_tokens: Vec<(Token, Type)>,
        body_node: Node,
        name: Option<Token>,
        return_type: Type,
    ) -> Result<Function<'ctx>, Error> {
        Ok(Function {
            prototype: Prototype {
                name: if name.is_none() {
                    None
                } else {
                    Some(name.unwrap().value.into_string())
                },
                args: arg_tokens
                    .iter()
                    .map(|x| {
                        (
                            x.0.value.into_string(),
                            try_any_to_basic(x.1.to_llvm_type(&self.context)),
                        )
                    })
                    .collect(),
                ret_type: return_type.to_llvm_type(&self.context),
            },
            body: body_node,
        })
    }
}
