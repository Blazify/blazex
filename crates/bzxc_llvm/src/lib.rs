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

#![allow(unused_variables)]
use std::collections::HashMap;

use bzxc_shared::{DynType, Error, Node, Position, Tokens};
use inkwell::{
    builder::Builder,
    context::Context,
    module::{Linkage, Module},
    passes::PassManager,
    types::{AnyTypeEnum, BasicType, BasicTypeEnum},
    values::{BasicValue, BasicValueEnum, FunctionValue, PointerValue},
    AddressSpace, FloatPredicate, IntPredicate,
};

#[derive(Debug, Clone)]
pub struct Prototype<'ctx> {
    pub name: Option<String>,
    pub args: Vec<(String, BasicTypeEnum<'ctx>)>,
    pub ret_type: AnyTypeEnum<'ctx>,
}

#[derive(Debug, Clone)]
pub struct Function<'ctx> {
    pub prototype: Prototype<'ctx>,
    pub body: Node,
}

pub struct Compiler<'a, 'ctx> {
    pub context: &'ctx Context,
    pub builder: &'a Builder<'ctx>,
    pub module: &'a Module<'ctx>,
    pub fpm: &'a PassManager<FunctionValue<'ctx>>,
    pub function: Function<'ctx>,

    variables: HashMap<String, PointerValue<'ctx>>,
    fn_value_opt: Option<FunctionValue<'ctx>>,
}

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    fn error(&self, pos: (Position, Position), description: &'static str) -> Error {
        Error::new("Compiler Error", pos.0, pos.1, description)
    }

    fn to_func_with_proto(&self, node: Node) -> Result<Function<'ctx>, Error> {
        match node.clone() {
            Node::FunDef {
                arg_tokens,
                body_node,
                name,
                return_type,
            } => Ok(Function {
                prototype: Prototype {
                    name: if name.is_none() {
                        None
                    } else {
                        Some(name.unwrap().value.into_string())
                    },
                    args: arg_tokens
                        .iter()
                        .map(|x| {
                            (
                                x.0.value.into_string(),
                                try_any_to_basic(x.1.to_llvm_type(&self.context)),
                            )
                        })
                        .collect(),
                    ret_type: return_type.to_llvm_type(&self.context),
                },
                body: *body_node,
            }),
            _ => Err(self.error(node.get_pos(), "Not a functions")),
        }
    }

    #[inline]
    fn get_function(&self, name: &str) -> Option<FunctionValue<'ctx>> {
        self.module.get_function(name)
    }

    #[inline]
    fn fn_value(&self) -> FunctionValue<'ctx> {
        self.fn_value_opt.unwrap()
    }

    fn create_entry_block_alloca<T: BasicType<'ctx>>(
        &self,
        name: &str,
        ty: T,
    ) -> PointerValue<'ctx> {
        let builder = self.context.create_builder();

        let entry = self.fn_value().get_first_basic_block().unwrap();

        match entry.get_first_instruction() {
            Some(first_instr) => builder.position_before(&first_instr),
            None => builder.position_at_end(entry),
        }

        builder.build_alloca(ty, name)
    }

    fn compile_node(&mut self, node: Node) -> Result<BasicValueEnum<'ctx>, Error> {
        match node.clone() {
            Node::Statements { statements } => {
                let mut ret = None;
                for statement in statements {
                    ret = Some(self.compile_node(statement)?);
                }

                if ret.is_some() {
                    let val = ret.unwrap();
                    if val.is_int_value() {
                        return Ok(val);
                    }
                }

                return Ok(BasicValueEnum::IntValue(
                    self.context.i128_type().const_int(0, false),
                ));
            }
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
            Node::BinaryNode {
                left,
                op_token,
                right,
            } => {
                let left_val = self.compile_node(*left)?;
                let right_val = self.compile_node(*right)?;

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
                            if op_token.matches(Tokens::Keyword, DynType::String("and".to_string()))
                            {
                                lhs.const_and(rhs)
                            } else if op_token
                                .matches(Tokens::Keyword, DynType::String("or".to_string()))
                            {
                                lhs.const_or(rhs)
                            } else {
                                return Err(self.error(node.get_pos(), "Unknown operation"));
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
                            let cmp = self.builder.build_float_compare(
                                FloatPredicate::ULT,
                                lhs,
                                rhs,
                                "tmpcmp",
                            );

                            self.builder.build_unsigned_int_to_float(
                                cmp,
                                self.context.f64_type(),
                                "tmpbool",
                            )
                        }
                        Tokens::GreaterThan => {
                            let cmp = self.builder.build_float_compare(
                                FloatPredicate::UGT,
                                rhs,
                                lhs,
                                "tmpcmp",
                            );

                            self.builder.build_unsigned_int_to_float(
                                cmp,
                                self.context.f64_type(),
                                "tmpbool",
                            )
                        }
                        Tokens::LessThanEquals => {
                            let cmp = self.builder.build_float_compare(
                                FloatPredicate::ULE,
                                lhs,
                                rhs,
                                "tmpcmp",
                            );

                            self.builder.build_unsigned_int_to_float(
                                cmp,
                                self.context.f64_type(),
                                "tmpbool",
                            )
                        }
                        Tokens::GreaterThanEquals => {
                            let cmp = self.builder.build_float_compare(
                                FloatPredicate::OGE,
                                rhs,
                                lhs,
                                "tmpcmp",
                            );

                            self.builder.build_unsigned_int_to_float(
                                cmp,
                                self.context.f64_type(),
                                "tmpbool",
                            )
                        }
                        _ => return Err(self.error(node.get_pos(), "Unknown operation")),
                    };
                    return Ok(ret.into());
                }

                Err(self.error(node.get_pos(), "Unknown operation"))
            }
            Node::UnaryNode {
                node: child,
                op_token,
            } => {
                let val = self.compile_node(*child)?;

                if val.is_float_value() {
                    let built = val.into_float_value();
                    let ret = match op_token.typee {
                        Tokens::Plus => built,
                        Tokens::Minus => built.const_neg(),
                        _ => return Err(self.error(node.get_pos(), "Unknown unary operation")),
                    };
                    return Ok(ret.into());
                }

                if val.is_int_value() {
                    let built = val.into_int_value();
                    let ret = match op_token.typee {
                        Tokens::Plus => built,
                        Tokens::Minus => built.const_neg(),
                        _ => return Err(self.error(node.get_pos(), "Unknown unary operation")),
                    };
                    return Ok(ret.into());
                }

                Err(self.error(node.get_pos(), "Unknown unary operation"))
            }
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
            Node::CharNode { token } => Ok(self
                .context
                .i8_type()
                .const_int(token.value.into_char() as u64, false)
                .into()),
            Node::BooleanNode { token } => Ok(self
                .context
                .bool_type()
                .const_int(token.value.into_boolean() as u64, false)
                .into()),
            Node::VarAssignNode {
                name,
                value,
                reassignable: _,
            } => {
                let var_name = name.value.into_string();
                let initial_val = self.compile_node(*value)?;
                let alloca =
                    self.create_entry_block_alloca(var_name.as_str(), initial_val.get_type());

                self.builder.build_store(alloca, initial_val);

                self.variables.insert(var_name, alloca);
                Ok(initial_val)
            }
            Node::VarReassignNode { name, value, typee } => {
                let name = name.value.into_string();
                let val = self.compile_node(*value)?;

                let var = self
                    .variables
                    .get(name.as_str())
                    .ok_or(self.error(node.get_pos(), "Variable not found to be reassigned"))?;
                match typee.typee.clone() {
                    Tokens::Equals => {
                        self.builder.build_store(*var, val);
                        Ok(val)
                    }
                    Tokens::PlusEquals => {
                        let curr_var = self.builder.build_load(*var, &name);

                        let new_var: BasicValueEnum = if curr_var.is_int_value()
                            && val.is_int_value()
                        {
                            self.builder
                                .build_int_add(
                                    curr_var.into_int_value(),
                                    val.into_int_value(),
                                    "new_val",
                                )
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
                            return Err(self.error(node.get_pos(), "Unknown compound assignment"));
                        };

                        self.builder.build_store(*var, new_var);
                        Ok(new_var.into())
                    }
                    Tokens::MinusEquals => {
                        let curr_var = self.builder.build_load(*var, &name);

                        let new_var: BasicValueEnum = if curr_var.is_int_value()
                            && val.is_int_value()
                        {
                            self.builder
                                .build_int_sub(
                                    curr_var.into_int_value(),
                                    val.into_int_value(),
                                    "new_val",
                                )
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
                            return Err(self.error(node.get_pos(), "Unknown compound assignment"));
                        };

                        self.builder.build_store(*var, new_var);
                        Ok(new_var)
                    }
                    Tokens::MultiplyEquals => {
                        let curr_var = self.builder.build_load(*var, &name);

                        let new_var: BasicValueEnum = if curr_var.is_int_value()
                            && val.is_int_value()
                        {
                            self.builder
                                .build_int_mul(
                                    curr_var.into_int_value(),
                                    val.into_int_value(),
                                    "new_val",
                                )
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
                            return Err(self.error(node.get_pos(), "Unknown compound assignment"));
                        };

                        self.builder.build_store(*var, new_var);
                        Ok(new_var)
                    }
                    Tokens::DivideEquals => {
                        let curr_var = self.builder.build_load(*var, &name);

                        let new_var: BasicValueEnum = if curr_var.is_int_value()
                            && val.is_int_value()
                        {
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
                            return Err(self.error(node.get_pos(), "Unknown compound assignment"));
                        };

                        self.builder.build_store(*var, new_var);
                        Ok(new_var)
                    }
                    _ => Err(self.error(node.get_pos(), "Unknown compound assignment")),
                }
            }
            Node::VarAccessNode { token } => {
                match self.variables.get(token.value.into_string().as_str()) {
                    Some(var) => Ok(self
                        .builder
                        .build_load(*var, token.value.into_string().as_str())),
                    None => {
                        let func = self.get_function(token.value.into_string().as_str());
                        match func {
                            Some(fun) => Ok(fun.as_global_value().as_pointer_value().into()),
                            None => Err(self.error(node.get_pos(), "Variable not found")),
                        }
                    }
                }
            }
            Node::IfNode { cases, else_case } => {
                let mut blocks = vec![self.builder.get_insert_block().unwrap()];
                let parent = self.fn_value();
                for _ in 1..cases.len() {
                    blocks.push(self.context.append_basic_block(parent, "if_start"));
                }

                let else_block = if else_case.is_some() {
                    let result = self.context.append_basic_block(parent, "else");
                    blocks.push(result);
                    Some(result)
                } else {
                    None
                };

                let after_block = self.context.append_basic_block(parent, "after");
                blocks.push(after_block);

                for (i, (cond, body)) in cases.iter().enumerate() {
                    let then_block = blocks[i];
                    let else_block = blocks[i + 1];

                    self.builder.position_at_end(then_block);

                    let condition = self.compile_node(cond.clone())?;
                    let conditional_block = self.context.prepend_basic_block(else_block, "if_body");

                    self.builder.build_conditional_branch(
                        condition.into_int_value(),
                        conditional_block,
                        else_block,
                    );

                    self.builder.position_at_end(conditional_block);
                    self.compile_node(body.clone())?;
                    self.builder.build_unconditional_branch(after_block);
                }

                if let Some(else_block) = else_block {
                    self.builder.position_at_end(else_block);
                    self.compile_node(else_case.unwrap())?;
                    self.builder.build_unconditional_branch(after_block);
                }

                self.builder.position_at_end(after_block);

                Ok(self.context.i128_type().const_int(0, false).into())
            }
            Node::ForNode {
                var_name_token,
                start_value,
                end_value,
                body_node,
                step_value_node,
            } => {
                let parent = self.fn_value();

                let start = self.compile_node(*start_value)?;
                let start_alloca = self.create_entry_block_alloca(
                    &var_name_token.value.into_string(),
                    start.get_type(),
                );

                self.builder.build_store(start_alloca, start);

                let loop_block = self.context.append_basic_block(parent, "for_loop");

                self.builder.build_unconditional_branch(loop_block);
                self.builder.position_at_end(loop_block);

                let old_val = self.variables.remove(&var_name_token.value.into_string());

                self.variables
                    .insert(var_name_token.value.into_string(), start_alloca);

                self.compile_node(*body_node)?;
                let step = self.compile_node(*step_value_node)?;
                let end_condition = self.compile_node(*end_value)?;

                let curr_var = self
                    .builder
                    .build_load(start_alloca, &var_name_token.value.into_string());

                if !((curr_var.is_int_value()
                    && step.is_int_value()
                    && end_condition.is_int_value())
                    || (curr_var.is_float_value()
                        && step.is_float_value()
                        && end_condition.is_float_value()))
                {
                    return Err(self.error(
                        node.get_pos(),
                        "Expected same type in all start, step and end",
                    ));
                }

                let next_var: BasicValueEnum = if curr_var.is_int_value() {
                    self.builder
                        .build_int_add(curr_var.into_int_value(), step.into_int_value(), "nextvar")
                        .into()
                } else {
                    self.builder
                        .build_float_add(
                            curr_var.into_float_value(),
                            step.into_float_value(),
                            "nextvar",
                        )
                        .into()
                };

                self.builder.build_store(start_alloca, next_var);

                let end_condition = if curr_var.is_int_value() {
                    self.builder.build_int_compare(
                        IntPredicate::NE,
                        next_var.into_int_value(),
                        end_condition.into_int_value(),
                        "loopcond",
                    )
                } else {
                    self.builder.build_float_compare(
                        FloatPredicate::ONE,
                        next_var.into_float_value(),
                        end_condition.into_float_value(),
                        "loopcond",
                    )
                };

                let after_block = self.context.append_basic_block(parent, "afterloop");

                self.builder
                    .build_conditional_branch(end_condition, loop_block, after_block);
                self.builder.position_at_end(after_block);
                self.variables.remove(&var_name_token.value.into_string());

                if let Some(val) = old_val {
                    self.variables
                        .insert(var_name_token.value.into_string(), val);
                }

                Ok(self.context.i128_type().const_int(0, false).into())
            }
            Node::WhileNode {
                condition_node,
                body_node,
            } => {
                let parent = self.fn_value();
                let loop_block = self.context.append_basic_block(parent, "while_loop");

                let after_block = self.context.append_basic_block(parent, "afterloop");

                self.builder.build_conditional_branch(
                    self.compile_node(*condition_node.clone())?.into_int_value(),
                    loop_block,
                    after_block,
                );

                self.builder.position_at_end(loop_block);
                self.compile_node(*body_node)?;
                self.builder.build_conditional_branch(
                    self.compile_node(*condition_node.clone())?.into_int_value(),
                    loop_block,
                    after_block,
                );
                self.builder.position_at_end(after_block);

                Ok(self.context.i128_type().const_int(0, false).into())
            }
            Node::ExternNode {
                name,
                arg_tokens,
                return_type,
                var_args,
            } => {
                let args_types = &arg_tokens
                    .iter()
                    .map(|x| try_any_to_basic(x.to_llvm_type(&self.context)))
                    .collect::<Vec<BasicTypeEnum>>()[..];
                Ok(self
                    .module
                    .add_function(
                        &name.value.into_string(),
                        match return_type.to_llvm_type(&self.context) {
                            AnyTypeEnum::ArrayType(x) => x.fn_type(args_types, var_args),
                            AnyTypeEnum::FloatType(x) => x.fn_type(args_types, var_args),
                            AnyTypeEnum::FunctionType(x) => x
                                .ptr_type(AddressSpace::Generic)
                                .fn_type(args_types, var_args),
                            AnyTypeEnum::IntType(x) => x.fn_type(args_types, var_args),
                            AnyTypeEnum::PointerType(x) => x.fn_type(args_types, var_args),
                            AnyTypeEnum::StructType(x) => x.fn_type(args_types, var_args),
                            AnyTypeEnum::VectorType(x) => x.fn_type(args_types, var_args),
                            AnyTypeEnum::VoidType(x) => x.fn_type(args_types, var_args),
                        },
                        Some(Linkage::External),
                    )
                    .as_global_value()
                    .as_pointer_value()
                    .into())
            }
            Node::FunDef {
                name,
                body_node,
                arg_tokens,
                ..
            } => {
                let func = self.to_func_with_proto(node.clone())?;
                let fun: FunctionValue = self.compile_fn(func)?;

                Ok(fun.as_global_value().as_pointer_value().into())
            }
            Node::CallNode { node_to_call, args } => {
                let mut compiled_args = vec![];

                for arg in args {
                    compiled_args.push(self.compile_node(arg)?);
                }

                let func = self.compile_node(*node_to_call)?;
                if !func.is_pointer_value() {
                    return Err(self.error(
                        node.get_pos(),
                        "Expected a Function pointer found something else",
                    ));
                }

                Ok(self
                    .builder
                    .build_call(func.into_pointer_value(), &compiled_args[..], "tmpcall")
                    .try_as_basic_value()
                    .unwrap_left())
            }
            Node::ArrayNode { element_nodes } => {
                Err(self.error(node.get_pos(), "Node can't be compiled"))
            }
            Node::ArrayAcess { array, index } => {
                Err(self.error(node.get_pos(), "Node can't be compiled"))
            }
            Node::ReturnNode { value } => Err(self.error(node.get_pos(), "Node can't be compiled")),
            Node::ObjectDefNode { properties } => {
                Err(self.error(node.get_pos(), "Node can't be compiled"))
            }
            Node::ObjectPropAccess { object, property } => {
                Err(self.error(node.get_pos(), "Node can't be compiled"))
            }
            Node::ObjectPropEdit {
                object,
                property,
                new_val,
            } => Err(self.error(node.get_pos(), "Node can't be compiled")),
            Node::ClassDefNode {
                name,
                constructor,
                properties,
                methods,
            } => Err(self.error(node.get_pos(), "Node can't be compiled")),
            Node::ClassInitNode {
                name,
                constructor_params,
            } => Err(self.error(node.get_pos(), "Node can't be compiled")),
        }
    }

    fn compile_prototype(&self, proto: &'a Prototype<'ctx>) -> Result<FunctionValue<'ctx>, Error> {
        let ret_type = proto.ret_type;
        let args_types = proto
            .args
            .iter()
            .map(|x| x.1)
            .collect::<Vec<BasicTypeEnum>>();
        let args_types = args_types.as_slice();

        let fn_type = match ret_type {
            AnyTypeEnum::ArrayType(x) => x.fn_type(args_types, false),
            AnyTypeEnum::FloatType(x) => x.fn_type(args_types, false),
            AnyTypeEnum::FunctionType(x) => {
                x.ptr_type(AddressSpace::Generic).fn_type(args_types, false)
            }
            AnyTypeEnum::IntType(x) => x.fn_type(args_types, false),
            AnyTypeEnum::PointerType(x) => x.fn_type(args_types, false),
            AnyTypeEnum::StructType(x) => x.fn_type(args_types, false),
            AnyTypeEnum::VectorType(x) => x.fn_type(args_types, false),
            AnyTypeEnum::VoidType(x) => x.fn_type(args_types, false),
        };
        let fn_val = self.module.add_function(
            proto
                .name
                .as_ref()
                .unwrap_or(&String::from("__anonymous__"))
                .as_str(),
            fn_type,
            None,
        );

        for (i, arg) in fn_val.get_param_iter().enumerate() {
            arg.set_name(proto.args[i].0.as_str());
        }

        Ok(fn_val)
    }

    fn compile_fn(&mut self, func: Function<'ctx>) -> Result<FunctionValue<'ctx>, Error> {
        let parent = self.fn_value_opt.clone();

        let proto = &func.prototype;
        let function = self.compile_prototype(&proto)?;

        let entry = self.context.append_basic_block(function, "entry");

        let main_block = self.builder.get_insert_block();
        self.builder.position_at_end(entry);

        self.fn_value_opt = Some(function);

        self.variables.reserve(proto.args.len());

        for (i, arg) in function.get_param_iter().enumerate() {
            let arg_name = proto.args[i].0.as_str();
            let alloca = self.create_entry_block_alloca(arg_name, arg.get_type());

            self.builder.build_store(alloca, arg);

            self.variables.insert(proto.args[i].0.clone(), alloca);
        }

        let body = self.compile_node(func.body.clone())?;

        self.builder.build_return(Some(&body));

        if main_block.is_some() {
            self.builder.position_at_end(main_block.unwrap());
        }

        self.fn_value_opt = parent;

        if function.verify(true) {
            self.fpm.run_on(&function);

            Ok(function)
        } else {
            unsafe {
                function.delete();
            }

            Err(self.error(func.body.get_pos(), "Invalid generated function"))
        }
    }

    fn compile_top(&mut self) -> Result<FunctionValue<'ctx>, Error> {
        let func = self.function.clone();
        self.compile_fn(func)
    }

    pub fn compile(
        context: &'ctx Context,
        builder: &'a Builder<'ctx>,
        module: &'a Module<'ctx>,
        fpm: &'a PassManager<FunctionValue<'ctx>>,
        function: Function<'ctx>,
    ) -> Result<FunctionValue<'ctx>, Error> {
        let mut compiler = Compiler {
            builder,
            context,
            module,
            fpm,
            variables: HashMap::new(),
            function,
            fn_value_opt: None,
        };

        compiler.compile_top()
    }
}

fn try_any_to_basic(k: AnyTypeEnum) -> BasicTypeEnum {
    match k {
        AnyTypeEnum::ArrayType(x) => x.into(),
        AnyTypeEnum::FloatType(x) => x.into(),
        AnyTypeEnum::FunctionType(x) => x.ptr_type(AddressSpace::Generic).into(),
        AnyTypeEnum::IntType(x) => x.into(),
        AnyTypeEnum::PointerType(x) => x.into(),
        AnyTypeEnum::StructType(x) => x.into(),
        AnyTypeEnum::VectorType(x) => x.into(),
        AnyTypeEnum::VoidType(x) => panic!("void not convertible to basic type"),
    }
}
