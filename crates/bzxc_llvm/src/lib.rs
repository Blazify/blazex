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
    values::{BasicValue, BasicValueEnum, FunctionValue, PointerValue},
    AddressSpace,
};
use rand::{distributions::Alphanumeric, Rng};

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
                .unwrap_or(
                    &rand::thread_rng()
                        .sample_iter(&Alphanumeric)
                        .take(20)
                        .map(char::from)
                        .collect(),
                )
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

        if let AnyTypeEnum::VoidType(_) = func.prototype.ret_type {
            self.builder.build_return(None);
        } else {
            self.builder.build_return(Some(&body));
        }

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
        AnyTypeEnum::VoidType(_) => panic!("void not convertible to basic type"),
    }
}
