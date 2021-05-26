#![allow(dead_code, unused_variables)]
use std::{collections::HashMap, path::Path};

use bzs_shared::{DynType, Node, Tokens};
use inkwell::{
    builder::Builder,
    context::Context,
    module::Module,
    passes::PassManager,
    targets::{CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine},
    types::BasicTypeEnum,
    values::{BasicValue, BasicValueEnum, FunctionValue, PointerValue},
    FloatPredicate, IntPredicate, OptimizationLevel,
};

#[derive(Debug)]
pub struct Prototype {
    pub name: String,
    pub args: Vec<String>,
}

#[derive(Debug)]
pub struct Function {
    pub prototype: Prototype,
    pub body: Option<Node>,
}

pub struct Compiler<'a, 'ctx> {
    pub context: &'ctx Context,
    pub builder: &'a Builder<'ctx>,
    pub module: &'a Module<'ctx>,
    pub fpm: &'a PassManager<FunctionValue<'ctx>>,
    pub function: &'a Function,

    variables: HashMap<String, PointerValue<'ctx>>,
    fn_value_opt: Option<FunctionValue<'ctx>>,
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

    fn create_entry_block_alloca(&self, name: &str) -> PointerValue<'ctx> {
        let builder = self.context.create_builder();

        let entry = self.fn_value().get_first_basic_block().unwrap();

        match entry.get_first_instruction() {
            Some(first_instr) => builder.position_before(&first_instr),
            None => builder.position_at_end(entry),
        }

        builder.build_alloca(self.context.f64_type(), name)
    }

    fn compile_node(&mut self, node: Node) -> Result<BasicValueEnum<'ctx>, &'static str> {
        match node {
            Node::Statements { statements } => {
                let mut ret = None;
                for statement in statements {
                    ret = Some(self.compile_node(statement)?);
                }
                return ret.ok_or("Empty program");
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
                                .build_int_compare(IntPredicate::ULT, rhs, lhs, "tmpcmp")
                        }
                        _ => return Err("Unknown op"),
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
                                FloatPredicate::ULT,
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
                        _ => return Err("Unknown op"),
                    };
                    return Ok(BasicValueEnum::FloatValue(ret));
                }

                Err(Box::leak(
                    format!(
                        "{:#?} and {:#?} cannot be operated by {:?}",
                        left_val, right_val, op_token.typee
                    )
                    .into_boxed_str()
                    .to_owned(),
                ))
            }
            Node::UnaryNode { node, op_token } => {
                let val = self.compile_node(*node)?;

                if val.is_float_value() {
                    let built = val.into_float_value();
                    let ret = match op_token.typee {
                        Tokens::Plus => built,
                        Tokens::Minus => built.const_neg(),
                        _ => return Err("Unknown unary op"),
                    };
                    return Ok(BasicValueEnum::FloatValue(ret));
                }

                if val.is_int_value() {
                    let built = val.into_int_value();
                    let ret = match op_token.typee {
                        Tokens::Plus => built,
                        Tokens::Minus => built.const_neg(),
                        _ => return Err("Unknown unary op"),
                    };
                    return Ok(BasicValueEnum::IntValue(ret));
                }

                Err(Box::leak(
                    format!("{:#?} cannot be operated by {:?}", val, op_token.typee)
                        .into_boxed_str()
                        .to_owned(),
                ))
            }
            Node::VarAssignNode {
                name,
                value,
                reassignable: _,
            } => {
                let var_name = name.value.into_string();
                let initial_val = self.compile_node(*value)?;
                let alloca = self.create_entry_block_alloca(var_name.as_str());

                self.builder.build_store(alloca, initial_val);

                self.variables.insert(var_name, alloca);
                Ok(initial_val)
            }
            Node::VarReassignNode {
                name,
                value,
                typee: _,
            } => {
                let name = name.value.into_string();
                let val = self.compile_node(*value)?;

                let var = self
                    .variables
                    .get(name.as_str())
                    .ok_or("Undefined variable.")?;
                self.builder.build_store(*var, val);
                Ok(val)
            }
            Node::VarAccessNode { token } => {
                match self.variables.get(token.value.into_string().as_str()) {
                    Some(var) => Ok(self
                        .builder
                        .build_load(*var, token.value.into_string().as_str())),
                    None => Err("Could not find a matching variable."),
                }
            }
            Node::StringNode { token } => Ok(BasicValueEnum::VectorValue(
                self.context
                    .const_string(&token.value.into_string().as_bytes(), false),
            )),
            Node::CharNode { token } => Ok(BasicValueEnum::IntValue(
                self.context
                    .i8_type()
                    .const_int(token.value.into_char() as u64, false),
            )),
            Node::WhileNode {
                condition_node,
                body_node,
            } => Err("Please don't use -l "),

            Node::IfNode { cases, else_case } => Err("Please don't use -l "),
            Node::FunDef {
                name,
                body_node,
                arg_tokens,
            } => Err("Please don't use -l "),
            Node::ForNode {
                var_name_token,
                start_value,
                end_value,
                body_node,
                step_value_node,
            } => Err("Please don't use -l "),
            Node::CallNode { node_to_call, args } => Err("Please don't use -l "),
            Node::BooleanNode { token } => Err("Please don't use -l "),
            Node::ArrayNode { element_nodes } => Err("Please don't use -l "),
            Node::ArrayAcess { array, index } => Err("Please don't use -l "),
            Node::ReturnNode { ref value } => Err("Please don't use -l "),
            Node::ObjectDefNode { properties } => Err("Please don't use -l "),
            Node::ObjectPropAccess { object, property } => Err("Please don't use -l "),
            Node::ObjectPropEdit {
                object,
                property,
                new_val,
            } => Err("Please don't use -l "),
            Node::ClassDefNode {
                name,
                constructor,
                properties,
                methods,
            } => Err("Please don't use -l "),
            Node::ClassInitNode {
                name,
                constructor_params,
            } => Err("Please don't use -l "),
        }
    }

    fn compile_prototype(&self, proto: &Prototype) -> Result<FunctionValue<'ctx>, &'static str> {
        let ret_type = self.context.f64_type();
        let args_types = std::iter::repeat(ret_type)
            .take(proto.args.len())
            .map(|f| f.into())
            .collect::<Vec<BasicTypeEnum>>();
        let args_types = args_types.as_slice();

        let fn_type = self.context.void_type().fn_type(args_types, false);
        let fn_val = self.module.add_function(proto.name.as_str(), fn_type, None);

        for (i, arg) in fn_val.get_param_iter().enumerate() {
            arg.into_float_value().set_name(proto.args[i].as_str());
        }

        Ok(fn_val)
    }

    fn compile_fn(&mut self) -> Result<FunctionValue<'ctx>, &'static str> {
        let proto = &self.function.prototype;
        let function = self.compile_prototype(proto)?;

        if self.function.body.is_none() {
            return Ok(function);
        }

        let entry = self.context.append_basic_block(function, "entry");

        self.builder.position_at_end(entry);

        self.fn_value_opt = Some(function);

        self.variables.reserve(proto.args.len());

        for (i, arg) in function.get_param_iter().enumerate() {
            let arg_name = proto.args[i].as_str();
            let alloca = self.create_entry_block_alloca(arg_name);

            self.builder.build_store(alloca, arg);

            self.variables.insert(proto.args[i].clone(), alloca);
        }

        let body = self.compile_node(self.function.body.as_ref().unwrap().clone())?;

        self.builder.build_return(None);

        if function.verify(true) {
            self.fpm.run_on(&function);

            Ok(function)
        } else {
            unsafe {
                function.delete();
            }

            Err("Invalid generated function.")
        }
    }

    pub fn compile(
        context: &'ctx Context,
        builder: &'a Builder<'ctx>,
        module: &'a Module<'ctx>,
        fpm: &'a PassManager<FunctionValue<'ctx>>,
        function: &Function,
    ) -> Result<FunctionValue<'ctx>, &'static str> {
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

pub fn compile(node: Node, output: String) {
    let context = Context::create();
    let module = context.create_module("Blazescript");
    let builder = context.create_builder();

    let fpm = PassManager::create(&module);

    fpm.add_instruction_combining_pass();
    fpm.add_reassociate_pass();
    fpm.add_gvn_pass();
    fpm.add_cfg_simplification_pass();
    fpm.add_basic_alias_analysis_pass();
    fpm.add_promote_memory_to_register_pass();
    fpm.add_instruction_combining_pass();
    fpm.add_reassociate_pass();

    fpm.initialize();

    let func = Function {
        body: Some(node),
        prototype: Prototype {
            name: String::from("main"),
            args: vec![],
        },
    };

    match Compiler::compile(&context, &builder, &module, &fpm, &func) {
        Ok(function) => {
            println!("Wrote object file to {}", output);
            let path = Path::new(&output);

            Target::initialize_all(&InitializationConfig::default());
            let target = Target::from_name("x86-64").unwrap();
            let target_machine = target
                .create_target_machine(
                    &TargetMachine::get_default_triple(),
                    "x86-64",
                    TargetMachine::get_host_cpu_features().to_string().as_str(),
                    OptimizationLevel::Default,
                    RelocMode::Default,
                    CodeModel::Default,
                )
                .unwrap();

            target_machine
                .write_to_file(&module, FileType::Object, &path)
                .ok();
        }
        Err(err) => {
            println!("Error compiling function: {}", err);
        }
    }
}
