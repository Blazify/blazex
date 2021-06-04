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

use bzxc_shared::{DynType, Error, Node, Position, Token, Tokens};
use inkwell::{values::BasicValueEnum, FloatPredicate, IntPredicate};

use crate::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub(crate) fn binary_op(
        &mut self,
        left: Node,
        op_token: Token,
        right: Node,
        pos: (Position, Position),
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        let left_val = self.compile_node(left)?;
        let right_val = self.compile_node(right)?;

        match op_token.typee {
            Tokens::DoubleEquals => {
                return Ok(self
                    .context
                    .bool_type()
                    .const_int((left_val == right_val) as u64, false)
                    .into())
            }
            Tokens::NotEquals => {
                return Ok(self
                    .context
                    .bool_type()
                    .const_int((left_val != right_val) as u64, false)
                    .into())
            }
            _ => (),
        }

        if left_val.is_int_value() && right_val.is_int_value() {
            let lhs = left_val.into_int_value();
            let rhs = right_val.into_int_value();

            let ret = match op_token.typee {
                Tokens::Plus => self.builder.build_int_add(lhs, rhs, "tmpadd"),
                Tokens::Minus => self.builder.build_int_sub(lhs, rhs, "tmpsub"),
                Tokens::Multiply => self.builder.build_int_mul(lhs, rhs, "tmpmul"),
                Tokens::Divide => self.builder.build_int_unsigned_div(lhs, rhs, "tmpdiv"),
                Tokens::LessThan => {
                    self.builder
                        .build_int_compare(IntPredicate::ULT, lhs, rhs, "tmpcmp")
                }
                Tokens::GreaterThan => {
                    self.builder
                        .build_int_compare(IntPredicate::UGT, lhs, rhs, "tmpcmp")
                }
                Tokens::LessThanEquals => {
                    self.builder
                        .build_int_compare(IntPredicate::ULE, lhs, rhs, "tmpcmp")
                }
                Tokens::GreaterThanEquals => {
                    self.builder
                        .build_int_compare(IntPredicate::UGE, lhs, rhs, "tmpcmp")
                }
                _ => {
                    if op_token.matches(Tokens::Keyword, DynType::String("and".to_string())) {
                        lhs.const_and(rhs)
                    } else if op_token.matches(Tokens::Keyword, DynType::String("or".to_string())) {
                        lhs.const_or(rhs)
                    } else {
                        return Err(self.error(pos, "Unknown operation"));
                    }
                }
            };
            return Ok(ret.into());
        }

        if left_val.is_float_value() && right_val.is_float_value() {
            let lhs = left_val.into_float_value();
            let rhs = right_val.into_float_value();

            let ret = match op_token.typee {
                Tokens::Plus => self.builder.build_float_add(lhs, rhs, "tmpadd"),
                Tokens::Minus => self.builder.build_float_sub(lhs, rhs, "tmpsub"),
                Tokens::Multiply => self.builder.build_float_mul(lhs, rhs, "tmpmul"),
                Tokens::Divide => self.builder.build_float_div(lhs, rhs, "tmpdiv"),
                Tokens::LessThan => {
                    let cmp =
                        self.builder
                            .build_float_compare(FloatPredicate::ULT, lhs, rhs, "tmpcmp");

                    self.builder.build_unsigned_int_to_float(
                        cmp,
                        self.context.f64_type(),
                        "tmpbool",
                    )
                }
                Tokens::GreaterThan => {
                    let cmp =
                        self.builder
                            .build_float_compare(FloatPredicate::UGT, rhs, lhs, "tmpcmp");

                    self.builder.build_unsigned_int_to_float(
                        cmp,
                        self.context.f64_type(),
                        "tmpbool",
                    )
                }
                Tokens::LessThanEquals => {
                    let cmp =
                        self.builder
                            .build_float_compare(FloatPredicate::ULE, lhs, rhs, "tmpcmp");

                    self.builder.build_unsigned_int_to_float(
                        cmp,
                        self.context.f64_type(),
                        "tmpbool",
                    )
                }
                Tokens::GreaterThanEquals => {
                    let cmp =
                        self.builder
                            .build_float_compare(FloatPredicate::OGE, rhs, lhs, "tmpcmp");

                    self.builder.build_unsigned_int_to_float(
                        cmp,
                        self.context.f64_type(),
                        "tmpbool",
                    )
                }
                _ => return Err(self.error(pos, "Unknown operation")),
            };
            return Ok(ret.into());
        }

        Err(self.error(pos, "Unknown operation"))
    }

    pub(crate) fn unary_op(
        &mut self,
        child: Node,
        op_token: Token,
        pos: (Position, Position),
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        let val = self.compile_node(child)?;

        if val.is_float_value() {
            let built = val.into_float_value();
            let ret = match op_token.typee {
                Tokens::Plus => built,
                Tokens::Minus => built.const_neg(),
                _ => return Err(self.error(pos, "Unknown unary operation")),
            };
            return Ok(ret.into());
        }

        if val.is_int_value() {
            let built = val.into_int_value();
            let ret = match op_token.typee {
                Tokens::Plus => built,
                Tokens::Minus => built.const_neg(),
                _ => return Err(self.error(pos, "Unknown unary operation")),
            };
            return Ok(ret.into());
        }

        Err(self.error(pos, "Unknown unary operation"))
    }
}
