use crate::{
    core::{
        interpreter::{r#type::Type, runtime_result::RuntimeResult},
        parser::nodes::Node,
        token::Token,
    },
    utils::{
        constants::{DynType, Tokens},
        context::Context,
        error::Error,
        symbol::Symbol,
    },
    LanguageServer,
};
use std::collections::HashMap;

pub struct Interpreter {}

impl LanguageServer for Interpreter {
    fn from_ast(node: &Node, ctx: &mut Context) -> Result<Type, Error> {
        let res = Self::interpret_node(node.clone(), ctx);
        return if res.val.is_some() {
            return if res.clone().should_return() {
                Ok(res.val.unwrap())
            } else {
                Ok(Type::Null)
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
                    res.success(Type::Int {
                        val: token.value.into_int(),
                        ctx: ctx.clone(),
                        pos_start,
                        pos_end,
                    })
                } else {
                    res.success(Type::Float {
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
            } => res.success(Type::String {
                val: token.value.into_string(),
                ctx: ctx.clone(),
                pos_start,
                pos_end,
            }),
            Node::CharNode {
                token,
                pos_start,
                pos_end,
            } => res.success(Type::Char {
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
            } => res.success(Type::Boolean {
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

                ctx.symbol_table = ctx.clone().symbol_table.set(
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
                            "Variable not reassignable",
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
                ctx.symbol_table = ctx.clone().symbol_table.set(
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
                res.success(Type::Null)
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
                        .unwrap_or(Type::Int {
                            val: 1,
                            pos_start,
                            pos_end,
                            ctx: ctx.clone(),
                        });
                } else {
                    step_value = Type::Int {
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
                    ctx.symbol_table = ctx.clone().symbol_table.set(
                        var_name_token.clone().value.into_string(),
                        Symbol::new(
                            Type::Int {
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

                res.success(Type::Null)
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

                res.success(Type::Null)
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

                let val = Type::Function {
                    name: func_name.clone(),
                    body_node: *body_node.clone(),
                    args: arg_tokens,
                    pos_start,
                    pos_end,
                    ctx: ctx.clone(),
                };

                if func_name.clone().value.into_string() != "anonymous" {
                    ctx.symbol_table = ctx.clone().symbol_table.set(
                        func_name.clone().value.into_string(),
                        Symbol::new(val.clone(), true),
                    );
                }

                res.success(val)
            }
            Node::CallNode {
                node_to_call, args, ..
            } => {
                let mut eval_args: Vec<Type> = vec![];
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
                let mut elements: Vec<Type> = vec![];

                for node in element_nodes {
                    let element = Self::interpret_node(node, ctx);
                    if element.clone().should_return() {
                        return element;
                    }
                    elements.push(element.val.unwrap());
                }

                res.success(Type::Array {
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
                    val = Type::Null;
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

                res.success(Type::Object {
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
            _ => {
                panic!("Not implemented yet")
            }
        }
    }
}
