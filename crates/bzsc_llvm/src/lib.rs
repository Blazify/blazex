#![allow(dead_code, unused_variables)]
use std::collections::HashMap;

use bzs_shared::{DynType, Error, Node, Position, Tokens};
use inkwell::{
    builder::Builder,
    context::Context,
    module::Module,
    passes::PassManager,
    types::{AnyTypeEnum, BasicType, BasicTypeEnum},
    values::{BasicValue, BasicValueEnum, FunctionValue, PointerValue},
    AddressSpace, FloatPredicate, IntPredicate,
};

#[derive(Debug)]
pub struct Prototype<'ctx> {
    pub name: String,
    pub args: Vec<(String, AnyTypeEnum<'ctx>)>,
    pub ret_type: AnyTypeEnum<'ctx>,
}

#[derive(Debug)]
pub struct Function<'ctx> {
    pub prototype: Prototype<'ctx>,
    pub body: Node,
}

pub struct Compiler<'a, 'ctx> {
    pub context: &'ctx Context,
    pub builder: &'a Builder<'ctx>,
    pub module: &'a Module<'ctx>,
    pub fpm: &'a PassManager<FunctionValue<'ctx>>,
    pub function: &'a Function<'ctx>,

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
                        "anonymous".to_string()
                    } else {
                        name.unwrap().value.into_string()
                    },
                    args: arg_tokens
                        .iter()
                        .map(|x| (x.0.value.into_string(), x.1.to_llvm_type(&self.context)))
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
                    Ok(BasicValueEnum::FloatValue(
                        self.context.f64_type().const_float(i),
                    ))
                } else {
                    Ok(BasicValueEnum::IntValue(
                        self.context
                            .i128_type()
                            .const_int(token.value.into_int() as u64, false),
                    ))
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
                        return Ok(BasicValueEnum::IntValue(
                            self.context
                                .bool_type()
                                .const_int((left_val == right_val) as u64, false),
                        ))
                    }
                    Tokens::NotEquals => {
                        return Ok(BasicValueEnum::IntValue(
                            self.context
                                .bool_type()
                                .const_int((left_val != right_val) as u64, false),
                        ))
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
                    return Ok(BasicValueEnum::IntValue(ret));
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
                    return Ok(BasicValueEnum::FloatValue(ret));
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
                    return Ok(BasicValueEnum::FloatValue(ret));
                }

                if val.is_int_value() {
                    let built = val.into_int_value();
                    let ret = match op_token.typee {
                        Tokens::Plus => built,
                        Tokens::Minus => built.const_neg(),
                        _ => return Err(self.error(node.get_pos(), "Unknown unary operation")),
                    };
                    return Ok(BasicValueEnum::IntValue(ret));
                }

                Err(self.error(node.get_pos(), "Unknown unary operation"))
            }
            Node::StringNode { token } => Ok(BasicValueEnum::PointerValue(
                self.builder.build_pointer_cast(
                    unsafe {
                        self.builder
                            .build_global_string(token.value.into_string().as_str(), "str")
                            .as_pointer_value()
                    },
                    self.context.i8_type().ptr_type(AddressSpace::Generic),
                    "stri8",
                ),
            )),
            Node::CharNode { token } => Ok(BasicValueEnum::IntValue(
                self.context
                    .i8_type()
                    .const_int(token.value.into_char() as u64, false),
            )),
            Node::BooleanNode { token } => Ok(BasicValueEnum::IntValue(
                self.context
                    .bool_type()
                    .const_int(token.value.into_boolean() as u64, false),
            )),
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
                    .ok_or("Undefined variable.");
                match var {
                    Ok(var) => match typee.typee.clone() {
                        Tokens::Equals => {
                            self.builder.build_store(*var, val);
                            Ok(val)
                        }
                        _ => Err(self.error(node.get_pos(), "Unknown compound assignment")),
                    },
                    Err(e) => Err(self.error(node.get_pos(), e)),
                }
            }
            Node::VarAccessNode { token } => {
                match self.variables.get(token.value.into_string().as_str()) {
                    Some(var) => Ok(self
                        .builder
                        .build_load(*var, token.value.into_string().as_str())),
                    None => Err(self.error(node.get_pos(), "Variable not found")),
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

                Ok(BasicValueEnum::IntValue(
                    self.context.i128_type().const_int(0, false),
                ))
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

                let next_var = self.builder.build_int_add(
                    curr_var.into_int_value(),
                    step.into_int_value(),
                    "nextvar",
                );

                self.builder.build_store(start_alloca, next_var);

                let end_condition = self.builder.build_int_compare(
                    IntPredicate::NE,
                    next_var,
                    end_condition.into_int_value(),
                    "loopcond",
                );

                let after_block = self.context.append_basic_block(parent, "afterloop");

                self.builder
                    .build_conditional_branch(end_condition, loop_block, after_block);
                self.builder.position_at_end(after_block);
                self.variables.remove(&var_name_token.value.into_string());

                if let Some(val) = old_val {
                    self.variables
                        .insert(var_name_token.value.into_string(), val);
                }

                Ok(BasicValueEnum::IntValue(
                    self.context.i128_type().const_int(0, false),
                ))
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

                Ok(BasicValueEnum::IntValue(
                    self.context.i128_type().const_int(0, false),
                ))
            }
            Node::FunDef {
                name,
                body_node,
                arg_tokens,
                ..
            } => {
                let func = self.to_func_with_proto(node.clone())?;
                let proto: FunctionValue = self.compile_prototype(&func.prototype)?;

                Ok(BasicValueEnum::PointerValue(
                    proto.as_global_value().as_pointer_value(),
                ))
            }
            Node::CallNode { node_to_call, args } => {
                let mut compiled_args = vec![];
                let mut compiled_args_type = vec![];
                for arg in args {
                    let a = self.compile_node(arg)?;
                    compiled_args.push(a);
                    compiled_args_type.push(a.get_type());
                }
                match *node_to_call {
                    Node::VarAccessNode { token: tok } => {
                        let func = self
                            .get_function(tok.value.into_string().as_str())
                            .ok_or(self.error(node.get_pos(), "Function not found"))?;
                        Ok(self
                            .builder
                            .build_call(func, &compiled_args[..], "tmpcall")
                            .try_as_basic_value()
                            .unwrap_left())
                    }
                    _ => {
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
                }
            }
            Node::ArrayNode { element_nodes } => {
                Err(self.error(node.get_pos(), "Please don't use -l "))
            }
            Node::ArrayAcess { array, index } => {
                Err(self.error(node.get_pos(), "Please don't use -l "))
            }
            Node::ReturnNode { value } => Err(self.error(node.get_pos(), "Please don't use -l ")),
            Node::ObjectDefNode { properties } => {
                Err(self.error(node.get_pos(), "Please don't use -l "))
            }
            Node::ObjectPropAccess { object, property } => {
                Err(self.error(node.get_pos(), "Please don't use -l "))
            }
            Node::ObjectPropEdit {
                object,
                property,
                new_val,
            } => Err(self.error(node.get_pos(), "Please don't use -l ")),
            Node::ClassDefNode {
                name,
                constructor,
                properties,
                methods,
            } => Err(self.error(node.get_pos(), "Please don't use -l ")),
            Node::ClassInitNode {
                name,
                constructor_params,
            } => Err(self.error(node.get_pos(), "Please don't use -l ")),
        }
    }

    fn compile_prototype(&self, proto: &'a Prototype<'ctx>) -> Result<FunctionValue<'ctx>, Error> {
        let ret_type = proto.ret_type;
        let args_types = proto
            .args
            .iter()
            .map(|x| try_any_to_basic(x.1))
            .collect::<Vec<BasicTypeEnum>>();
        let args_types = args_types.as_slice();

        let fn_type = match ret_type {
            AnyTypeEnum::ArrayType(x) => x.fn_type(args_types, false),
            AnyTypeEnum::FloatType(x) => x.fn_type(args_types, false),
            AnyTypeEnum::FunctionType(x) => panic!("functions can't return functions"),
            AnyTypeEnum::IntType(x) => x.fn_type(args_types, false),
            AnyTypeEnum::PointerType(x) => x.fn_type(args_types, false),
            AnyTypeEnum::StructType(x) => x.fn_type(args_types, false),
            AnyTypeEnum::VectorType(x) => x.fn_type(args_types, false),
            AnyTypeEnum::VoidType(x) => x.fn_type(args_types, false),
        };
        let fn_val = self.module.add_function(proto.name.as_str(), fn_type, None);

        for (i, arg) in fn_val.get_param_iter().enumerate() {
            arg.into_int_value().set_name(proto.args[i].0.as_str());
        }

        Ok(fn_val)
    }

    fn compile_fn(&mut self) -> Result<FunctionValue<'ctx>, Error> {
        let proto = &self.function.prototype;
        let function = self.compile_prototype(&proto)?;

        let entry = self.context.append_basic_block(function, "entry");

        self.builder.position_at_end(entry);

        self.fn_value_opt = Some(function);

        self.variables.reserve(proto.args.len());

        for (i, arg) in function.get_param_iter().enumerate() {
            let arg_name = proto.args[i].0.as_str();
            let alloca = self.create_entry_block_alloca(arg_name, arg.get_type());

            self.builder.build_store(alloca, arg);

            self.variables.insert(proto.args[i].0.clone(), alloca);
        }

        self.compile_node(self.function.body.clone())?;

        self.builder
            .build_return(Some(&self.context.i32_type().const_int(0, false)));

        if function.verify(true) {
            self.fpm.run_on(&function);

            Ok(function)
        } else {
            unsafe {
                function.delete();
            }

            Err(self.error(self.function.body.get_pos(), "Unknown operation"))
        }
    }

    pub fn compile(
        context: &'ctx Context,
        builder: &'a Builder<'ctx>,
        module: &'a Module<'ctx>,
        fpm: &'a PassManager<FunctionValue<'ctx>>,
        function: &'a Function<'ctx>,
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

        compiler.compile_fn()
    }
}

fn try_any_to_basic(k: AnyTypeEnum) -> BasicTypeEnum {
    match k {
        AnyTypeEnum::ArrayType(x) => BasicTypeEnum::ArrayType(x),
        AnyTypeEnum::FloatType(x) => BasicTypeEnum::FloatType(x),
        AnyTypeEnum::FunctionType(x) => panic!("Not convertible"),
        AnyTypeEnum::IntType(x) => BasicTypeEnum::IntType(x),
        AnyTypeEnum::PointerType(x) => BasicTypeEnum::PointerType(x),
        AnyTypeEnum::StructType(x) => BasicTypeEnum::StructType(x),
        AnyTypeEnum::VectorType(x) => BasicTypeEnum::VectorType(x),
        AnyTypeEnum::VoidType(x) => panic!("Not convertible"),
    }
}
