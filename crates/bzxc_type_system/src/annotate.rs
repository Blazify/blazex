use std::collections::BTreeMap;

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
use bzxc_shared::{Binder, Node, Tokens, Type, TypedNode};

use crate::TypeSystem;

impl<'ctx> TypeSystem<'ctx> {
    pub(crate) fn annotate(&mut self, node: Node) -> TypedNode {
        match node.clone() {
            Node::Statements { statements } => TypedNode::Statements(
                statements
                    .iter()
                    .map(|statement| self.annotate(statement.clone()))
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
                val: box self.annotate(*node),
                op_token,
            },
            Node::BinaryNode {
                left,
                op_token,
                right,
            } => TypedNode::Binary {
                ty: Type::fresh_var(),
                left: box self.annotate(*left.clone()),
                right: box self.annotate(*right.clone()),
                op_token,
            },
            Node::VarAccessNode { token } => match self.type_env.get(token.value.into_string()) {
                Some(ty) => TypedNode::Var {
                    ty,
                    name: token.value.into_string(),
                },
                None => {
                    println!("{:?}", self.type_env); panic!("No var found {}", token.value.into_string()); },
            },
            Node::VarAssignNode { name, value, .. } => {
                let val = self.annotate(*value);
                let ty = val.get_type();
                self.type_env.set(name.value.into_string(), ty.clone());
                TypedNode::Let {
                    ty,
                    name: name.value.into_string(),
                    val: box val,
                }
            }
            Node::FunDef {
                arg_tokens,
                body_node,
                name,
            } => {
                let ty = Type::fresh_var();

                let name = if let Some(tok) = name {
                    self.type_env.set(tok.value.into_string(), ty.clone());
                    tok.value.into_string()
                } else {
                    "%anonymous%".to_string()
                };


                self.type_env.push_scope();
                let mut binders = vec![];
                for arg in arg_tokens {
                    let ty = Type::fresh_var();

                    self.type_env.set(arg.value.into_string(), ty.clone());
                    let binder = Binder {
                        ty,
                        name: arg.value.into_string(),
                    };
                    binders.push(binder);
                }

                let fun = TypedNode::Fun {
                    ty,
                    name,
                    params: binders,
                    body: box self.annotate(*body_node),
                };

                self.type_env.pop_scope();

                fun
            }
            Node::CallNode { args, node_to_call } => TypedNode::Call {
                ty: Type::fresh_var(),
                fun: box self.annotate(*node_to_call),
                args: args
                    .iter()
                    .map(|x| self.annotate(x.clone()))
                    .collect(),
            },
            Node::ReturnNode { value } => {
                let val = box if let Some(val) = *value.clone() {
                    self.annotate(val)
                } else {
                    TypedNode::Null { ty: Type::Null }
                };
                TypedNode::Return {
                    ty: val.get_type(),
                    val,
                }
            }
            Node::IfNode { cases, else_case } => TypedNode::If {
                ty: Type::fresh_var(),
                cases: cases
                    .iter()
                    .map(|x| {
                        self.type_env.push_scope();
                        let val = (
                            self.annotate(x.0.clone()),
                            self.annotate(x.1.clone()),
                        );
                        self.type_env.pop_scope();
                        val
                    })
                    .collect(),
                else_case: if let Some(n) = *else_case.clone() {
                    self.type_env.push_scope();
                    let val = box self.annotate(n);
                    self.type_env.pop_scope();
                    Some(val)
                } else {
                    None
                },
            },
            Node::WhileNode {
                condition_node,
                body_node,
            } => {
                self.type_env.push_scope();
                let val = TypedNode::While {
                    ty: Type::fresh_var(),
                    cond: box self.annotate(*condition_node),
                    body: box self.annotate(*body_node),
                };
                self.type_env.pop_scope();
                val
            },
            Node::ForNode {
                var_name_token,
                start_value,
                end_value,
                step_value_node,
                body_node,
            } => {
                self.type_env.push_scope();
                let val = TypedNode::For {
                    ty: Type::fresh_var(),
                    var: var_name_token.value.into_string(),
                    start: {
                        let start = box self.annotate(*start_value);
                        self.type_env.set(var_name_token.value.into_string(), start.get_type());
                        start
                    },
                    end: box self.annotate(*end_value),
                    step: box self.annotate(*step_value_node),
                    body: box self.annotate(*body_node),
                };
                self.type_env.pop_scope();
                val
            },
            Node::ArrayNode { element_nodes } => TypedNode::Array {
                ty: Type::fresh_var(),
                elements: element_nodes
                    .iter()
                    .map(|x| self.annotate(x.clone()))
                    .collect(),
            },
            Node::ArrayAcess { array, index } => TypedNode::Index {
                ty: Type::fresh_var(),
                array: box self.annotate(*array),
                idx: box self.annotate(*index),
            },
            Node::VarReassignNode { name, typee, value } => {
                let name = name.value.into_string();
                let val = box self.annotate(*value);
                let prev = self.type_env.get(name.clone()).unwrap().clone();

                TypedNode::ReLet {
                    ty: val.get_type(),
                    name,
                    val,
                    prev,
                }
            }
            Node::ObjectDefNode { properties } => TypedNode::Object {
                ty: Type::fresh_var(),
                properties: {
                    let mut tree = BTreeMap::new();

                    for (name, node) in properties {
                        tree.insert(name.value.into_string(), self.annotate(node.clone()));
                    }
                    tree.clone()
                },
            },
            Node::ObjectPropAccess { object, property } => TypedNode::ObjectAccess {
                ty: Type::fresh_var(),
                object: box self.annotate(*object),
                property: property.value.into_string(),
            },
            Node::ObjectPropEdit {
                object,
                property,
                new_val,
            } => TypedNode::ObjectEdit {
                ty: Type::fresh_var(),
                object: box self.annotate(*object),
                property: property.value.into_string(),
                new_val: box self.annotate(*new_val),
            },
            Node::ObjectMethodCall {
                object,
                property,
                args,
            } => TypedNode::ObjectMethodCall {
                ty: Type::fresh_var(),
                object: box self.annotate(*object),
                property: property.value.into_string(),
                args: args
                    .iter()
                    .map(|x| self.annotate(x.clone()))
                    .collect(),
            },
            Node::ClassDefNode {
                methods: mthds,
                properties: props,
                constructor,
                name,
                static_members: st_mthds,
            } => {
                let mut properties = BTreeMap::new();
                let mut methods = BTreeMap::new();
                let mut static_members = BTreeMap::new();

                for (name, value) in st_mthds {
                    static_members.insert(name.value.into_string(), self.annotate(value.clone()));
                }

                for (name, node) in props {
                    properties.insert(name.value.into_string(), self.annotate(node.clone()));
                }

                let obj_ty = Type::fresh_var();
                let ty = Type::Class(box obj_ty.clone());



                let static_obj = TypedNode::Object {
                    ty: Type::fresh_var(),
                    properties: static_members,
                };
                self.type_env.set(name.value.into_string(), static_obj.get_type());
                self.type_env.push_scope();
                self.type_env.set("soul".to_string(), obj_ty.clone());

                for (name, args, body) in mthds {
                    methods.insert(
                        name.value.into_string(),
                        self.annotate(Node::FunDef { name: Some(name), body_node: box body, arg_tokens: args })
                    );
                }

                self.class_env.insert(name.value.into_string(), ty.clone());


                let mut params = vec![];
                let mut params_ty = vec![];
                for arg in constructor.0 {
                    let ty = Type::fresh_var();
                    params_ty.push(ty.clone());
                    self.type_env.set(arg.value.into_string(), ty.clone());
                    params.push(Binder {
                        ty,
                        name: arg.value.into_string(),
                    });
                }

                let val = TypedNode::Class {
                    ty,
                    name: name.value.into_string(),
                    properties,
                    constructor: box TypedNode::Fun {
                        ty: Type::Fun(params_ty, box obj_ty),
                        name: "%constructor%".to_string(),
                        params,
                        body: box self.annotate(*constructor.1),
                    },
                    methods,
                    static_obj: box static_obj
                };

                self.type_env.pop_scope();

                val
            }
            Node::ClassInitNode {
                name,
                constructor_params,
            } => TypedNode::ClassInit {
                ty: Type::fresh_var(),
                class: self.class_env.get(&*name.value.into_string()).unwrap().clone(),
                constructor_params: constructor_params
                    .iter()
                    .map(|x| self.annotate(x.clone()))
                    .collect(),
            },
            Node::ExternNode {
                name,
                arg_tokens,
                return_type,
                var_args
            } => {
                self.type_env.push_scope();
                let name = name.value.into_string();
                let mut params = vec![];
                for arg in arg_tokens {
                    params.push(self.annotate(arg.clone()));
                }

                let ret = self.annotate(*return_type);

                let fun = TypedNode::Extern {
                    ty: Type::fresh_var(),
                    name: name.clone(),
                    args: params,
                    return_type: box ret,
                    var_args
                };
                self.type_env.pop_scope();
                self.type_env.set(name, fun.get_type());

                fun
            }
            Node::TypeKeyword { token } => {
                match token.value.into_string().as_str() {
                    "int" => TypedNode::Int {
                        ty: Type::Int,
                        val: 0
                    },
                    "float" => TypedNode::Float {
                        ty: Type::Float,
                        val: 0.0
                    },
                    "bool" => TypedNode::Boolean {
                        ty: Type::Boolean,
                        val: false
                    },
                    "string" => TypedNode::String {
                        ty: Type::String,
                        val: String::new()
                    },
                    "void" => TypedNode::Null {
                        ty: Type::Null,
                    },
                    _ => unreachable!()
                }
            }
        }
    }
}
