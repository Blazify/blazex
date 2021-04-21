use crate::utils::error::Error;
use crate::utils::symbol::Symbol;
use crate::{
    core::{
        interpreter::{r#type::Type, runtime_result::RuntimeResult},
        parser::nodes::Node,
    },
    utils::{constants::DynType, context::Context},
    Interpret,
};

pub struct Interpreter {}

impl Interpret for Interpreter {
    fn from_ast(node: &Node, ctx: Context) -> Result<String, String> {
        let res = Self::interpret_node(node.clone(), ctx);
        if res.val.is_some() {
            return Ok(format!("{:?}", res.val.unwrap()));
        } else if res.error.is_some() {
            return Ok(res.error.unwrap().prettify());
        }
        Ok("".to_string())
    }
}

/**
   WhileNode
   IfNode
   FunDef
   ForNode
   CharNode
   CallNode
   x VarReassignNode
   x VarAssignNode
   x VarAccessNode
   x UnaryNode
   x StringNode
   x NumberNode
   x BooleanNode
   x BinOpNode
*/
impl Interpreter {
    fn interpret_node(node: Node, mut ctx: Context) -> RuntimeResult {
        let res = RuntimeResult::new();
        match node {
            Node::NumberNode {
                token,
                pos_start,
                pos_end,
            } => {
                if let DynType::Int(_i) = token.value {
                    res.success(Type::Int {
                        val: token.value.into_int(),
                        ctx,
                        pos_start,
                        pos_end,
                    })
                } else {
                    res.success(Type::Float {
                        val: token.value.into_float(),
                        ctx,
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
                ctx,
                pos_start,
                pos_end,
            }),
            Node::CharNode {
                token,
                pos_start,
                pos_end,
            } => res.success(Type::Char {
                val: token.value.into_char(),
                ctx,
                pos_start,
                pos_end,
            }),
            Node::BinOpNode {
                left,
                right,
                op_token,
                ..
            } => {
                let left_built = res
                    .clone()
                    .register(Self::interpret_node(*left.clone(), ctx.clone()));
                if left_built.error.is_some() {
                    return left_built;
                }

                let right_built = res
                    .clone()
                    .register(Self::interpret_node(*right.clone(), ctx.clone()));
                if right_built.error.is_some() {
                    return right_built;
                }

                res.register(
                    left_built
                        .val
                        .unwrap()
                        .op(right_built.val.unwrap(), op_token),
                )
            }
            Node::UnaryNode { node, op_token, .. } => {
                let built = res
                    .clone()
                    .register(Self::interpret_node(*node.clone(), ctx));
                if built.error.is_some() {
                    return built;
                }

                res.register(built.val.unwrap().unary(op_token.r#type))
            }
            Node::BooleanNode {
                token,
                pos_start,
                pos_end,
            } => res.success(Type::Boolean {
                val: token.value.into_boolean(),
                pos_start,
                pos_end,
                ctx,
            }),
            Node::VarAssignNode {
                name,
                value,
                reassignable,
                pos_start,
                pos_end,
            } => {
                let val = res
                    .clone()
                    .register(Self::interpret_node(*value.clone(), ctx.clone()));
                if val.error.is_some() {
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
                        .set_ctx(ctx),
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

                let val = res
                    .clone()
                    .register(Self::interpret_node(*value.clone(), ctx.clone()));
                if val.error.is_some() {
                    return val;
                }
                ctx.symbol_table.set(
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
            _ => {
                panic!("Not implemented yet")
            }
        }
    }
}
