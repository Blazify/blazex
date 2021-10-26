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
use bzxc_shared::TypedNode;

use crate::{Compiler, Function, Prototype};

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub(crate) fn compile_prototype(&self, proto: &'a Prototype<'ctx>) -> FunctionValue<'ctx> {
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

        fn_val
    }

    pub(crate) fn compile_fn(&mut self, func: Function<'ctx>) -> FunctionValue<'ctx> {
        let parent = self.fn_value_opt.clone();

        let proto = &func.prototype;
        let function = self.compile_prototype(&proto);

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
        let body = self.compile_node(func.body.clone());

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

        if parental_block.is_some() {
            self.builder.position_at_end(parental_block.unwrap());
        }

        self.ret = false;
        self.fn_value_opt = parent;

        if function.verify(true) {
            self.fpm.run_on(&function);

            function
        } else {
            eprintln!(
                "Invalid LLVM IR:\n{}",
                self.module.print_to_string().to_string()
            );
            unsafe {
                function.delete();
            }
            panic!()
        }
    }

    pub(crate) fn fun_decl(
        &mut self,
        name: Option<&'static str>,
        arg_tokens: &'ctx [(&'static str, AnyTypeEnum<'ctx>)],
        body: TypedNode<'ctx>,
        return_type: AnyTypeEnum<'ctx>,
    ) -> BasicValueEnum<'ctx> {
        let func = self.to_func_with_proto(arg_tokens, body, name, return_type);
        let fun = self.compile_fn(func);

        fun.as_global_value().as_pointer_value().into()
    }

    pub(crate) fn fun_call(
        &mut self,
        node_to_call: TypedNode<'ctx>,
        args: &'ctx [&'ctx TypedNode<'ctx>],
    ) -> BasicValueEnum<'ctx> {
        let mut compiled_args = Vec::with_capacity(args.len());

        for arg in args {
            compiled_args.push(self.compile_node(**arg));
        }

        let func = self.compile_node(node_to_call);

        let ptr = func.into_pointer_value();

        self.builder
            .build_call(ptr, &compiled_args[..], "tmpcall")
            .ok()
            .unwrap()
            .try_as_basic_value()
            .left_or(self.null())
    }

    pub(crate) fn fun_extern(
        &self,
        name: &'static str,
        arg_tokens: &'ctx [AnyTypeEnum<'ctx>],
        return_type: AnyTypeEnum<'ctx>,
        var_args: bool,
    ) -> BasicValueEnum<'ctx> {
        let args_types = &arg_tokens
            .iter()
            .map(|x| x.to_basic_type_enum())
            .collect::<Vec<BasicTypeEnum>>()[..];
        self.module
            .add_function(
                &name.to_string(),
                return_type.fn_type(args_types, var_args),
                Some(Linkage::External),
            )
            .as_global_value()
            .as_pointer_value()
            .into()
    }

    pub(crate) fn ret(&mut self, node: Option<&'ctx TypedNode<'ctx>>) -> BasicValueEnum<'ctx> {
        if let Some(ret) = node {
            let rett = self.compile_node(*ret);
            self.builder.build_return(Some(&rett));
            self.ret = true;
            rett
        } else {
            let null = self.null();
            self.builder.build_return(Some(&null));
            self.ret = true;
            null
        }
    }

    pub(crate) fn to_func_with_proto(
        &self,
        arg_tokens: &'ctx [(&'static str, AnyTypeEnum<'ctx>)],
        body_node: TypedNode<'ctx>,
        name: Option<&'static str>,
        return_type: AnyTypeEnum<'ctx>,
    ) -> Function<'ctx> {
        Function {
            prototype: Prototype {
                name: if name.is_none() {
                    None
                } else {
                    Some(name.unwrap().to_string())
                },
                args: arg_tokens
                    .iter()
                    .map(|x| (x.0.to_string(), x.1.to_basic_type_enum()))
                    .collect(),
                ret_type: return_type,
            },
            body: body_node,
        }
    }
}
