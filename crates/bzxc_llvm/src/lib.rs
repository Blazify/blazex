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
use bzxc_shared::{Error, Node, Position};

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
    objects: HashMap<(PointerType<'ctx>, String), u32>,
    object_aligner: u32,
}

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    fn error(&self, pos: (Position, Position), description: &'static str) -> Error {
        Error::new("Compiler Error", pos.0, pos.1, description)
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

                return Ok(if ret.is_none() {
                    self.context.i128_type().const_int(0, false).into()
                } else {
                    ret.unwrap()
                });
            }
            Node::WhileNode {
                condition_node,
                body_node,
            } => self.while_loop(*condition_node, *body_node),
            Node::VarReassignNode { name, typee, value } => {
                self.var_reassign(name, *value, typee, node.get_pos())
            }
            Node::VarAssignNode { name, value, .. } => self.var_assign(name, *value),
            Node::VarAccessNode { token } => self.var_access(token, node.get_pos()),
            Node::UnaryNode {
                node: child,
                op_token,
            } => self.unary_op(*child, op_token, node.get_pos()),
            Node::StringNode { token } => self.string(token),
            Node::NumberNode { token } => self.num(token),
            Node::IfNode { cases, else_case } => self.if_decl(cases, *else_case),
            Node::FunDef {
                name,
                arg_tokens,
                body_node,
                return_type,
            } => self.fun_decl(arg_tokens, *body_node, name, return_type),
            Node::ForNode {
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
                node.get_pos(),
            ),
            Node::CharNode { token } => self.char(token),
            Node::CallNode { node_to_call, args } => {
                self.fun_call(*node_to_call, args, node.get_pos())
            }
            Node::BooleanNode { token } => self.boolean(token),
            Node::BinaryNode {
                left,
                right,
                op_token,
            } => self.binary_op(*left, op_token, *right, node.get_pos()),
            Node::ArrayNode { element_nodes } => self.array_decl(element_nodes, node.get_pos()),
            Node::ArrayAcess { array, index } => self.array_access(*array, *index, node.get_pos()),
            Node::ReturnNode { value } => self.ret(*value, node.get_pos()),
            Node::ObjectDefNode { properties } => self.obj_decl(properties),
            Node::ObjectPropAccess { object, property } => {
                self.obj_get(*object, property, node.get_pos())
            }
            Node::ObjectPropEdit {
                object,
                property,
                new_val,
            } => self.obj_edit(*object, property, *new_val, node.get_pos()),
            Node::ClassDefNode {
                methods,
                properties,
                constructor,
                name,
            } => self.class_decl(name, constructor, properties, methods, node.get_pos()),
            Node::ClassInitNode {
                name,
                constructor_params,
            } => self.class_init(name, constructor_params, node.get_pos()),
            Node::ExternNode {
                name,
                arg_tokens,
                return_type,
                var_args,
            } => self.fun_extern(name, arg_tokens, return_type, var_args),
        }
    }

    pub fn compile_main(&mut self) -> Result<FunctionValue<'ctx>, Error> {
        let func = self.function.clone();
        self.compile_fn(func)
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
        }
    }
}
