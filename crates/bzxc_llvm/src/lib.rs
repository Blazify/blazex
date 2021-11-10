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
use std::collections::HashMap;

use bzxc_llvm_wrapper::{
    builder::Builder,
    context::Context,
    module::Module,
    passes::PassManager,
    types::{BasicType, StructType},
    values::{BasicValue, BasicValueEnum, FunctionValue, PointerValue},
    FloatPredicate, IntPredicate,
};
use bzxc_shared::{LLVMNode, Tokens};

pub struct Compiler<'a, 'ctx> {
    pub context: &'ctx Context,
    pub builder: &'a Builder<'ctx>,
    pub module: &'a Module<'ctx>,
    pub fpm: &'a PassManager<FunctionValue<'ctx>>,
    pub main: LLVMNode<'ctx>,

    fn_value_opt: Option<FunctionValue<'ctx>>,
    variables: HashMap<String, PointerValue<'ctx>>,
    objects: HashMap<(String, StructType<'ctx>), usize>,
    ret: bool,
}

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub fn init(
        context: &'ctx Context,
        builder: &'a Builder<'ctx>,
        module: &'a Module<'ctx>,
        fpm: &'a PassManager<FunctionValue<'ctx>>,
        main: LLVMNode<'ctx>,
    ) -> Self {
        Self {
            builder,
            context,
            fpm,
            module,
            main,
            fn_value_opt: None,
            variables: HashMap::new(),
            objects: HashMap::new(),
            ret: false,
        }
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

    fn null(&self) -> BasicValueEnum<'ctx> {
        let null = self.context.struct_type(&[], false).get_undef();
        let ptr = self.create_entry_block_alloca("null", null.get_type());
        self.builder.build_store(ptr, null);

        ptr.into()
    }

    pub fn compile_main(&mut self) {
        let func =
            self.module
                .add_function("main", self.context.i32_type().fn_type(&[], false), None);

        let entry = self.context.append_basic_block(func, "entry");
        self.builder.position_at_end(entry);

        self.fn_value_opt = Some(func);
        self.compile(self.main.clone());
        self.builder
            .build_return(Some(&self.context.i32_type().const_int(0, true)));

        self.ret = true;

        if func.verify(true) {
            self.fpm.run_on(&func);
        } else {
            eprintln!(
                "Invalid LLVM IR:\n{}",
                self.module.print_to_string().to_string()
            );
            unsafe {
                func.delete();
            }
        }
    }

    pub(crate) fn compile(&mut self, node: LLVMNode<'ctx>) -> BasicValueEnum<'ctx> {
        match node {
            LLVMNode::Statements(stmts) => {
                let mut ret = None;
                for statement in stmts {
                    if self.ret {
                        continue;
                    }
                    ret = Some(self.compile(statement));
                }

                return if ret.is_none() {
                    self.context.i128_type().const_int(0, false).into()
                } else {
                    ret.unwrap()
                };
            }
            LLVMNode::Int { ty, val } => ty.into_int_type().const_int(val, true).into(),
            LLVMNode::Float { ty, val } => ty.into_float_type().const_float(val).into(),
            LLVMNode::Boolean { ty, val } => ty.into_int_type().const_int(val as i128, true).into(),
            LLVMNode::Char { ty, val } => ty.into_int_type().const_int(val as i128, true).into(),
            LLVMNode::String { ty, val } => self
                .builder
                .build_pointer_cast(
                    unsafe {
                        self.builder
                            .build_global_string(val.as_str(), "str")
                            .as_pointer_value()
                    },
                    ty.into_pointer_type(),
                    "str_i8",
                )
                .into(),
            LLVMNode::Unary { ty, val, op_token } => {
                let val = self.compile(*val);
                if ty.is_int_type() {
                    match op_token.value {
                        Tokens::Plus => val,
                        Tokens::Minus => val.into_int_value().const_neg().into(),
                        _ => unreachable!(),
                    }
                } else {
                    match op_token.value {
                        Tokens::Plus => val,
                        Tokens::Minus => val.into_float_value().const_neg().into(),
                        _ => unreachable!(),
                    }
                }
            }
            LLVMNode::Binary {
                ty,
                left,
                right,
                op_token,
            } => {
                if ty.is_int_type() {
                    let lhs = self.compile(*left).into_int_value();
                    let rhs = self.compile(*right).into_int_value();

                    match op_token.value {
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
                                unreachable!();
                            }
                        }
                    }
                    .into()
                } else {
                    let lhs = self.compile(*left).into_float_value();
                    let rhs = self.compile(*right).into_float_value();

                    match op_token.value {
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
                        Tokens::DoubleEquals => {
                            let cmp = self.builder.build_float_compare(
                                FloatPredicate::OEQ,
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
                        Tokens::NotEquals => {
                            let cmp = self.builder.build_float_compare(
                                FloatPredicate::ONE,
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
                        _ => unreachable!(),
                    }
                    .into()
                }
            }
            LLVMNode::Fun {
                ty,
                name,
                params,
                body,
            } => {
                let func = self.module.add_function(
                    name.as_str(),
                    ty.into_pointer_type()
                        .get_element_type()
                        .into_function_type(),
                    None,
                );

                let parent = self.fn_value_opt.clone();

                let parental_block = self.builder.get_insert_block();

                let entry = self.context.append_basic_block(func, "entry");
                self.builder.position_at_end(entry);

                self.fn_value_opt = Some(func);

                self.variables.reserve(params.len());

                for (i, arg) in func.get_param_iter().enumerate() {
                    let arg_name = params.get(i).unwrap().0.as_str().clone();
                    arg.set_name(arg_name);
                    let alloca = self.create_entry_block_alloca(arg_name, arg.get_type());

                    self.builder.build_store(alloca, arg);
                    self.variables.insert(arg_name.to_string(), alloca);
                }

                self.ret = false;
                self.compile(*body);

                self.builder.position_at_end(parental_block.unwrap());
                self.fn_value_opt = parent;

                func.as_global_value().as_pointer_value().into()
            }
            LLVMNode::Let { ty, name, val } => {
                let alloca = self.create_entry_block_alloca(name.as_str(), ty);
                self.builder.build_store(alloca, self.compile(*val));
                self.variables.insert(name, alloca);
                alloca.into()
            }
            LLVMNode::Var { ty: _, name } => self
                .builder
                .build_load(*self.variables.get(&name).unwrap(), name.as_str()),
            LLVMNode::Call { ty: _, fun, args } => self
                .builder
                .build_call(
                    self.compile(*fun).into_pointer_value(),
                    &args
                        .iter()
                        .map(|x| self.compile(x.clone()))
                        .collect::<Vec<BasicValueEnum>>()[..],
                    "build_call",
                )
                .unwrap()
                .try_as_basic_value()
                .left_or(self.null()),
            LLVMNode::Return { ty: _, val } => {
                let rett = self.compile(*val);
                self.builder.build_return(Some(&rett));
                self.ret = true;
                rett
            }
            LLVMNode::Null { ty } => ty.into_struct_type().const_zero().into(),
            LLVMNode::If {
                ty: _,
                cases,
                else_case,
            } => {
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

                    let condition = self.compile(cond.clone());
                    let conditional_block = self.context.prepend_basic_block(else_block, "if_body");

                    self.builder.build_conditional_branch(
                        condition.into_int_value(),
                        conditional_block,
                        else_block,
                    );

                    self.builder.position_at_end(conditional_block);
                    self.compile(body.clone());
                    if !self.ret {
                        self.builder.build_unconditional_branch(after_block);
                    };
                }

                if let Some(else_block) = else_block {
                    self.builder.position_at_end(else_block);
                    self.compile(*else_case.unwrap());
                    self.builder.build_unconditional_branch(after_block);
                }

                self.builder.position_at_end(after_block);
                self.ret = false;

                self.null()
            }
            LLVMNode::While { ty: _, cond, body } => {
                let parent = self.fn_value();
                let loop_block = self.context.append_basic_block(parent, "while_loop");

                let after_block = self.context.append_basic_block(parent, "afterloop");

                self.builder.build_conditional_branch(
                    self.compile(*cond.clone()).into_int_value(),
                    loop_block,
                    after_block,
                );

                self.builder.position_at_end(loop_block);
                self.compile(*body);
                self.builder.build_conditional_branch(
                    self.compile(*cond.clone()).into_int_value(),
                    loop_block,
                    after_block,
                );
                self.builder.position_at_end(after_block);

                self.null()
            }
            LLVMNode::For {
                ty: _,
                var,
                start,
                end,
                step,
                body,
            } => {
                let parent = self.fn_value();

                let start = self.compile(*start);
                let start_alloca = self.create_entry_block_alloca(&var, start.get_type());

                self.builder.build_store(start_alloca, start);

                let loop_block = self.context.append_basic_block(parent, "for_loop");

                self.builder.build_unconditional_branch(loop_block);
                self.builder.position_at_end(loop_block);

                let old_val = self.variables.remove(&var);

                self.variables.insert(var.clone(), start_alloca);

                self.compile(*body);
                let step = self.compile(*step);
                let end_condition = self.compile(*end);

                let curr_var = self.builder.build_load(start_alloca, &var);

                let next_var: BasicValueEnum = self
                    .builder
                    .build_int_add(curr_var.into_int_value(), step.into_int_value(), "nextvar")
                    .into();

                self.builder.build_store(start_alloca, next_var);

                let end_condition = self.builder.build_int_compare(
                    IntPredicate::NE,
                    next_var.into_int_value(),
                    end_condition.into_int_value(),
                    "loopcond",
                );

                let after_block = self.context.append_basic_block(parent, "afterloop");

                self.builder
                    .build_conditional_branch(end_condition, loop_block, after_block);
                self.builder.position_at_end(after_block);
                self.variables.remove(&var);

                if let Some(val) = old_val {
                    self.variables.insert(var, val);
                }

                self.ret = false;

                self.null()
            }
            LLVMNode::Array { ty, elements } => {
                let array_alloca = self.builder.build_alloca(ty, "array_alloca");
                let mut array = self
                    .builder
                    .build_load(array_alloca, "array_load")
                    .into_array_value();

                for (i, k) in elements.iter().enumerate() {
                    let elem = self.compile(k.clone());

                    array = self
                        .builder
                        .build_insert_value(array, elem, i as u32, "load_array")
                        .unwrap()
                        .into_array_value();
                }

                array.into()
            }
            LLVMNode::Index { ty: _, array, idx } => {
                let arr = self.compile(*array);
                let array_alloca = self.builder.build_alloca(arr.get_type(), "arr_alloc");
                self.builder.build_store(array_alloca, arr);

                let array_elem_ptr = unsafe {
                    self.builder.build_gep(
                        array_alloca,
                        &[
                            self.context.i32_type().const_int(0, false),
                            self.compile(*idx).into_int_value(),
                        ],
                        "get_array_elem_ptr",
                    )
                };
                let array_elem = self.builder.build_load(array_elem_ptr, "array_elem");

                array_elem
            }
            LLVMNode::Object { ty, properties } => {
                let ty = ty.into_pointer_type().get_element_type().into_struct_type();
                let mut struct_val = self
                    .builder
                    .build_insert_value(
                        ty.get_undef(),
                        ty.get_field_type_at_index(0).unwrap().const_zero(),
                        0,
                        "%alignment%",
                    )
                    .unwrap()
                    .into_struct_value();
                for (i, (name, val)) in properties.iter().enumerate() {
                    let idx = i + 1;
                    self.objects.insert((name.clone(), ty), idx);
                    struct_val = self
                        .builder
                        .build_insert_value(
                            struct_val,
                            self.compile(val.clone()),
                            idx as u32,
                            name.as_str(),
                        )
                        .unwrap()
                        .into_struct_value();
                }

                let struct_ptr = self
                    .builder
                    .build_alloca(struct_val.get_type(), "struct_alloca");
                self.builder.build_store(struct_ptr, struct_val);
                struct_ptr.into()
            }
            LLVMNode::ObjectAccess {
                ty: _,
                object,
                property,
            } => {
                let struct_ptr = self.compile(*object).into_pointer_value();

                let i = self
                    .objects
                    .get(&(
                        property,
                        struct_ptr.get_type().get_element_type().into_struct_type(),
                    ))
                    .unwrap();

                let ptr = self
                    .builder
                    .build_struct_gep(struct_ptr, *i as u32, "struct_gep")
                    .ok()
                    .unwrap();

                self.builder.build_load(ptr, "struct_load")
            }
        }
    }
}
