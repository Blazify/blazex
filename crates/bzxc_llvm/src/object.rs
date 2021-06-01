use bzxc_shared::{Error, Node};
use inkwell::values::BasicValueEnum;

use crate::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub(crate) fn obj_decl(&self, node: Node) -> Result<BasicValueEnum<'ctx>, Error> {
        match node {
            Node::ObjectDefNode { properties: _ } => {
                Err(self.error(node.get_pos(), "Node can't be compiled"))
            }
            _ => panic!(),
        }
    }

    pub(crate) fn obj_get(&self, node: Node) -> Result<BasicValueEnum<'ctx>, Error> {
        match node {
            Node::ObjectPropAccess {
                object: _,
                property: _,
            } => Err(self.error(node.get_pos(), "Node can't be compiled")),
            _ => panic!(),
        }
    }

    pub(crate) fn obj_edit(&self, node: Node) -> Result<BasicValueEnum<'ctx>, Error> {
        match node {
            Node::ObjectPropEdit {
                object: _,
                property: _,
                new_val: _,
            } => Err(self.error(node.get_pos(), "Node can't be compiled")),
            _ => panic!(),
        }
    }
}
