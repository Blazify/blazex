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

use bzxc_llvm_wrapper::{values::BasicValueEnum, FloatPredicate, IntPredicate};
use bzxc_shared::TypedNode;

use crate::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub(crate) fn for_loop(
        &mut self,
        var_name_token: &'static str,
        start_value: TypedNode<'ctx>,
        end_value: TypedNode<'ctx>,
        body_node: TypedNode<'ctx>,
        step_value_node: TypedNode<'ctx>,
    ) -> BasicValueEnum<'ctx> {
        let parent = self.fn_value();

        let start = self.compile_node(start_value);
        let start_alloca =
            self.create_entry_block_alloca(&var_name_token.to_string(), start.get_type());

        self.builder.build_store(start_alloca, start);

        let loop_block = self.context.append_basic_block(parent, "for_loop");

        self.builder.build_unconditional_branch(loop_block);
        self.builder.position_at_end(loop_block);

        let old_val = self.variables.remove(&var_name_token.to_string());

        self.variables
            .insert(var_name_token.to_string(), start_alloca);

        self.compile_node(body_node);
        let step = self.compile_node(step_value_node);
        let end_condition = self.compile_node(end_value);

        let curr_var = self
            .builder
            .build_load(start_alloca, &var_name_token.to_string());

        let next_var: BasicValueEnum = if curr_var.is_int_value() {
            self.builder
                .build_int_add(curr_var.into_int_value(), step.into_int_value(), "nextvar")
                .into()
        } else {
            self.builder
                .build_float_add(
                    curr_var.into_float_value(),
                    step.into_float_value(),
                    "nextvar",
                )
                .into()
        };

        self.builder.build_store(start_alloca, next_var);

        let end_condition = if curr_var.is_int_value() {
            self.builder.build_int_compare(
                IntPredicate::NE,
                next_var.into_int_value(),
                end_condition.into_int_value(),
                "loopcond",
            )
        } else {
            self.builder.build_float_compare(
                FloatPredicate::ONE,
                next_var.into_float_value(),
                end_condition.into_float_value(),
                "loopcond",
            )
        };

        let after_block = self.context.append_basic_block(parent, "afterloop");

        self.builder
            .build_conditional_branch(end_condition, loop_block, after_block);
        self.builder.position_at_end(after_block);
        self.variables.remove(&var_name_token.to_string());

        if let Some(val) = old_val {
            self.variables.insert(var_name_token.to_string(), val);
        }

        self.null()
    }

    pub(crate) fn while_loop(
        &mut self,
        condition_node: TypedNode<'ctx>,
        body_node: TypedNode<'ctx>,
    ) -> BasicValueEnum<'ctx> {
        let parent = self.fn_value();
        let loop_block = self.context.append_basic_block(parent, "while_loop");

        let after_block = self.context.append_basic_block(parent, "afterloop");

        self.builder.build_conditional_branch(
            self.compile_node(condition_node.clone()).into_int_value(),
            loop_block,
            after_block,
        );

        self.builder.position_at_end(loop_block);
        self.compile_node(body_node);
        self.builder.build_conditional_branch(
            self.compile_node(condition_node.clone()).into_int_value(),
            loop_block,
            after_block,
        );
        self.builder.position_at_end(after_block);

        self.null()
    }
}
