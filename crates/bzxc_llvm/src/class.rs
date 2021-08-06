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
    types::{AnyTypeEnum, BasicTypeEnum},
    values::{BasicValueEnum, PointerValue},
};
use bzxc_shared::{to_static_str, Error, Node, Position, Token, Type};

use crate::{Compiler, Function, Prototype};

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub(crate) fn class_decl(
        &mut self,
        name: Token,
        constructor: (Vec<(Token, Type)>, Box<Node>),
        properties: Vec<(Token, Node)>,
        methods: Vec<(Token, Vec<(Token, Type)>, Node, Type)>,
        pos: (Position, Position),
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        let obj = self.obj_decl(properties)?.into_pointer_value();
        self.classes
            .insert(obj.get_type(), name.value.into_string());

        self.variables.insert(name.value.into_string(), obj);

        for (m_name, args, body, ret) in methods {
            let constr = self.class_method(
                name,
                obj,
                to_static_str(m_name.value.into_string()),
                args,
                body,
                ret.to_llvm_type(self.context),
            );
            self.compile_fn(constr)?;
        }

        let constr = self.class_method(
            name,
            obj,
            "Init",
            constructor.0,
            *constructor.1,
            obj.get_type().into(),
        );

        let compiled_constr = self.compile_fn(constr)?;
        Ok(compiled_constr.as_global_value().as_pointer_value().into())
    }

    pub(crate) fn class_init(
        &mut self,
        name: Token,
        constructor_params: Vec<Node>,
        pos: (Position, Position),
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        let name_str = name.value.into_string();
        let class = self.variables.get(&name_str);
        if let Some(klass) = class {
            let constructor = self.get_function(&(name_str + &"Init")).unwrap().clone();
            let mut compiled_args: Vec<BasicValueEnum> =
                Vec::with_capacity(constructor_params.len() + 1);
            let klas_ = self.builder.build_load(*klass, "base_class_obj");
            let alloca = self.create_entry_block_alloca("class_init", klas_.get_type());
            self.builder.build_store(alloca, klas_);
            compiled_args.push(alloca.into());

            for arg in constructor_params {
                compiled_args.push(self.compile_node(arg)?);
            }

            let call = self
                .builder
                .build_call(constructor, &compiled_args[..], "tmpcall")
                .ok()
                .unwrap();

            Ok(call
                .try_as_basic_value()
                .left_or(self.context.i128_type().const_int(0, false).into()))
        } else {
            Err(self.error(pos, "No class found"))
        }
    }

    fn class_method(
        &mut self,
        name: Token,
        obj: PointerValue<'ctx>,
        method: &'static str,
        args: Vec<(Token, Type)>,
        body: Node,
        ret_type: AnyTypeEnum<'ctx>,
    ) -> Function<'ctx> {
        let mut constr_args = vec![("soul".to_string(), obj.get_type().into())];
        constr_args.extend(
            args.iter()
                .map(|x| {
                    (
                        x.0.value.into_string(),
                        x.1.to_llvm_type(&self.context).to_basic_type_enum(),
                    )
                })
                .collect::<Vec<(String, BasicTypeEnum)>>(),
        );

        let constr = Function {
            prototype: Prototype {
                name: Some(name.value.into_string() + &method),
                args: constr_args,
                ret_type,
            },
            body,
        };

        constr
    }
}
