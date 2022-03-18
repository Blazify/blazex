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

use bzxc_llvm_wrapper::module::Linkage;
use bzxc_llvm_wrapper::{
    builder::Builder,
    context::Context,
    module::Module,
    passes::PassManager,
    types::BasicType,
    values::{BasicValue, BasicValueEnum, FunctionValue, PointerValue},
    AddressSpace, FloatPredicate, IntPredicate,
};
use bzxc_shared::{LLVMNode, Tokens};

mod oop;

#[derive(Debug, Clone)]
pub struct Compiler<'a, 'ctx> {
    pub context: &'ctx Context,
    pub builder: &'a Builder<'ctx>,
    pub module: &'a Module<'ctx>,
    pub fpm: &'a PassManager<FunctionValue<'ctx>>,
    pub main: LLVMNode<'ctx>,

    fn_value_opt: Option<FunctionValue<'ctx>>,
    variables: HashMap<String, PointerValue<'ctx>>,
    objects: HashMap<(String, u32), usize>,
    classes: HashMap<
        u32,
        (
            String,
            PointerValue<'ctx>,
            HashMap<String, BasicValueEnum<'ctx>>,
        ),
    >,
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
            classes: HashMap::new(),
            ret: false,
        }
    }

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

    fn compile(&mut self, node: LLVMNode<'ctx>) -> BasicValueEnum<'ctx> {
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
                    self.null()
                } else {
                    ret.unwrap()
                };
            }
            LLVMNode::Int { ty, val } => ty.into_int_type().const_int(val, false).into(),
            LLVMNode::Float { ty, val } => ty.into_float_type().const_float(val).into(),
            LLVMNode::Boolean { ty, val } => ty.into_int_type().const_int(val as i128, false).into(),
            LLVMNode::Char { ty, val } => ty.into_int_type().const_int(val as i128, false).into(),
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
                        Tokens::Keyword("not") => val.into_int_value().const_not().into(),
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
                        Tokens::Divide => self.builder.build_int_unsigned_div(lhs, rhs, "tmpdiv"),
                        Tokens::Modulo => self.builder.build_int_unsigned_rem(lhs, rhs, "tmpmod"),
                        Tokens::LessThan => {
                            self.builder.build_int_cast(
                                self.builder.build_int_compare(IntPredicate::ULT, lhs, rhs, "tmpcmp"),
                                self.context.bool_type(),
                                "bool_cast"
                            )
                        }
                        Tokens::GreaterThan => {
                            self.builder.build_int_cast(
                                self.builder.build_int_compare(IntPredicate::UGT, lhs, rhs, "tmpcmp"),
                                self.context.bool_type(),
                                "bool_cast"
                            )
                        }
                        Tokens::LessThanEquals => {
                            self.builder.build_int_cast(
                                self.builder.build_int_compare(IntPredicate::ULE, lhs, rhs, "tmpcmp"),
                                self.context.bool_type(),
                                "bool_cast"
                            )
                        }
                        Tokens::GreaterThanEquals => {
                            self.builder.build_int_cast(
                                self.builder.build_int_compare(IntPredicate::UGE, lhs, rhs, "tmpcmp"),
                                self.context.bool_type(),
                                "bool_cast"
                            )
                        }
                        Tokens::DoubleEquals => {
                            self.builder.build_int_cast(
                                self.builder.build_int_compare(IntPredicate::EQ, lhs, rhs, "tmpcmp"),
                                self.context.bool_type(),
                                "bool_cast"
                            )
                        }
                        Tokens::NotEquals => {
                            self.builder.build_int_cast(
                                self.builder.build_int_compare(IntPredicate::NE, lhs, rhs, "tmpcmp"),
                                self.context.bool_type(),
                                "bool_cast"
                            )

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
                        Tokens::Plus => self.builder.build_float_add(lhs, rhs, "tmpadd").into(),
                        Tokens::Minus => self.builder.build_float_sub(lhs, rhs, "tmpsub").into(),
                        Tokens::Multiply => self.builder.build_float_mul(lhs, rhs, "tmpmul").into(),
                        Tokens::Divide => self.builder.build_float_div(lhs, rhs, "tmpdiv").into(),
                        Tokens::Modulo => self.builder.build_float_rem(lhs, rhs, "tmpmod").into(),
                        Tokens::LessThan => {
                            let cmp = self.builder.build_float_compare(
                                FloatPredicate::ULT,
                                lhs,
                                rhs,
                                "tmpcmp",
                            );

                            self.builder.build_int_cast(
                                cmp,
                                self.context.bool_type(),
                                "bool_cast"
                            ).into()
                        }
                        Tokens::GreaterThan => {
                            let cmp = self.builder.build_float_compare(
                                FloatPredicate::UGT,
                                rhs,
                                lhs,
                                "tmpcmp",
                            );

                            self.builder.build_int_cast(
                                cmp,
                                self.context.bool_type(),
                                "bool_cast"
                            ).into()
                        }
                        Tokens::LessThanEquals => {
                            let cmp = self.builder.build_float_compare(
                                FloatPredicate::ULE,
                                lhs,
                                rhs,
                                "tmpcmp",
                            );

                            self.builder.build_int_cast(
                                cmp,
                                self.context.bool_type(),
                                "bool_cast"
                            ).into()
                        }
                        Tokens::GreaterThanEquals => {
                            let cmp = self.builder.build_float_compare(
                                FloatPredicate::OGE,
                                rhs,
                                lhs,
                                "tmpcmp",
                            );

                            self.builder.build_int_cast(
                                cmp,
                                self.context.bool_type(),
                                "bool_cast"
                            ).into()
                        }
                        Tokens::DoubleEquals => {
                            let cmp = self.builder.build_float_compare(
                                FloatPredicate::OEQ,
                                rhs,
                                lhs,
                                "tmpcmp",
                            );

                            self.builder.build_int_cast(
                                cmp,
                                self.context.bool_type(),
                                "bool_cast"
                            ).into()
                        }
                        Tokens::NotEquals => {
                            let cmp = self.builder.build_float_compare(
                                FloatPredicate::ONE,
                                rhs,
                                lhs,
                                "tmpcmp",
                            );

                            self.builder.build_int_cast(
                                cmp,
                                self.context.bool_type(),
                                "bool_cast"
                            ).into()
                        }
                        _ => unreachable!(),
                    }
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

                let ret = self.ret.clone();
                self.ret = false;
                self.compile(*body);

                self.builder.position_at_end(parental_block.unwrap());
                self.fn_value_opt = parent;

                self.ret = ret;

                if func.verify(true) {
                    self.fpm.run_on(&func);
                } else {
                    eprintln!("{}", self.module.print_to_string().to_string());
                    unsafe {
                        func.delete();
                    }
                    panic!("function {} failed to verify", name);
                }

                let ptr = func.as_global_value().as_pointer_value();
                let alloca = self.create_entry_block_alloca(name.as_str(), ptr.get_type());
                self.builder.build_store(alloca, ptr);
                self.variables.insert(name, alloca);
                ptr.into()
            }
            LLVMNode::Extern {
                ty: _,
                name,
                return_type,
                args,
                var_args,
            } => {
                let ret = self.compile(*return_type).get_type();
                let args = args
                    .iter()
                    .map(|arg| self.compile(arg.clone()).get_type())
                    .collect::<Vec<_>>();

                self.module.add_function(
                    name.as_str(),
                    ret.fn_type(&args[..], var_args),
                    Some(Linkage::External),
                );

                self.null()
            }
            LLVMNode::Let { ty: _, name, val } => {
                let val = self.compile(*val);
                let alloca = self.create_entry_block_alloca(name.as_str(), val.get_type());
                self.builder.build_store(alloca, val);
                self.variables.insert(name.to_string(), alloca);
                self.null()
            }
            LLVMNode::Var { ty: _, name } => {
                match self.variables.get(name.as_str()) {
                    Some(val) => self.builder.build_load(val.clone(), name.as_str()),
                    None => self.module.get_function(name.as_str()).unwrap().as_global_value().as_pointer_value().into(),
                }
            },
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
                let cond_block = self.context.append_basic_block(parent, "while_cond");
                let body_block = self.context.append_basic_block(parent, "while_body");
                let after_block = self.context.append_basic_block(parent, "after");
                self.builder.build_unconditional_branch(cond_block);
                self.builder.position_at_end(cond_block);

                let cond = self.compile(*cond.clone());
                self.builder.build_conditional_branch(
                    cond.into_int_value(),
                    body_block,
                    after_block
                );
                self.builder.position_at_end(body_block);
                self.compile(*body.clone());
                self.builder.build_unconditional_branch(cond_block);

                self.builder.position_at_end(after_block);

                self.ret = false;
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
                let vec_ty = ty.into_vector_type().get_undef();

                for (i, element) in elements.iter().enumerate() {
                    self.builder.build_insert_element(
                        vec_ty,
                        self.compile(element.clone()),
                        self.context.i128_type().const_int(i as i128, true),
                        "vec_push",
                    );
                }

                vec_ty.into()
            }
            LLVMNode::Index { ty: _, array, idx } => {
                let arr = self.compile(*array).into_vector_value();

                self.builder.build_extract_element(
                    arr,
                    self.compile(*idx).into_int_value(),
                    "vec_extract",
                )
            }
            LLVMNode::Object { ty, properties } => self.create_obj(ty, properties),
            LLVMNode::ObjectAccess {
                ty: _,
                object,
                property,
            } => {
                let obj = self.compile(*object).into_pointer_value();
                let ptr = self.obj_property(obj, property);

                self.builder.build_load(ptr, "struct_load")
            }
            LLVMNode::ObjectEdit {
                ty: _,
                object,
                property,
                new_val,
            } => {
                let val = self.compile(*new_val);

                let struct_ty = self.compile(*object).into_pointer_value();
                let ptr = self.obj_property(struct_ty, property);
                self.builder.build_store(ptr, val);

                struct_ty.into()
            }
            LLVMNode::ObjectMethodCall {
                ty: _,
                property,
                object,
                args,
            } => {
                let struct_val = self.compile(*object.clone());

                let ptr = struct_val.into_pointer_value();

                let class = self
                    .classes
                    .get(
                        &ptr.get_type()
                            .get_element_type()
                            .into_struct_type()
                            .get_field_type_at_index(0)
                            .unwrap()
                            .into_vector_type()
                            .get_size(),
                    )
                    .clone();
                let is_class = class.is_some();

                let mut compiled_args: Vec<BasicValueEnum> = Vec::with_capacity(args.len());

                if is_class {
                    compiled_args.push(ptr.into());
                }
                for arg in args {
                    let compiled = self.clone().compile(arg.clone());
                    compiled_args.push(compiled);
                }

                if !is_class {
                    let func = self.obj_property(ptr, property);
                    let call = self
                        .builder
                        .build_call(
                            self.builder
                                .build_load(func, "func_load")
                                .into_pointer_value(),
                            &compiled_args[..],
                            "obj_func_call",
                        )
                        .ok()
                        .unwrap();

                    return call.try_as_basic_value().left_or(self.null());
                }

                let fun = class
                    .clone()
                    .unwrap()
                    .2
                    .get(&property)
                    .unwrap()
                    .clone()
                    .into_pointer_value();

                let call = self
                    .builder
                    .build_call(fun, &compiled_args[..], "tmpcall")
                    .ok()
                    .unwrap();

                call.try_as_basic_value().left_or(self.null())
            }
            LLVMNode::Class {
                ty,
                properties,
                methods,
                constructor,
                name: class,
                static_obj,
            } => {
                let static_obj = self.compile(*static_obj);

                let gb = self.module.add_global(
                    static_obj.get_type(),
                    Some(AddressSpace::Global),
                    "static_obj",
                );
                gb.set_initializer(&static_obj);
                gb.set_constant(false);

                self.variables.insert(class.clone(), gb.as_pointer_value());

                let klass = self.create_obj(ty, properties).into_pointer_value();
                let constructor = self.class_method(class.clone(), klass.get_type(), *constructor);

                let mut n_methods = HashMap::new();

                for (name, method) in methods {
                    n_methods.insert(
                        name,
                        self.class_method(class.clone(), ty.into_pointer_type(), method),
                    );
                }
                self.classes.insert(
                    ty.into_pointer_type()
                        .get_element_type()
                        .into_struct_type()
                        .get_field_type_at_index(0)
                        .unwrap()
                        .into_vector_type()
                        .get_size(),
                    (
                        klass.get_name().to_str().unwrap().to_string(),
                        constructor.into_pointer_value(),
                        n_methods,
                    ),
                );
                self.null()
            }
            LLVMNode::ClassInit {
                ty: _,
                class,
                constructor_params,
            } => {
                let (base, constructor, _) = self
                    .classes
                    .get(
                        &class
                            .into_pointer_type()
                            .get_element_type()
                            .into_struct_type()
                            .get_field_type_at_index(0)
                            .unwrap()
                            .into_vector_type()
                            .get_size(),
                    )
                    .unwrap()
                    .clone();

                let base = self
                    .module
                    .get_global(&*base)
                    .unwrap()
                    .get_initializer()
                    .unwrap();

                let gb =
                    self.module
                        .add_global(base.get_type(), Some(AddressSpace::Global), "soul_obj");
                gb.set_initializer(&base);
                gb.set_constant(false);

                let ptr = gb.as_pointer_value();

                let mut params: Vec<BasicValueEnum<'ctx>> = vec![ptr.into()];
                params.extend(
                    constructor_params
                        .iter()
                        .map(|x| self.compile(x.clone()))
                        .collect::<Vec<BasicValueEnum<'ctx>>>(),
                );
                self.builder
                    .build_call(constructor, &params[..], "klass_init")
                    .ok()
                    .unwrap()
                    .try_as_basic_value()
                    .left_or(self.null());

                ptr.into()
            }
        }
    }
}
