use bzxc_shared::{Error, Node, Position, Token, Tokens};
use inkwell::values::BasicValueEnum;

use crate::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub(crate) fn var_assign(
        &mut self,
        name: Token,
        value: Node,
        reassignable: bool,
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        let var_name = name.value.into_string();
        let initial_val = self.compile_node(value)?;
        let alloca = self.create_entry_block_alloca(var_name.as_str(), initial_val.get_type());

        self.builder.build_store(alloca, initial_val);

        self.variables.insert(var_name, (alloca, reassignable));
        Ok(initial_val)
    }

    pub(crate) fn var_access(
        &self,
        token: Token,
        pos: (Position, Position),
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        match self.variables.get(token.value.into_string().as_str()) {
            Some(var) => Ok(self
                .builder
                .build_load(var.0, token.value.into_string().as_str())),
            None => {
                let func = self.get_function(token.value.into_string().as_str());
                match func {
                    Some(fun) => Ok(fun.as_global_value().as_pointer_value().into()),
                    None => Err(self.error(pos, "Variable not found")),
                }
            }
        }
    }

    pub(crate) fn var_reassign(
        &mut self,
        name: Token,
        value: Node,
        typee: Token,
        pos: (Position, Position),
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        let name = name.value.into_string();
        let val = self.compile_node(value)?;

        let value = self
            .variables
            .get(name.as_str())
            .ok_or(self.error(pos, "Variable not found to be reassigned"))?;

        if !value.1 {
            return Err(self.error(pos, "Variable isn't mutable"));
        }

        let var = &value.0;
        match typee.typee.clone() {
            Tokens::Equals => {
                self.builder.build_store(*var, val);
                Ok(val)
            }
            Tokens::PlusEquals => {
                let curr_var = self.builder.build_load(*var, &name);

                let new_var: BasicValueEnum = if curr_var.is_int_value() && val.is_int_value() {
                    self.builder
                        .build_int_add(curr_var.into_int_value(), val.into_int_value(), "new_val")
                        .into()
                } else if curr_var.is_float_value() && val.is_float_value() {
                    self.builder
                        .build_float_add(
                            curr_var.into_float_value(),
                            val.into_float_value(),
                            "addtmp",
                        )
                        .into()
                } else {
                    return Err(self.error(pos, "Unknown compound assignment"));
                };

                self.builder.build_store(*var, new_var);
                Ok(new_var.into())
            }
            Tokens::MinusEquals => {
                let curr_var = self.builder.build_load(*var, &name);

                let new_var: BasicValueEnum = if curr_var.is_int_value() && val.is_int_value() {
                    self.builder
                        .build_int_sub(curr_var.into_int_value(), val.into_int_value(), "new_val")
                        .into()
                } else if curr_var.is_float_value() && val.is_float_value() {
                    self.builder
                        .build_float_sub(
                            curr_var.into_float_value(),
                            val.into_float_value(),
                            "addtmp",
                        )
                        .into()
                } else {
                    return Err(self.error(pos, "Unknown compound assignment"));
                };

                self.builder.build_store(*var, new_var);
                Ok(new_var)
            }
            Tokens::MultiplyEquals => {
                let curr_var = self.builder.build_load(*var, &name);

                let new_var: BasicValueEnum = if curr_var.is_int_value() && val.is_int_value() {
                    self.builder
                        .build_int_mul(curr_var.into_int_value(), val.into_int_value(), "new_val")
                        .into()
                } else if curr_var.is_float_value() && val.is_float_value() {
                    self.builder
                        .build_float_mul(
                            curr_var.into_float_value(),
                            val.into_float_value(),
                            "addtmp",
                        )
                        .into()
                } else {
                    return Err(self.error(pos, "Unknown compound assignment"));
                };

                self.builder.build_store(*var, new_var);
                Ok(new_var)
            }
            Tokens::DivideEquals => {
                let curr_var = self.builder.build_load(*var, &name);

                let new_var: BasicValueEnum = if curr_var.is_int_value() && val.is_int_value() {
                    self.builder
                        .build_int_unsigned_div(
                            curr_var.into_int_value(),
                            val.into_int_value(),
                            "new_val",
                        )
                        .into()
                } else if curr_var.is_float_value() && val.is_float_value() {
                    self.builder
                        .build_float_div(
                            curr_var.into_float_value(),
                            val.into_float_value(),
                            "addtmp",
                        )
                        .into()
                } else {
                    return Err(self.error(pos, "Unknown compound assignment"));
                };

                self.builder.build_store(*var, new_var);
                Ok(new_var)
            }
            _ => Err(self.error(pos, "Unknown compound assignment")),
        }
    }
}
