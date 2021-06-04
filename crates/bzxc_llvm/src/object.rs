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

use bzxc_shared::{Error, Node, Position, Token};
use inkwell::values::BasicValueEnum;

use crate::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub(crate) fn obj_decl(
        &self,
        properties: Vec<(Token, Node)>,
        pos: (Position, Position),
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        Err(self.error(pos, "Node can't be compiled"))
    }

    pub(crate) fn obj_get(
        &self,
        object: Node,
        property: Token,
        pos: (Position, Position),
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        Err(self.error(pos, "Node can't be compiled"))
    }

    pub(crate) fn obj_edit(
        &self,
        object: Node,
        property: Token,
        new_val: Node,
        pos: (Position, Position),
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        Err(self.error(pos, "Node can't be compiled"))
    }
}
