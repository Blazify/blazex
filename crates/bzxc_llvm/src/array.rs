use bzxc_shared::{Error, Node, Position};
use inkwell::values::BasicValueEnum;

use crate::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub(crate) fn array_decl(
        &self,
        element_nodes: Vec<Node>,
        pos: (Position, Position),
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        Err(self.error(pos, "Node can't be compiled"))
    }

    pub(crate) fn array_access(
        &self,
        array: Node,
        index: Node,
        pos: (Position, Position),
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        Err(self.error(pos, "Node can't be compiled"))
    }
}
