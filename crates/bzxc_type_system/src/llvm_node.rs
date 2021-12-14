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
use bzxc_llvm_wrapper::context::Context;
use bzxc_shared::{LLVMNode, Type, TypedNode};

use crate::substitution::Substitution;

pub struct LLVMNodeGenerator<'ctx> {
    pub context: &'ctx Context,
}

impl<'ctx> LLVMNodeGenerator<'ctx> {
    pub fn gen(&self, subs: Substitution, node: TypedNode) -> LLVMNode<'ctx> {
        let llvm = |ty: Type| ty.llvm(self.context, subs.0.clone());
        match node {
            TypedNode::Statements(stmts) => LLVMNode::Statements(
                stmts
                    .iter()
                    .map(|x| self.gen(subs.clone(), x.clone()))
                    .collect(),
            ),
            TypedNode::Int { ty, val } => LLVMNode::Int { ty: llvm(ty), val },
            TypedNode::Float { ty, val } => LLVMNode::Float { ty: llvm(ty), val },
            TypedNode::Boolean { ty, val } => LLVMNode::Boolean { ty: llvm(ty), val },
            TypedNode::Char { ty, val } => LLVMNode::Char { ty: llvm(ty), val },
            TypedNode::String { ty, val } => LLVMNode::String { ty: llvm(ty), val },
            TypedNode::Unary { ty, val, op_token } => LLVMNode::Unary {
                ty: llvm(ty),
                op_token,
                val: box self.gen(subs, *val),
            },
            TypedNode::Binary {
                ty,
                left,
                right,
                op_token,
            } => LLVMNode::Binary {
                ty: llvm(ty),
                left: box self.gen(subs.clone(), *left),
                right: box self.gen(subs, *right),
                op_token,
            },
            TypedNode::Fun {
                ty,
                name,
                params,
                body,
            } => LLVMNode::Fun {
                body: box self.gen(subs.clone(), *body),
                ty: llvm(ty),
                name,
                params: params
                    .iter()
                    .map(|x| (x.name.clone(), llvm(x.ty.clone())))
                    .collect(),
            },
            TypedNode::Let { ty, name, val } | TypedNode::ReLet { ty, name, val, .. } => {
                LLVMNode::Let {
                    ty: llvm(ty),
                    name,
                    val: box self.gen(subs, *val),
                }
            }
            TypedNode::Var { ty, name } => LLVMNode::Var { ty: llvm(ty), name },
            TypedNode::Call { ty, fun, args } => LLVMNode::Call {
                ty: llvm(ty),
                args: args
                    .iter()
                    .map(|arg| self.gen(subs.clone(), arg.clone()))
                    .collect(),
                fun: box self.gen(subs, *fun),
            },
            TypedNode::Return { ty, val } => LLVMNode::Return {
                ty: llvm(ty),
                val: box self.gen(subs, *val),
            },
            TypedNode::Null { ty } => LLVMNode::Null { ty: llvm(ty) },
            TypedNode::If {
                ty,
                cases,
                else_case,
            } => LLVMNode::If {
                ty: llvm(ty),
                cases: cases
                    .iter()
                    .map(|(cond, body)| {
                        (
                            self.gen(subs.clone(), cond.clone()),
                            self.gen(subs.clone(), body.clone()),
                        )
                    })
                    .collect(),
                else_case: if let Some(els) = else_case {
                    Some(box self.gen(subs, *els))
                } else {
                    None
                },
            },
            TypedNode::While { ty, cond, body } => LLVMNode::While {
                ty: llvm(ty),
                cond: box self.gen(subs.clone(), *cond),
                body: box self.gen(subs, *body),
            },
            TypedNode::For {
                ty,
                var,
                start,
                end,
                step,
                body,
            } => LLVMNode::For {
                ty: llvm(ty),
                body: box self.gen(subs.clone(), *body),
                var,
                end: box self.gen(subs.clone(), *end),
                start: box self.gen(subs.clone(), *start),
                step: box self.gen(subs.clone(), *step),
            },
            TypedNode::Array { ty, elements } => LLVMNode::Array {
                ty: llvm(ty),
                elements: elements
                    .iter()
                    .map(|elem| self.gen(subs.clone(), elem.clone()))
                    .collect(),
            },
            TypedNode::Index { ty, array, idx } => LLVMNode::Index {
                ty: llvm(ty),
                array: box self.gen(subs.clone(), *array),
                idx: box self.gen(subs.clone(), *idx),
            },
            TypedNode::Object { ty, properties } => LLVMNode::Object {
                ty: llvm(ty),
                properties: properties
                    .iter()
                    .map(|(name, node)| (name.clone(), self.gen(subs.clone(), node.clone())))
                    .collect(),
            },
            TypedNode::ObjectAccess {
                ty,
                object,
                property,
            } => LLVMNode::ObjectAccess {
                ty: llvm(ty),
                object: box self.gen(subs, *object),
                property,
            },
            TypedNode::ObjectEdit {
                ty,
                property,
                object,
                new_val,
            } => LLVMNode::ObjectEdit {
                ty: llvm(ty),
                new_val: box self.gen(subs.clone(), *new_val),
                object: box self.gen(subs.clone(), *object),
                property,
            },
            TypedNode::ObjectMethodCall {
                ty,
                property,
                object,
                args,
            } => LLVMNode::ObjectMethodCall {
                ty: llvm(ty),
                property,
                args: args
                    .iter()
                    .map(|x| self.gen(subs.clone(), x.clone()).clone())
                    .collect(),
                object: box self.gen(subs, *object),
            },
            TypedNode::Class {
                ty,
                properties,
                methods,
                constructor,
                name,
            } => LLVMNode::Class {
                ty: llvm(ty),
                name,
                constructor: box self.gen(subs.clone(), *constructor),
                properties: properties
                    .iter()
                    .map(|(name, node)| (name.clone(), self.gen(subs.clone(), node.clone())))
                    .collect(),
                methods: methods
                    .iter()
                    .map(|(name, node)| (name.clone(), self.gen(subs.clone(), node.clone())))
                    .collect(),
            },
            TypedNode::ClassInit {
                ty,
                class,
                constructor_params,
            } => LLVMNode::ClassInit {
                ty: llvm(ty),
                class: llvm(class),
                constructor_params: constructor_params
                    .iter()
                    .map(|x| self.gen(subs.clone(), x.clone()))
                    .collect(),
            },
        }
    }
}
