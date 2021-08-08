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
use bzxc_shared::{Error, Node};

use crate::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub(crate) fn if_decl(
        &mut self,
        cases: Vec<(Node, Node)>,
        else_case: Option<Node>,
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        let mut blocks = vec![self.builder.get_insert_block().unwrap()];
        let parent = self.fn_value();
        for _ in 1..cases.len() {
            blocks.push(self.context.append_basic_block(parent, "if_start"));
        }

        let else_block = if else_case.is_some() {
            let result = self.context.append_basic_block(parent, "else");
            blocks.push(result);
            Some(result)
        } else {
            None
        };

        let after_block = self.context.append_basic_block(parent, "after");
        blocks.push(after_block);

        for (i, (cond, body)) in cases.iter().enumerate() {
            let then_block = blocks[i];
            let else_block = blocks[i + 1];

            self.builder.position_at_end(then_block);

            let condition = self.compile_node(cond.clone())?;
            let conditional_block = self.context.prepend_basic_block(else_block, "if_body");

            self.builder.build_conditional_branch(
                condition.into_int_value(),
                conditional_block,
                else_block,
            );

            self.builder.position_at_end(conditional_block);
            self.compile_node(body.clone())?;
            if !self.ret {
                self.builder.build_unconditional_branch(after_block);
            };
        }

        if let Some(else_block) = else_block {
            self.builder.position_at_end(else_block);
            self.compile_node(else_case.unwrap())?;
            self.builder.build_unconditional_branch(after_block);
        }

        self.builder.position_at_end(after_block);
        self.ret = false;

        Ok(self.null())
    }
}
