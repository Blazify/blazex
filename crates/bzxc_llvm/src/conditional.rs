use bzxc_shared::{Error, Node};
use inkwell::values::BasicValueEnum;

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
            self.builder.build_unconditional_branch(after_block);
        }

        if let Some(else_block) = else_block {
            self.builder.position_at_end(else_block);
            self.compile_node(else_case.unwrap())?;
            self.builder.build_unconditional_branch(after_block);
        }

        self.builder.position_at_end(after_block);

        Ok(self.context.i128_type().const_int(0, false).into())
    }
}
