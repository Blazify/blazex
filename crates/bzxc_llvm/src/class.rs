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

use bzxc_llvm_wrapper::values::BasicValueEnum;
use bzxc_shared::{Error, Node, Position, Token, Type};

use crate::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub(crate) fn class_decl(
        &mut self,
        name: Token,
        constructor: (Vec<(Token, Type)>, Box<Node>),
        properties: Vec<(Token, Node)>,
        methods: Vec<(Token, Vec<(Token, Type)>, Node, Type)>,
        pos: (Position, Position),
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        println!(
            "{:#?}\n{:#?}\n{:#?}\n{:#?}",
            name, constructor, properties, methods
        );
        Err(self.error(pos, "Node can't be compiled"))
    }

    pub(crate) fn class_init(
        &self,
        name: Token,
        constructor_params: Vec<Node>,
        pos: (Position, Position),
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        Err(self.error(pos, "Node can't be compiled"))
    }
}
