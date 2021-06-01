use bzxc_shared::{Error, Node};
use inkwell::values::BasicValueEnum;

use crate::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub(crate) fn array_decl(&self, node: Node) -> Result<BasicValueEnum<'ctx>, Error> {
        match node {
            Node::ArrayNode { element_nodes: _ } => {
                Err(self.error(node.get_pos(), "Node can't be compiled"))
            }
            _ => panic!(),
        }
    }

    pub(crate) fn array_access(&self, node: Node) -> Result<BasicValueEnum<'ctx>, Error> {
        match node {
            Node::ArrayAcess { array: _, index: _ } => {
                Err(self.error(node.get_pos(), "Node can't be compiled"))
            }
            _ => panic!(),
        }
    }
}
