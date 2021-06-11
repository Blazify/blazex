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

use bzxc_llvm_wrapper::{types::BasicTypeEnum, values::BasicValueEnum};
use bzxc_shared::{Error, Node, Position};

use crate::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub(crate) fn array_decl(
        &mut self,
        element_nodes: Vec<Node>,
        pos: (Position, Position),
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        if element_nodes.is_empty() {
            return Ok(self.context.i128_type().array_type(0).const_zero().into());
        }

        let ty = self
            .compile_node(element_nodes.get(0).unwrap().clone())?
            .get_type();

        let size = element_nodes.len() as u32;
        let arr = match ty {
            BasicTypeEnum::ArrayType(x) => x.array_type(size),
            BasicTypeEnum::FloatType(x) => x.array_type(size),
            BasicTypeEnum::IntType(x) => x.array_type(size),
            BasicTypeEnum::PointerType(x) => x.array_type(size),
            BasicTypeEnum::StructType(x) => x.array_type(size),
            BasicTypeEnum::VectorType(x) => x.array_type(size),
        };

        let array_alloca = self.builder.build_alloca(arr, "array_alloca");
        let mut array = self
            .builder
            .build_load(array_alloca, "array_load")
            .into_array_value();

        for (i, k) in element_nodes.iter().enumerate() {
            let elem = self.compile_node(k.clone())?;

            if elem.get_type() != ty {
                return Err(self.error(pos, "Arrays cannot be of multiple types"));
            }

            array = self
                .builder
                .build_insert_value(array, elem, i as u32, "load_array")
                .unwrap()
                .into_array_value();
        }

        Ok(array.into())
    }

    pub(crate) fn array_access(
        &mut self,
        array: Node,
        index: Node,
        pos: (Position, Position),
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        let array = self.compile_node(array)?;
        if !array.is_array_value() {
            return Err(self.error(pos, "Expected a 'array'"));
        }
        let idx = self.compile_node(index)?;
        if !idx.is_int_value() {
            return Err(self.error(pos, "Expected a index"));
        }

        let arr = array.into_array_value();
        let array_alloca = self.builder.build_alloca(arr.get_type(), "arr_alloc");
        self.builder.build_store(array_alloca, arr);

        let array_elem_ptr = unsafe {
            self.builder.build_gep(
                array_alloca,
                &[
                    self.context.i32_type().const_int(0, false),
                    idx.into_int_value(),
                ],
                "get_array_elem_ptr",
            )
        };
        let array_elem = self.builder.build_load(array_elem_ptr, "array_elem");

        Ok(array_elem)
    }
}
