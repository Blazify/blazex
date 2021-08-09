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

use bzxc_llvm_wrapper::{
    module::Linkage,
    types::{AnyTypeEnum, BasicTypeEnum},
    values::{BasicValue, BasicValueEnum, FunctionValue},
};
use bzxc_shared::{Error, Node, Position, Token, Type};

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

        let fn_type = ret_type.fn_type(args_types, false);
        let fn_val = self.module.add_function(
            proto.name.as_ref().unwrap_or(&"%".to_string()).as_str(),
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

            self.variables.insert(proto.args[i].0.clone(), alloca);
        }

        self.ret = false;
        let body = self.compile_node(func.body.clone())?;

        if !self.ret {
            if parental_block.is_none() {
                self.ret = true;
                self.builder
                    .build_return(Some(&self.context.i128_type().const_int(0, false)));
            } else {
                if let AnyTypeEnum::PointerType(x) = proto.ret_type {
                    if x == self.null().into_pointer_value().get_type() {
                        self.ret = true;
                        self.builder.build_return(Some(&self.null()));
                    }
                }
            }
        }

        if !self.ret {
            return Err(self.error(func.body.get_pos(), "Expected 'return'"));
        }

        if parental_block.is_some() {
            self.builder.position_at_end(parental_block.unwrap());
        }

        self.ret = false;
        self.fn_value_opt = parent;

        if function.verify(true) {
            self.fpm.run_on(&function);

            Ok(function)
        } else {
            eprintln!(
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

        let ptr = func.into_pointer_value();

        Ok(self
            .builder
            .build_call(ptr, &compiled_args[..], "tmpcall")
            .ok()
            .ok_or(self.error(pos, "Not a function"))?
            .try_as_basic_value()
            .left_or(self.null()))
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
            .map(|x| x.to_llvm_type(&self.context).to_basic_type_enum())
            .collect::<Vec<BasicTypeEnum>>()[..];
        Ok(self
            .module
            .add_function(
                &name.value.into_string(),
                return_type
                    .to_llvm_type(&self.context)
                    .fn_type(args_types, var_args),
                Some(Linkage::External),
            )
            .as_global_value()
            .as_pointer_value()
            .into())
    }

    pub(crate) fn ret(
        &mut self,
        node: Option<Node>,
        pos: (Position, Position),
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        if let Some(ret) = node {
            let rett = self.compile_node(ret)?;
            self.builder.build_return(Some(&rett));
            self.ret = true;
            Ok(rett)
        } else {
            let null = self.null();
            self.builder.build_return(Some(&null));
            self.ret = true;
            Ok(null)
        }
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
                            x.1.to_llvm_type(&self.context).to_basic_type_enum(),
                        )
                    })
                    .collect(),
                ret_type: return_type.to_llvm_type(&self.context),
            },
            body: body_node,
        })
    }
}
