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
mod array;
mod class;
mod conditional;
mod function;
mod literals;
mod loops;
mod object;
mod operation;
mod variable;

use std::collections::HashMap;

use bzxc_llvm_wrapper::{
    builder::Builder,
    context::Context,
    module::Module,
    passes::PassManager,
    types::{AnyTypeEnum, BasicType, BasicTypeEnum, PointerType},
    values::{BasicValueEnum, FunctionValue, PointerValue},
};
use bzxc_shared::TypedNode;

#[derive(Debug, Clone)]
pub struct Prototype<'ctx> {
    pub name: Option<String>,
    pub args: Vec<(String, BasicTypeEnum<'ctx>)>,
    pub ret_type: AnyTypeEnum<'ctx>,
}

#[derive(Debug, Clone)]
pub struct Function<'ctx> {
    pub prototype: Prototype<'ctx>,
    pub body: TypedNode<'ctx>,
}

pub struct Compiler<'a, 'ctx> {
    pub context: &'ctx Context,
    pub builder: &'a Builder<'ctx>,
    pub module: &'a Module<'ctx>,
    pub fpm: &'a PassManager<FunctionValue<'ctx>>,
    pub function: Function<'ctx>,

    variables: HashMap<String, PointerValue<'ctx>>,
    fn_value_opt: Option<FunctionValue<'ctx>>,
    objects: HashMap<(PointerType<'ctx>, String), u32>,
    object_aligner: u32,
    classes: HashMap<PointerType<'ctx>, String>,
    ret: bool,
}

impl<'a, 'ctx> Compiler<'a, 'ctx> {
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

    fn null(&self) -> BasicValueEnum<'ctx> {
        let null = self.context.struct_type(&[], false).get_undef();
        let ptr = self.create_entry_block_alloca("null", null.get_type());
        self.builder.build_store(ptr, null);

        ptr.into()
    }

    fn compile_node(&mut self, node: TypedNode<'ctx>) -> BasicValueEnum<'ctx> {
        match node.clone() {
            TypedNode::Statements { statements } => {
                let mut ret = None;
                for statement in statements {
                    if self.ret {
                        continue;
                    }
                    ret = Some(self.compile_node(**statement));
                }

                return if ret.is_none() {
                    self.context.i128_type().const_int(0, false).into()
                } else {
                    ret.unwrap()
                };
            }
            TypedNode::While {
                condition_node,
                body_node,
            } => self.while_loop(*condition_node, *body_node),
            TypedNode::VarReassign { name, typee, value } => self.var_reassign(name, *value, typee),
            TypedNode::VarAssign { name, value } => self.var_assign(name, *value),
            TypedNode::VarAccess { token } => self.var_access(token),
            TypedNode::Unary {
                node: child,
                op_token,
            } => self.unary_op(*child, op_token),
            TypedNode::String { token } => self.string(token),
            TypedNode::Int { token } => self.int(token),
            TypedNode::If { cases, else_case } => self.if_decl(cases, else_case),
            TypedNode::Fun {
                name,
                arg_tokens,
                body,
                return_type,
            } => self.fun_decl(name, arg_tokens, *body, return_type),
            TypedNode::For {
                var_name_token,
                start_value,
                end_value,
                body_node,
                step_value_node,
            } => self.for_loop(
                var_name_token,
                *start_value,
                *end_value,
                *body_node,
                *step_value_node,
            ),
            TypedNode::Char { token } => self.char(token),
            TypedNode::Call { node_to_call, args } => self.fun_call(*node_to_call, args),
            TypedNode::Boolean { token } => self.boolean(token),
            TypedNode::Binary {
                left,
                right,
                op_token,
            } => self.binary_op(*left, op_token, *right),
            TypedNode::Array {
                typee,
                element_nodes,
            } => self.array_decl(typee, element_nodes),
            TypedNode::Index { array, index } => self.array_access(*array, *index),
            TypedNode::Return { value } => self.ret(value),
            TypedNode::Object { properties } => self.obj_decl(properties),
            TypedNode::ObjectAccess { object, property } => self.obj_get(*object, property),
            TypedNode::ObjectEdit {
                object,
                property,
                new_val,
            } => self.obj_edit(*object, property, *new_val),
            TypedNode::ObjectCall {
                args,
                object,
                property,
            } => self.obj_method_call(*object, property, args),
            TypedNode::Class {
                methods,
                properties,
                constructor,
                name,
            } => self.class_decl(name, constructor, properties, methods),
            TypedNode::ClassInit {
                name,
                constructor_params,
            } => self.class_init(name, constructor_params),
            TypedNode::Extern {
                name,
                arg_tokens,
                return_type,
                var_args,
            } => self.fun_extern(name, arg_tokens, return_type, var_args),
            TypedNode::Float { token } => self.float(token),
        }
    }

    pub fn compile_main(&mut self) -> FunctionValue<'ctx> {
        self.compile_fn(self.function.clone())
    }

    pub fn init(
        context: &'ctx Context,
        builder: &'a Builder<'ctx>,
        module: &'a Module<'ctx>,
        fpm: &'a PassManager<FunctionValue<'ctx>>,
        function: Function<'ctx>,
    ) -> Compiler<'a, 'ctx> {
        Compiler {
            builder,
            context,
            module,
            fpm,
            variables: HashMap::new(),
            function,
            fn_value_opt: None,
            objects: HashMap::new(),
            object_aligner: 0,
            classes: HashMap::new(),
            ret: false,
        }
    }
}
