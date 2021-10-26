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

use bzxc_llvm_wrapper::values::BasicValueEnum;
use bzxc_shared::{Token, Tokens, TypedNode};

use crate::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub(crate) fn var_assign(
        &mut self,
        name: &'static str,
        value: TypedNode<'ctx>,
    ) -> BasicValueEnum<'ctx> {
        let var_name = name.to_string();
        let initial_val = self.compile_node(value);
        let alloca = self.create_entry_block_alloca(var_name.as_str(), initial_val.get_type());

        self.builder.build_store(alloca, initial_val);

        self.variables.insert(var_name, alloca);
        initial_val
    }

    pub(crate) fn var_access(&self, token: &'static str) -> BasicValueEnum<'ctx> {
        match self.variables.get(token) {
            Some(var) => self.builder.build_load(*var, token),
            None => {
                let func = self.get_function(token);
                match func {
                    Some(fun) => fun.as_global_value().as_pointer_value().into(),
                    None => panic!(),
                }
            }
        }
    }

    pub(crate) fn var_reassign(
        &mut self,
        name: &'static str,
        value: TypedNode<'ctx>,
        typee: Token,
    ) -> BasicValueEnum<'ctx> {
        let name = name.to_string();
        let val = self.compile_node(value);

        let value = self.variables.get(name.as_str()).unwrap();

        match typee.value.clone() {
            Tokens::Equals => {
                self.builder.build_store(*value, val);
                val
            }
            Tokens::PlusEquals => {
                let curr_var = self.builder.build_load(*value, &name);

                let new_var: BasicValueEnum = if curr_var.is_int_value() && val.is_int_value() {
                    self.builder
                        .build_int_add(curr_var.into_int_value(), val.into_int_value(), "new_val")
                        .into()
                } else if curr_var.is_float_value() && val.is_float_value() {
                    self.builder
                        .build_float_add(
                            curr_var.into_float_value(),
                            val.into_float_value(),
                            "addtmp",
                        )
                        .into()
                } else {
                    panic!()
                };

                self.builder.build_store(*value, new_var);
                new_var.into()
            }
            Tokens::MinusEquals => {
                let curr_var = self.builder.build_load(*value, &name);

                let new_var: BasicValueEnum = if curr_var.is_int_value() && val.is_int_value() {
                    self.builder
                        .build_int_sub(curr_var.into_int_value(), val.into_int_value(), "new_val")
                        .into()
                } else if curr_var.is_float_value() && val.is_float_value() {
                    self.builder
                        .build_float_sub(
                            curr_var.into_float_value(),
                            val.into_float_value(),
                            "addtmp",
                        )
                        .into()
                } else {
                    panic!()
                };

                self.builder.build_store(*value, new_var);
                new_var
            }
            Tokens::MultiplyEquals => {
                let curr_var = self.builder.build_load(*value, &name);

                let new_var: BasicValueEnum = if curr_var.is_int_value() && val.is_int_value() {
                    self.builder
                        .build_int_mul(curr_var.into_int_value(), val.into_int_value(), "new_val")
                        .into()
                } else if curr_var.is_float_value() && val.is_float_value() {
                    self.builder
                        .build_float_mul(
                            curr_var.into_float_value(),
                            val.into_float_value(),
                            "addtmp",
                        )
                        .into()
                } else {
                    panic!()
                };

                self.builder.build_store(*value, new_var);
                new_var
            }
            Tokens::DivideEquals => {
                let curr_var = self.builder.build_load(*value, &name);

                let new_var: BasicValueEnum = if curr_var.is_int_value() && val.is_int_value() {
                    self.builder
                        .build_int_unsigned_div(
                            curr_var.into_int_value(),
                            val.into_int_value(),
                            "new_val",
                        )
                        .into()
                } else if curr_var.is_float_value() && val.is_float_value() {
                    self.builder
                        .build_float_div(
                            curr_var.into_float_value(),
                            val.into_float_value(),
                            "addtmp",
                        )
                        .into()
                } else {
                    panic!()
                };

                self.builder.build_store(*value, new_var);
                new_var
            }
            _ => panic!(),
        }
    }
}
