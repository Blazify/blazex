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

use bzxc_llvm_wrapper::{values::BasicValueEnum, FloatPredicate, IntPredicate};
use bzxc_shared::{Token, Tokens, TypedNode};

use crate::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub(crate) fn binary_op(
        &mut self,
        left: TypedNode<'ctx>,
        op_token: Token,
        right: TypedNode<'ctx>,
    ) -> BasicValueEnum<'ctx> {
        let left_val = self.compile_node(left);
        let right_val = self.compile_node(right);

        if left_val.is_int_value() && right_val.is_int_value() {
            let lhs = left_val.into_int_value();
            let rhs = right_val.into_int_value();

            let ret = match op_token.value {
                Tokens::Plus => self.builder.build_int_add(lhs, rhs, "tmpadd"),
                Tokens::Minus => self.builder.build_int_sub(lhs, rhs, "tmpsub"),
                Tokens::Multiply => self.builder.build_int_mul(lhs, rhs, "tmpmul"),
                Tokens::Divide => self.builder.build_int_signed_div(lhs, rhs, "tmpdiv"),
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
                Tokens::DoubleEquals => {
                    self.builder
                        .build_int_compare(IntPredicate::EQ, lhs, rhs, "tmpcmp")
                }
                Tokens::NotEquals => {
                    self.builder
                        .build_int_compare(IntPredicate::NE, lhs, rhs, "tmpcmp")
                }
                _ => {
                    if op_token.value == Tokens::Keyword("and") {
                        self.builder.build_and(lhs, rhs, "and")
                    } else if op_token.value == Tokens::Keyword("or") {
                        self.builder.build_or(lhs, rhs, "or")
                    } else {
                        panic!();
                    }
                }
            };
            return ret.into();
        }

        if left_val.is_float_value() && right_val.is_float_value() {
            let lhs = left_val.into_float_value();
            let rhs = right_val.into_float_value();

            let ret = match op_token.value {
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
                Tokens::DoubleEquals => {
                    let cmp =
                        self.builder
                            .build_float_compare(FloatPredicate::OEQ, rhs, lhs, "tmpcmp");

                    self.builder.build_unsigned_int_to_float(
                        cmp,
                        self.context.f64_type(),
                        "tmpbool",
                    )
                }
                Tokens::NotEquals => {
                    let cmp =
                        self.builder
                            .build_float_compare(FloatPredicate::ONE, rhs, lhs, "tmpcmp");

                    self.builder.build_unsigned_int_to_float(
                        cmp,
                        self.context.f64_type(),
                        "tmpbool",
                    )
                }
                _ => panic!(),
            };
            return ret.into();
        }

        panic!()
    }

    pub(crate) fn unary_op(
        &mut self,
        child: TypedNode<'ctx>,
        op_token: Token,
    ) -> BasicValueEnum<'ctx> {
        let val = self.compile_node(child);

        if val.is_float_value() {
            let built = val.into_float_value();
            let ret = match op_token.value {
                Tokens::Plus => built,
                Tokens::Minus => built.const_neg(),
                _ => panic!(),
            };
            return ret.into();
        }

        if val.is_int_value() {
            let built = val.into_int_value();
            let ret = match op_token.value {
                Tokens::Plus => built,
                Tokens::Minus => built.const_neg(),
                _ => panic!(),
            };
            return ret.into();
        }

        panic!()
    }
}
