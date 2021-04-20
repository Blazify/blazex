use crate::{
    core::{
        interpreter::{r#type::Type, runtime_result::RuntimeResult},
        parser::nodes::Node,
    },
    utils::{constants::DynType, context::Context, symbol_table::SymbolTable},
    Interpret,
};

pub struct Interpreter {}

impl Interpret for Interpreter {
    fn from_ast(node: &Node) -> Result<String, String> {
        let global = SymbolTable::new(None);
        let ctx = Context::new("<Main>".to_string(), Some(global), Box::new(None), None);
        let res = Self::interpret_node(node.clone(), ctx);
        if res.val.is_some() {
            return Ok(format!("{:?}", res.val.unwrap()));
        }
        Ok("".to_string())
    }
}

impl Interpreter {
    fn interpret_node(node: Node, ctx: Context) -> RuntimeResult {
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

                res.success(
                    left_built
                        .val
                        .unwrap()
                        .op(right_built.val.unwrap(), op_token.r#type)
                        .val
                        .unwrap(),
                )
            }
            _ => {
                panic!("Not implemented yet")
            }
        }
    }
}
