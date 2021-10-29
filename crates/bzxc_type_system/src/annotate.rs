use bzxc_shared::{Binder, Node, Tokens, Type, TypedNode};

use crate::{type_env::TypeEnv, TypeSystem};

impl TypeSystem {
    pub fn annotate(&self, node: Node, tenv: &mut TypeEnv) -> TypedNode {
        match node.clone() {
            Node::Statements { statements } => TypedNode::Statements(
                statements
                    .iter()
                    .map(|statement| self.annotate(statement.clone(), tenv))
                    .collect(),
            ),
            Node::NumberNode { token } => match token.value {
                Tokens::Int(i) => TypedNode::Int {
                    ty: Type::fresh_var(),
                    val: i,
                },
                Tokens::Float(f) => TypedNode::Float {
                    ty: Type::fresh_var(),
                    val: f,
                },
                _ => unreachable!(),
            },
            Node::BooleanNode { token } => {
                if let Tokens::Boolean(b) = token.value {
                    TypedNode::Boolean {
                        ty: Type::fresh_var(),
                        val: b,
                    }
                } else {
                    unreachable!()
                }
            }
            Node::BinaryNode {
                left,
                op_token,
                right,
            } => {
                let left = box self.annotate(*left.clone(), tenv);
                let right = box self.annotate(*right.clone(), tenv);
                TypedNode::Binary {
                    ty: Type::fresh_var(),
                    left,
                    right,
                    op_token,
                }
            }
            Node::VarAccessNode { token } => match tenv.get(token.value.into_string()) {
                Some(ty) => TypedNode::Var {
                    ty,
                    name: token.value.into_string(),
                },
                None => panic!("No var found"),
            },
            Node::VarAssignNode { name, value, .. } => {
                let ty = Type::fresh_var();
                tenv.set(name.value.into_string(), ty.clone());
                TypedNode::Let {
                    ty,
                    val: box self.annotate(*value, tenv),
                }
            }
            Node::FunDef {
                arg_tokens,
                body_node,
                name,
            } => {
                let mut binders = vec![];
                for arg in arg_tokens {
                    let ty = Type::fresh_var();

                    tenv.set(arg.value.into_string(), ty.clone());
                    let binder = Binder {
                        ty,
                        name: arg.value.into_string(),
                    };
                    binders.push(binder);
                }

                let ty = Type::fresh_var();

                if let Some(tok) = name {
                    tenv.set(tok.value.into_string(), ty.clone());
                }

                let fun = TypedNode::Fun {
                    ty,
                    params: binders,
                    body: box self.annotate(*body_node, tenv),
                };

                fun
            }
            Node::CallNode { args, node_to_call } => TypedNode::Call {
                ty: Type::fresh_var(),
                fun: box self.annotate(*node_to_call, tenv),
                args: args
                    .iter()
                    .map(|x| self.annotate(x.clone(), tenv))
                    .collect(),
            },
            Node::ReturnNode { value } => TypedNode::Return {
                ty: Type::fresh_var(),
                val: box if let Some(val) = *value.clone() {
                    self.annotate(val, tenv)
                } else {
                    TypedNode::Int {
                        ty: Type::fresh_var(),
                        val: 0,
                    }
                },
            },
            _ => panic!("Not implemented node: {:#?}", node),
        }
    }
}
