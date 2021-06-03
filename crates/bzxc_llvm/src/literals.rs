use bzxc_shared::{DynType, Error, Token};
use inkwell::{values::BasicValueEnum, AddressSpace};

use crate::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub(crate) fn string(&self, token: Token) -> Result<BasicValueEnum<'ctx>, Error> {
        Ok(self
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
            .into())
    }

    pub(crate) fn char(&self, token: Token) -> Result<BasicValueEnum<'ctx>, Error> {
        Ok(self
            .context
            .i8_type()
            .const_int(token.value.into_char() as u64, false)
            .into())
    }

    pub(crate) fn boolean(&self, token: Token) -> Result<BasicValueEnum<'ctx>, Error> {
        Ok(self
            .context
            .bool_type()
            .const_int(token.value.into_boolean() as u64, false)
            .into())
    }

    pub(crate) fn num(&self, token: Token) -> Result<BasicValueEnum<'ctx>, Error> {
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
}
