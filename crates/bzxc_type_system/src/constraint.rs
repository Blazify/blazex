use std::collections::{BTreeMap, HashMap};

use bzxc_shared::{Tokens, Type, TypedNode};

use crate::TypeSystem;

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
#[derive(Debug, Clone)]
pub struct Constraint(pub Type, pub Type);

impl<'ctx> TypeSystem<'ctx> {
    pub(crate) fn collect(&mut self, node: TypedNode) -> Vec<Constraint> {
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
                        Tokens::Plus
                        | Tokens::Minus
                        | Tokens::Multiply
                        | Tokens::Divide
                        | Tokens::Modulo => left.get_type(),
                        _ => Type::Boolean,
                    },
                ));
                constr
            }
            TypedNode::Let { ty, val, .. } => {
                let mut constr = self.collect(*val.clone());
                constr.push(Constraint(ty, val.get_type()));
                constr
            }
            TypedNode::ReLet { ty, prev, val, .. } => {
                let mut constr = self.collect(*val.clone());
                constr.push(Constraint(prev.clone(), val.get_type()));
                constr.push(Constraint(ty, prev));
                constr
            }
            TypedNode::Fun {
                ty, params, body, ..
            } => {
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
            TypedNode::Extern {
                ty,
                return_type,
                args,
                ..
            } => {
                let mut constr = vec![];

                let mut param_ty = vec![];
                for arg in args {
                    constr.extend(self.collect(arg.clone()));
                    param_ty.push(arg.get_type());
                }

                constr.extend(self.collect(*return_type.clone()));

                constr.push(Constraint(
                    ty,
                    Type::Fun(param_ty, box return_type.get_type().clone()),
                ));

                constr
            }
            TypedNode::Call { ty, fun, args } => {
                let mut constr = self.collect(*fun.clone());
                let mut args_ty = vec![];
                for arg in args.clone() {
                    constr.extend(self.collect(arg.clone()));
                    args_ty.push(arg.get_type());
                }
                constr.push(Constraint(fun.get_type(), Type::Fun(args_ty, box ty)));
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
                constr.extend(self.collect(*cond.clone()));
                constr.push(Constraint(Type::Boolean, cond.get_type()));
                constr
            }
            TypedNode::Array { ty, elements } => {
                let elem_ty = if let Some(elem) = elements.first() {
                    elem.get_type()
                } else {
                    Type::Null
                };
                let mut constr = self.collect(elements.first().unwrap().clone());

                for element in elements.iter().skip(1).collect::<Vec<&TypedNode>>() {
                    constr.push(Constraint(elem_ty.clone(), element.get_type()));
                    constr.extend(self.collect(element.clone()));
                }

                constr.push(Constraint(
                    ty.clone(),
                    Type::Array(box elem_ty.clone(), None),
                ));
                constr
            }
            TypedNode::Index { ty, array, idx } => {
                let mut constr = self.collect(*array.clone());
                constr.extend(self.collect(*idx.clone()));
                constr.push(Constraint(idx.get_type(), Type::Int));
                constr.push(Constraint(array.get_type(), Type::Array(box ty, None)));
                constr
            }
            TypedNode::Object { ty, properties } => {
                let mut constr = vec![];
                let mut tree = BTreeMap::new();
                for (name, node) in properties {
                    constr.extend(self.collect(node.clone()));
                    tree.insert(name.clone(), node.get_type());
                }
                constr.push(Constraint(ty, Type::create_obj(tree)));
                constr
            }
            TypedNode::CObject { ty, object } => {
                let mut constr = self.collect(*object.clone());
                constr.push(Constraint(ty, object.get_type()));
                constr
            }
            TypedNode::CToBzxObject { ty, object } => {
                let mut constr = self.collect(*object.clone());
                constr.push(Constraint(ty, object.get_type()));
                constr
            }
            TypedNode::ObjectAccess {
                ty,
                property,
                object,
            } => {
                let mut constr = self.collect(*object.clone());
                constr.push(Constraint(
                    object.get_type(),
                    Type::Object(BTreeMap::from([(property, ty)])),
                ));
                constr
            }
            TypedNode::ObjectEdit {
                ty,
                property,
                object,
                new_val,
            } => {
                let mut constr = self.collect(*object.clone());
                constr.extend(self.collect(*new_val.clone()));
                constr.push(Constraint(ty.clone(), new_val.get_type()));
                constr.push(Constraint(
                    object.get_type(),
                    Type::Object(BTreeMap::from([(property, new_val.get_type())])),
                ));
                constr
            }
            TypedNode::ObjectMethodCall {
                ty,
                object,
                args,
                property,
            } => {
                let mut constr = self.collect(*object.clone());
                let mut args_ty = vec![];
                for arg in args.clone() {
                    constr.extend(self.collect(arg.clone()));
                    args_ty.push(arg.get_type());
                }
                constr.push(Constraint(
                    object.get_type(),
                    Type::Object(BTreeMap::from([(property, Type::Fun(args_ty, box ty))])),
                ));
                constr
            }
            TypedNode::Class {
                ty,
                constructor,
                methods,
                properties,
                name: _,
                static_obj,
            } => {
                let mut constr = self.collect(*static_obj.clone());
                let mut tree = BTreeMap::new();

                let mut methods_tree = HashMap::new();
                methods_tree.insert("constructor".to_string(), constructor.get_type());

                for (name, val) in properties {
                    tree.insert(name.clone(), val.get_type());
                    constr.extend(self.collect(val.clone()));
                }

                let obj = Type::create_obj(tree);

                constr.push(Constraint(ty, Type::Class(box obj)));

                constr.extend(self.collect(*constructor.clone()));
                for (name, val) in methods {
                    methods_tree.insert(name.clone(), val.get_type());
                    constr.extend(self.collect(val.clone()));
                }
                self.methods.insert(
                    Type::Array(box Type::Int, Some(Type::last_aligner() as u32)),
                    methods_tree,
                );

                constr
            }
            TypedNode::ClassInit {
                ty,
                class,
                constructor_params,
            } => {
                let mut constr = vec![];
                let mut params = vec![];

                for param in constructor_params {
                    constr.extend(self.collect(param.clone()));
                    params.push(param.get_type());
                }

                constr.push(Constraint(Type::Class(box ty.clone()), class));
                constr.push(Constraint(
                    ty.clone(),
                    Type::Object(BTreeMap::from([(
                        "constructor".to_string(),
                        Type::Fun(params, box ty),
                    )])),
                ));

                constr
            }
            _ => vec![],
        }
    }
}
