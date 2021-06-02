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

use bzxc_shared::{Error, Node, Position};
use inkwell::{
    builder::Builder,
    context::Context,
    module::Module,
    passes::PassManager,
    types::{AnyTypeEnum, BasicType, BasicTypeEnum},
    values::{BasicValueEnum, FunctionValue, PointerValue},
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

    variables: HashMap<String, (PointerValue<'ctx>, bool)>,
    fn_value_opt: Option<FunctionValue<'ctx>>,
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
            Node::NumberNode { .. } => self.num(node),
            Node::BooleanNode { .. } => self.boolean(node),
            Node::CharNode { .. } => self.char(node),
            Node::StringNode { .. } => self.string(node),
            Node::BinaryNode { .. } => self.binary_op(node),
            Node::UnaryNode { .. } => self.unary_op(node),
            Node::VarAssignNode { .. } => self.var_assign(node),
            Node::VarReassignNode { .. } => self.var_reassign(node),
            Node::VarAccessNode { .. } => self.var_access(node),
            Node::IfNode { .. } => self.if_decl(node),
            Node::ForNode { .. } => self.for_loop(node),
            Node::WhileNode { .. } => self.while_loop(node),
            Node::ExternNode { .. } => self.fun_extern(node),
            Node::FunDef { .. } => self.fun_decl(node),
            Node::CallNode { .. } => self.fun_call(node),
            Node::ArrayNode { .. } => self.array_decl(node),
            Node::ArrayAcess { .. } => self.array_access(node),
            Node::ReturnNode { .. } => self.ret(node),
            Node::ObjectDefNode { .. } => self.obj_decl(node),
            Node::ObjectPropAccess { .. } => self.obj_get(node),
            Node::ObjectPropEdit { .. } => self.obj_edit(node),
            Node::ClassDefNode { .. } => self.class_decl(node),
            Node::ClassInitNode { .. } => self.class_init(node),
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
