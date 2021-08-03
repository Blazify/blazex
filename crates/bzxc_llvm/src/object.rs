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
use bzxc_shared::{Error, Node, Position, Token};

use crate::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub(crate) fn obj_decl(
        &mut self,
        properties: Vec<(Token, Node)>,
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        let arr = self
            .context
            .i8_type()
            .array_type(self.object_aligner)
            .const_zero();

        self.object_aligner += 1;

        let mut values = vec![arr.into()];
        let mut types = vec![arr.get_type().into()];
        let mut names = vec![String::new()];

        for (k, v) in &properties {
            let val = self.compile_node(v.clone())?;
            values.push(val);
            types.push(val.get_type());
            names.push(k.value.into_string());
        }

        let mut struct_val = self.context.struct_type(&types[..], false).get_undef();

        for (i, val) in values.iter().enumerate() {
            struct_val = self
                .builder
                .build_insert_value(struct_val, val.clone(), i as u32, "")
                .unwrap()
                .into_struct_value();
        }

        let struct_ptr = self
            .builder
            .build_alloca(struct_val.get_type(), "struct_alloca");
        self.builder.build_store(struct_ptr, struct_val);

        for (i, name) in names.iter().enumerate() {
            self.objects
                .insert((struct_ptr.get_type(), name.clone()), i as u32);
        }

        Ok(struct_ptr.into())
    }

    pub(crate) fn obj_get(
        &mut self,
        object: Node,
        property: Token,
        pos: (Position, Position),
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        let struct_val = self.compile_node(object)?;
        if !struct_val.is_pointer_value() {
            return Err(self.error(pos, "Expected 'object'"));
        }

        let prop = self
            .objects
            .get(&(
                struct_val.into_pointer_value().get_type(),
                property.value.into_string(),
            ))
            .ok_or(self.error(pos, "Property not found on object"))?;

        let val = self
            .builder
            .build_struct_gep(struct_val.into_pointer_value(), *prop, "extract_obj")
            .ok()
            .ok_or(self.error(pos, "Property not found on object"))?;

        Ok(self.builder.build_load(val, "obj_prop"))
    }

    pub(crate) fn obj_edit(
        &mut self,
        object: Node,
        property: Token,
        new_val: Node,
        pos: (Position, Position),
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        let val = self.compile_node(new_val)?;

        let struct_val = self.compile_node(object)?;

        if !struct_val.is_pointer_value() {
            return Err(self.error(pos, "Expected 'object'"));
        }

        let struct_ptr = struct_val.into_pointer_value();

        let i = self
            .objects
            .get(&(struct_ptr.get_type(), property.value.into_string()))
            .ok_or(self.error(pos, "Property not found on object"))?;

        let ptr = self
            .builder
            .build_struct_gep(struct_ptr, *i, "struct_gep")
            .ok()
            .ok_or(self.error(pos, "Property not found on object"))?;
        self.builder.build_store(ptr, val);

        Ok(struct_ptr.into())
    }
}
