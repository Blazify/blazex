use bzxc_shared::{Tokens, Type, TypedNode};

use crate::TypeSystem;

#[derive(Debug, Clone)]
pub struct Constraint(pub Type, pub Type);

impl TypeSystem {
    pub fn collect(&self, node: TypedNode) -> Vec<Constraint> {
        match node {
            TypedNode::Statements(stmts) => stmts
                .iter()
                .map(|x| self.collect(x.clone()))
                .collect::<Vec<Vec<Constraint>>>()
                .concat(),
            TypedNode::Int { ty, .. } => {
                vec![Constraint(ty, Type::Int)]
            }
            TypedNode::Float { ty, .. } => {
                vec![Constraint(ty, Type::Float)]
            }
            TypedNode::Boolean { ty, .. } => {
                vec![Constraint(ty, Type::Boolean)]
            }
            TypedNode::Char { ty, .. } => vec![Constraint(ty, Type::Char)],
            TypedNode::String { ty, .. } => vec![Constraint(ty, Type::String)],
            TypedNode::Unary { ty, val, .. } => {
                let mut constr = self.collect(*val.clone());
                constr.push(Constraint(ty, val.get_type()));
                constr
            }
            TypedNode::Binary {
                ty,
                left,
                right,
                op_token,
            } => {
                let mut constr = self.collect(*left.clone());
                constr.extend(self.collect(*right.clone()));
                constr.push(Constraint(left.get_type(), right.get_type()));
                constr.push(Constraint(
                    ty.clone(),
                    match op_token.value {
                        Tokens::Plus | Tokens::Minus | Tokens::Multiply | Tokens::Divide => {
                            left.get_type()
                        }
                        _ => Type::Boolean,
                    },
                ));
                constr
            }
            TypedNode::Let { ty, val } => vec![Constraint(ty, val.get_type())],
            TypedNode::Fun { ty, params, body } => {
                let mut constr = self.collect(*body.clone());
                constr.push(Constraint(
                    ty,
                    Type::Fun(
                        params.iter().map(|x| x.ty.clone()).collect(),
                        box body.get_type(),
                    ),
                ));

                constr
            }
            TypedNode::Call { ty, fun, args } => {
                let mut constr = self.collect(*fun.clone());
                for arg in args.clone() {
                    constr.extend(self.collect(arg));
                }
                constr.push(Constraint(
                    fun.get_type(),
                    Type::Fun(args.iter().map(|x| x.get_type()).collect(), box ty),
                ));
                constr
            }
            TypedNode::Return { ty, val } => {
                let mut constr = self.collect(*val.clone());
                constr.push(Constraint(ty, val.get_type()));
                constr
            }
            TypedNode::If {
                ty,
                cases,
                else_case,
            } => {
                let mut constr = vec![];
                for (cond, body) in cases {
                    constr.extend(self.collect(cond.clone()));
                    constr.push(Constraint(Type::Boolean, cond.get_type()));
                    constr.extend(self.collect(body.clone()));
                    constr.push(Constraint(ty.clone(), body.get_type()));
                }

                if let Some(tn) = else_case {
                    constr.push(Constraint(ty.clone(), tn.get_type()));
                    constr.extend(self.collect(*tn));
                }

                return constr;
            }
            TypedNode::For {
                ty,
                start,
                end,
                step,
                body,
                ..
            } => {
                let mut constr = self.collect(*start.clone());
                constr.push(Constraint(ty, body.get_type()));
                constr.push(Constraint(start.get_type(), end.get_type()));
                constr.push(Constraint(start.get_type(), step.get_type()));
                constr.extend(self.collect(*end));
                constr.extend(self.collect(*step));
                constr.extend(self.collect(*body));
                constr
            }
            TypedNode::While { ty, cond, body } => {
                let mut constr = self.collect(*body.clone());
                constr.push(Constraint(ty, body.get_type()));
                constr.extend(self.collect(*cond));
                constr
            }
            _ => vec![],
        }
    }
}
