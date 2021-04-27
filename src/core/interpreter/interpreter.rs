use crate::{
    core::{
        interpreter::{runtime_result::RuntimeResult, value::Value},
        parser::nodes::Node,
        token::Token,
    },
    utils::{
        constants::{DynType, Tokens},
        context::Context,
        error::Error,
        symbol::Symbol,
        symbol_table::SymbolTable,
    },
    LanguageServer,
};

use std::collections::HashMap;
pub struct Interpreter {}

impl LanguageServer for Interpreter {
    fn from_ast(node: &Node, ctx: &mut Context) -> Result<Value, Error> {
        let res = Self::interpret_node(node.clone(), ctx);
        return if res.val.is_some() {
            return if res.clone().should_return() {
                Ok(res.val.unwrap())
            } else {
                Ok(Value::Null)
            };
        } else {
            Err(res.error.unwrap())
        };
    }
}

impl Interpreter {
    pub fn interpret_node(node: Node, ctx: &mut Context) -> RuntimeResult {
        let mut res = RuntimeResult::new();
        match node {
            Node::NumberNode {
                token,
                pos_start,
                pos_end,
            } => {
                if let DynType::Int(_) = token.value {
                    res.success(Value::Int {
                        val: token.value.into_int(),
                        ctx: ctx.clone(),
                        pos_start,
                        pos_end,
                    })
                } else {
                    res.success(Value::Float {
                        val: token.value.into_float(),
                        ctx: ctx.clone(),
                        pos_start,
                        pos_end,
                    })
                }
            }
            Node::StringNode {
                token,
                pos_start,
                pos_end,
            } => res.success(Value::String {
                val: token.value.into_string(),
                ctx: ctx.clone(),
                pos_start,
                pos_end,
            }),
            Node::CharNode {
                token,
                pos_start,
                pos_end,
            } => res.success(Value::Char {
                val: token.value.into_char(),
                ctx: ctx.clone(),
                pos_start,
                pos_end,
            }),
            Node::BinOpNode {
                left,
                right,
                op_token,
                ..
            } => {
                let left_built = Self::interpret_node(*left.clone(), ctx);
                if left_built.clone().should_return() {
                    return left_built;
                }

                let right_built = Self::interpret_node(*right.clone(), ctx);
                if right_built.clone().should_return() {
                    return right_built;
                }
                left_built
                    .val
                    .unwrap()
                    .op(right_built.val.unwrap(), op_token)
            }
            Node::UnaryNode { node, op_token, .. } => {
                let built = Self::interpret_node(*node.clone(), ctx);
                if built.clone().should_return() {
                    return built;
                }

                built.val.unwrap().unary(op_token.r#type)
            }
            Node::BooleanNode {
                token,
                pos_start,
                pos_end,
            } => res.success(Value::Boolean {
                val: token.value.into_boolean(),
                pos_start,
                pos_end,
                ctx: ctx.clone(),
            }),
            Node::VarAssignNode {
                name,
                value,
                reassignable,
                pos_start,
                pos_end,
            } => {
                let val = Self::interpret_node(*value.clone(), ctx);
                if val.clone().should_return() {
                    return val;
                }

                if ctx
                    .symbol_table
                    .clone()
                    .get(name.clone().value.into_string())
                    .is_some()
                {
                    return res.failure(
                        Error::new(
                            "Runtime Error",
                            pos_start,
                            pos_end,
                            "Variable Declared Twice",
                        )
                        .set_ctx(ctx.clone()),
                    );
                }

                ctx.symbol_table.set(
                    name.value.into_string(),
                    Symbol::new(val.clone().val.unwrap(), reassignable),
                );
                res.success(val.val.unwrap())
            }
            Node::VarReassignNode {
                value,
                name,
                pos_start,
                pos_end,
            } => {
                let result = ctx.symbol_table.get(name.clone().value.into_string());
                if result.clone().is_none() {
                    return res.failure(
                        Error::new(
                            "Runtime Error",
                            pos_start,
                            pos_end,
                            "Variable not found to be reassigned",
                        )
                        .set_ctx(ctx.clone()),
                    );
                }

                if result.clone().unwrap().reassignable == false {
                    return res.failure(
                        Error::new(
                            "Runtime Error",
                            pos_start,
                            pos_end,
                            "Variable not reassignable",
                        )
                        .set_ctx(ctx.clone()),
                    );
                }

                let val = Self::interpret_node(*value.clone(), ctx);
                if val.clone().should_return() {
                    return val;
                }

                ctx.symbol_table.get_and_set(
                    name.value.into_string(),
                    Symbol::new(val.clone().val.unwrap(), true),
                );

                res.success(val.clone().val.unwrap())
            }
            Node::VarAccessNode {
                token,
                pos_start,
                pos_end,
            } => {
                let result = ctx.symbol_table.get(token.clone().value.into_string());

                if result.clone().is_none() {
                    return res.failure(
                        Error::new("Runtime Error", pos_start, pos_end, "Variable not found")
                            .set_ctx(ctx.clone()),
                    );
                }

                res.success(result.unwrap().clone().value)
            }
            Node::IfNode {
                cases, else_case, ..
            } => {
                for (condition, expression) in cases {
                    let condition_val = Self::interpret_node(condition, ctx);
                    if condition_val.clone().should_return() {
                        return condition_val;
                    }

                    if condition_val.clone().val.unwrap().is_true() {
                        let expr_val = Self::interpret_node(expression, ctx);
                        if expr_val.clone().should_return() {
                            return expr_val;
                        }
                    }
                }
                if else_case.is_some() {
                    let else_val = Self::interpret_node(else_case.unwrap(), ctx);
                    if else_val.clone().should_return() {
                        return else_val;
                    }
                }
                res.success(Value::Null)
            }
            Node::ForNode {
                var_name_token,
                start_value,
                body_node,
                step_value_node,
                end_value,
                pos_start,
                pos_end,
            } => {
                let start = Self::interpret_node(*start_value.clone(), ctx);
                if start.clone().should_return() {
                    return start;
                }

                let end = Self::interpret_node(*end_value.clone(), ctx);
                if end.clone().should_return() {
                    return end;
                }

                let step_value;
                if step_value_node.is_some() {
                    step_value = Self::interpret_node(step_value_node.unwrap().clone(), ctx)
                        .val
                        .unwrap_or(Value::Int {
                            val: 1,
                            pos_start,
                            pos_end,
                            ctx: ctx.clone(),
                        });
                } else {
                    step_value = Value::Int {
                        val: 1,
                        pos_start,
                        pos_end,
                        ctx: ctx.clone(),
                    };
                }

                let mut i = start.val.unwrap().get_int();

                let condition;
                if step_value.clone().get_int() >= 0 {
                    condition = i < end.clone().val.unwrap().get_int();
                } else {
                    condition = i > end.clone().val.unwrap().get_int();
                }

                while condition {
                    if i == end.clone().val.unwrap().get_int() {
                        break;
                    }
                    ctx.symbol_table.set(
                        var_name_token.clone().value.into_string(),
                        Symbol::new(
                            Value::Int {
                                val: i,
                                pos_start,
                                pos_end,
                                ctx: ctx.clone(),
                            },
                            true,
                        ),
                    );

                    i += step_value.clone().get_int();
                    let body_eval = Self::interpret_node(*body_node.clone(), ctx);
                    if body_eval.clone().should_return() {
                        return body_eval;
                    }
                }

                res.success(Value::Null)
            }
            Node::WhileNode {
                condition_node,
                body_node,
                ..
            } => {
                loop {
                    let condition = Self::interpret_node(*condition_node.clone(), ctx);
                    if condition.clone().should_return() {
                        return condition;
                    }

                    if !condition.clone().val.unwrap().is_true() {
                        break;
                    }

                    let body_eval = Self::interpret_node(*body_node.clone(), ctx);
                    if body_eval.clone().should_return() {
                        return body_eval;
                    }
                }

                res.success(Value::Null)
            }
            Node::FunDef {
                name,
                body_node,
                arg_tokens,
                pos_start,
                pos_end,
            } => {
                let func_name = name.unwrap_or(Token::new(
                    Tokens::Identifier,
                    pos_start,
                    pos_end,
                    DynType::String("anonymous".to_string()),
                ));

                let val = Value::Function {
                    name: func_name.clone(),
                    body_node: *body_node.clone(),
                    args: arg_tokens,
                    pos_start,
                    pos_end,
                    ctx: ctx.clone(),
                };

                if func_name.clone().value.into_string() != "anonymous" {
                    ctx.symbol_table.set(
                        func_name.clone().value.into_string(),
                        Symbol::new(val.clone(), true),
                    );
                }

                res.success(val)
            }
            Node::CallNode {
                node_to_call, args, ..
            } => {
                let mut eval_args: Vec<Value> = vec![];
                let val_to_call = Self::interpret_node(*node_to_call.clone(), ctx);

                if val_to_call.error.is_some() {
                    return val_to_call;
                }

                for arg in args {
                    let eval_arg = Self::interpret_node(arg, ctx);
                    if eval_arg.error.is_some() {
                        return eval_arg;
                    }

                    eval_args.push(eval_arg.val.unwrap());
                }

                let mut val = val_to_call.val.unwrap().execute(eval_args);
                val.success(val.clone().val.unwrap())
            }
            Node::ArrayNode {
                element_nodes,
                pos_start,
                pos_end,
            } => {
                let mut elements: Vec<Value> = vec![];

                for node in element_nodes {
                    let element = Self::interpret_node(node, ctx);
                    if element.clone().should_return() {
                        return element;
                    }
                    elements.push(element.val.unwrap());
                }

                res.success(Value::Array {
                    elements,
                    pos_start,
                    pos_end,
                    ctx: ctx.clone(),
                })
            }
            Node::ReturnNode { value, .. } => {
                let val;
                if value.is_some() {
                    let eval_val = Self::interpret_node(value.unwrap().clone(), ctx);
                    if eval_val.clone().should_return() {
                        return eval_val;
                    }
                    val = eval_val.val.unwrap();
                } else {
                    val = Value::Null;
                }

                res.success_return(val)
            }
            Node::ObjectDefNode {
                properties,
                pos_start,
                pos_end,
            } => {
                let mut hash_props = HashMap::new();

                for (k, v) in properties {
                    let e_v = Self::interpret_node(v, ctx);
                    if e_v.clone().should_return() {
                        return e_v;
                    }

                    if hash_props.contains_key(&k.clone().value.into_string()) {
                        return res.failure(Error::new(
                            "Runtime Error",
                            pos_start,
                            pos_end,
                            "Key already exits",
                        ));
                    }
                    hash_props.insert(k.value.into_string(), e_v.val.unwrap());
                }

                res.success(Value::Object {
                    properties: hash_props,
                    pos_start,
                    pos_end,
                    ctx: ctx.clone(),
                })
            }
            Node::ObjectPropAccess {
                object, property, ..
            } => {
                let obj = Self::interpret_node(*object.clone(), ctx);
                if obj.clone().should_return() {
                    return obj;
                }

                let prop = obj
                    .val
                    .unwrap()
                    .get_obj_prop_val(property.value.into_string());
                if prop.clone().should_return() {
                    return prop;
                }

                res.success(prop.val.unwrap())
            }
            Node::ClassDefNode {
                name,
                constructor,
                methods,
                properties,
                pos_start,
                pos_end,
            } => {
                let mut constructor_e = None;
                let mut methods_e: HashMap<String, Value> = HashMap::new();
                let mut properties_e: HashMap<String, Value> = HashMap::new();

                let mut ctx_ = Context::new(
                    name.clone().value.into_string(),
                    SymbolTable::new(Some(Box::new(ctx.symbol_table.clone()))),
                    Box::new(Some(ctx.clone())),
                    Some(pos_start.clone()),
                );

                for (k, v) in properties {
                    let e_v = Self::interpret_node(v, &mut ctx_);
                    if e_v.clone().should_return() {
                        return e_v;
                    }

                    if properties_e.contains_key(&k.clone().value.into_string()) {
                        return res.failure(Error::new(
                            "Runtime Error",
                            pos_start,
                            pos_end,
                            "Key already exits",
                        ));
                    }
                    properties_e.insert(k.value.into_string(), e_v.val.unwrap());
                }
                for (k, v) in methods {
                    let e_v = Self::interpret_node(v, &mut ctx_);
                    if e_v.clone().should_return() {
                        return e_v;
                    }
                    if methods_e.contains_key(&k.clone().value.into_string()) {
                        return res.failure(Error::new(
                            "Runtime Error",
                            pos_start,
                            pos_end,
                            "Key already exits",
                        ));
                    }
                    methods_e.insert(k.value.into_string(), e_v.val.unwrap());
                }
                if constructor.is_some() {
                    let cons_ = Self::interpret_node(constructor.unwrap(), &mut ctx_);
                    if cons_.clone().should_return() {
                        return cons_;
                    }

                    constructor_e = cons_.val;
                }

                let class = Value::Class {
                    ctx: ctx_.clone(),
                    pos_start,
                    pos_end,
                    name: name.clone().value.into_string(),
                    constructor: Box::new(constructor_e),
                    methods: methods_e,
                    properties: properties_e,
                };

                ctx.symbol_table.set(
                    name.clone().value.into_string(),
                    Symbol::new(class.clone(), false),
                );

                res.success(class)
            }
            Node::ClassInitNode {
                name,
                constructor_params,
                pos_start,
                pos_end,
            } => {
                let mut constructor_params_e: Vec<Value> = vec![];
                let class = ctx.symbol_table.get(name.value.into_string()).clone();
                if class.clone().is_none() {
                    return res.failure(
                        Error::new("Runtime Error", pos_start, pos_end, "Unknown Class")
                            .set_ctx(ctx.clone()),
                    );
                }

                for param in constructor_params {
                    let param_e = Self::interpret_node(param, &mut ctx.clone());
                    if param_e.clone().should_return() {
                        return param_e;
                    }

                    constructor_params_e.push(param_e.val.unwrap());
                }

                (*class.clone().unwrap())
                    .value
                    .clone()
                    .init_class(constructor_params_e)
            }
        }
    }
}
