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
use bzxc_shared::{Node, TypedNode};

/// NO idea how type system should be implemented
pub struct TypeSystem<'ctx> {
    pub node: Node,
    pub ctx: &'ctx Context,
}

impl<'ctx> TypeSystem<'ctx> {
    pub fn new(node: Node, ctx: &'ctx Context) -> Self {
        TypeSystem { node, ctx }
    }

    pub fn typed_node(&self) -> TypedNode<'ctx> {
        TypedNode::Statements { statements: &[] }
    }
}
