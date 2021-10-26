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

use bzxc_llvm_wrapper::{types::AnyTypeEnum, values::BasicValueEnum};
use bzxc_shared::TypedNode;

use crate::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub(crate) fn array_decl(
        &mut self,
        typee: &'ctx AnyTypeEnum<'ctx>,
        element_nodes: &'ctx [&'ctx TypedNode<'ctx>],
    ) -> BasicValueEnum<'ctx> {
        if element_nodes.is_empty() {
            return typee.array_type(0).const_zero().into();
        }

        let size = element_nodes.len() as u32;

        let array_alloca = self
            .builder
            .build_alloca(typee.array_type(size), "array_alloca");
        let mut array = self
            .builder
            .build_load(array_alloca, "array_load")
            .into_array_value();

        for (i, k) in element_nodes.iter().enumerate() {
            let elem = self.compile_node(*k.clone());

            array = self
                .builder
                .build_insert_value(array, elem, i as u32, "load_array")
                .unwrap()
                .into_array_value();
        }

        array.into()
    }

    pub(crate) fn array_access(
        &mut self,
        array: TypedNode<'ctx>,
        index: TypedNode<'ctx>,
    ) -> BasicValueEnum<'ctx> {
        let array = self.compile_node(array);
        let idx = self.compile_node(index);

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

        array_elem
    }
}
