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
#![feature(box_syntax, box_patterns)]
#![allow(unused_variables)]

use std::collections::HashMap;

use bzxc_shared::{Node, Type, TypedNode};
use substitution::Substitution;
use type_env::TypeEnv;

mod annotate;
mod constraint;
pub mod llvm_node;
mod substitution;
mod type_env;
mod unifier;

pub struct TypeSystem {
    pub node: Node,
    methods: HashMap<Type, HashMap<String, Type>>,
}

impl TypeSystem {
    pub fn new(node: Node) -> Self {
        TypeSystem {
            node,
            methods: HashMap::new(),
        }
    }

    pub fn typed_node(&mut self) -> (Substitution, TypedNode) {
        let annotation = self.annotate(self.node.clone(), &mut TypeEnv::new());
        let constraints = self.collect(annotation.clone());
        let substitution = self.unify(constraints.clone());
        (substitution, annotation)
    }
}
