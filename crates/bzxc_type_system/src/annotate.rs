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
            Node::BooleanNode { token } => TypedNode::Boolean {
                ty: Type::fresh_var(),
                val: token.value.into_boolean(),
            },
            Node::CharNode { token } => TypedNode::Char {
                ty: Type::fresh_var(),
                val: token.value.into_char(),
            },
            Node::StringNode { token } => TypedNode::String {
                ty: Type::fresh_var(),
                val: token.value.into_string(),
            },
            Node::UnaryNode { node, op_token } => TypedNode::Unary {
                ty: Type::fresh_var(),
                val: box self.annotate(*node, tenv),
                op_token,
            },
            Node::BinaryNode {
                left,
                op_token,
                right,
            } => TypedNode::Binary {
                ty: Type::fresh_var(),
                left: box self.annotate(*left.clone(), tenv),
                right: box self.annotate(*right.clone(), tenv),
                op_token,
            },
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
                    TypedNode::Null { ty: Type::Null }
                },
            },
            Node::IfNode { cases, else_case } => TypedNode::If {
                ty: Type::fresh_var(),
                cases: cases
                    .iter()
                    .map(|x| {
                        (
                            self.annotate(x.0.clone(), tenv),
                            self.annotate(x.1.clone(), tenv),
                        )
                    })
                    .collect(),
                else_case: if let Some(n) = *else_case.clone() {
                    Some(box self.annotate(n, tenv))
                } else {
                    None
                },
            },
            Node::WhileNode {
                condition_node,
                body_node,
            } => TypedNode::While {
                ty: Type::fresh_var(),
                cond: box self.annotate(*condition_node, tenv),
                body: box self.annotate(*body_node, tenv),
            },
            Node::ForNode {
                var_name_token,
                start_value,
                end_value,
                step_value_node,
                body_node,
            } => TypedNode::For {
                ty: Type::fresh_var(),
                var: var_name_token,
                start: {
                    let start = box self.annotate(*start_value, tenv);
                    tenv.set(var_name_token.value.into_string(), start.get_type());
                    start
                },
                end: box self.annotate(*end_value, tenv),
                step: box self.annotate(*step_value_node, tenv),
                body: box self.annotate(*body_node, tenv),
            },
            Node::ArrayNode { element_nodes } => TypedNode::Array {
                ty: Type::fresh_var(),
                elements: element_nodes
                    .iter()
                    .map(|x| self.annotate(x.clone(), tenv))
                    .collect(),
            },
            Node::ArrayAcess { array, index } => TypedNode::Index {
                ty: Type::fresh_var(),
                array: box self.annotate(*array, tenv),
                idx: box self.annotate(*index, tenv),
            },
            _ => panic!("Not implemented node: {:#?}", node),
        }
    }
}
