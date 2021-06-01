use bzxc_shared::{DynType, Error, Node};
use inkwell::{values::BasicValueEnum, AddressSpace};

use crate::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub(crate) fn string(&self, node: Node) -> Result<BasicValueEnum<'ctx>, Error> {
        match node {
            Node::StringNode { token } => Ok(self
                .builder
                .build_pointer_cast(
                    unsafe {
                        self.builder
                            .build_global_string(&token.value.into_string(), "str")
                            .as_pointer_value()
                    },
                    self.context.i8_type().ptr_type(AddressSpace::Generic),
                    "str_i8",
                )
                .into()),
            _ => panic!(),
        }
    }

    pub(crate) fn char(&self, node: Node) -> Result<BasicValueEnum<'ctx>, Error> {
        match node {
            Node::CharNode { token } => Ok(self
                .context
                .i8_type()
                .const_int(token.value.into_char() as u64, false)
                .into()),
            _ => panic!(),
        }
    }

    pub(crate) fn boolean(&self, node: Node) -> Result<BasicValueEnum<'ctx>, Error> {
        match node {
            Node::BooleanNode { token } => Ok(self
                .context
                .bool_type()
                .const_int(token.value.into_boolean() as u64, false)
                .into()),
            _ => panic!(),
        }
    }

    pub(crate) fn num(&self, node: Node) -> Result<BasicValueEnum<'ctx>, Error> {
        match node {
            Node::NumberNode { token } => {
                if let DynType::Float(i) = token.value {
                    Ok(self.context.f64_type().const_float(i).into())
                } else {
                    Ok(self
                        .context
                        .i128_type()
                        .const_int(token.value.into_int() as u64, false)
                        .into())
                }
            }
            _ => panic!(),
        }
    }
}
