use bzxc_shared::{Error, Node, Position, Token};
use inkwell::values::BasicValueEnum;

use crate::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub(crate) fn class_decl(
        &self,
        constructor: Option<(Vec<Token>, Node)>,
        properties: Vec<(Token, Node)>,
        name: Token,
        methods: Vec<(Token, Vec<Token>, Node)>,
        pos: (Position, Position),
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        Err(self.error(pos, "Node can't be compiled"))
    }

    pub(crate) fn class_init(
        &self,
        name: Token,
        constructor_params: Vec<Node>,
        pos: (Position, Position),
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        Err(self.error(pos, "Node can't be compiled"))
    }
}
