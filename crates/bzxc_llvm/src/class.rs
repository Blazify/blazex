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
use bzxc_shared::TypedNode;

use crate::{Compiler, Function, Prototype};

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub(crate) fn class_decl(
        &mut self,
        name: &'static str,
        constructor: (
            &'ctx [(&'static str, AnyTypeEnum<'ctx>)],
            &'ctx TypedNode<'ctx>,
        ),
        properties: &'ctx [(&'static str, &'ctx TypedNode<'ctx>)],
        methods: &'ctx [(
            &'static str,
            &'ctx [(&'static str, AnyTypeEnum<'ctx>)],
            &'ctx TypedNode<'ctx>,
            AnyTypeEnum<'ctx>,
        )],
    ) -> BasicValueEnum<'ctx> {
        let obj = self.obj_decl(properties).into_pointer_value();
        self.classes.insert(obj.get_type(), name.to_string());

        self.variables.insert(name.to_string(), obj);

        for (m_name, args, body, ret) in methods {
            let constr = self.class_method(name, obj, m_name, *args, **body, *ret);
            self.compile_fn(constr);
        }

        let constr = self.class_method(
            name,
            obj,
            "Init",
            constructor.0,
            *constructor.1,
            obj.get_type().into(),
        );

        let compiled_constr = self.compile_fn(constr);
        compiled_constr.as_global_value().as_pointer_value().into()
    }

    pub(crate) fn class_init(
        &mut self,
        name: &'static str,
        constructor_params: &'ctx [&'ctx TypedNode<'ctx>],
    ) -> BasicValueEnum<'ctx> {
        let name_str = name.to_string();
        let klass = self.variables.get(&name_str).unwrap();

        let constructor = self.get_function(&(name_str + &"%Init")).unwrap().clone();
        let mut compiled_args: Vec<BasicValueEnum> =
            Vec::with_capacity(constructor_params.len() + 1);
        let klas_ = self.builder.build_load(*klass, "base_class_obj");
        let alloca = self.create_entry_block_alloca("class_init", klas_.get_type());
        self.builder.build_store(alloca, klas_);
        compiled_args.push(alloca.into());

        for arg in constructor_params {
            compiled_args.push(self.compile_node(**arg));
        }

        self.builder
            .build_call(constructor, &compiled_args[..], "tmpcall")
            .ok()
            .unwrap()
            .try_as_basic_value()
            .left_or(self.null())
    }

    fn class_method(
        &mut self,
        name: &'static str,
        obj: PointerValue<'ctx>,
        method: &'static str,
        args: &'ctx [(&'static str, AnyTypeEnum<'ctx>)],
        body: TypedNode<'ctx>,
        ret_type: AnyTypeEnum<'ctx>,
    ) -> Function<'ctx> {
        let mut constr_args = vec![("soul".to_string(), obj.get_type().into())];
        constr_args.extend(
            args.iter()
                .map(|x| (x.0.to_string(), x.1.to_basic_type_enum()))
                .collect::<Vec<(String, BasicTypeEnum)>>(),
        );

        Function {
            prototype: Prototype {
                name: Some(name.to_string() + &"%" + &method),
                args: constr_args,
                ret_type,
            },
            body,
        }
    }
}
