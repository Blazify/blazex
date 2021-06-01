use bzxc_shared::{Error, Node};
use inkwell::values::BasicValueEnum;

use crate::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub(crate) fn class_decl(&self, node: Node) -> Result<BasicValueEnum<'ctx>, Error> {
        match node {
            Node::ClassDefNode {
                constructor: _,
                methods: _,
                name: _,
                properties: _,
            } => Err(self.error(node.get_pos(), "Node can't be compiled")),
            _ => panic!(),
        }
    }

    pub(crate) fn class_init(&self, node: Node) -> Result<BasicValueEnum<'ctx>, Error> {
        match node {
            Node::ClassInitNode {
                name: _,
                constructor_params: _,
            } => Err(self.error(node.get_pos(), "Node can't be compiled")),
            _ => panic!(),
        }
    }
}
