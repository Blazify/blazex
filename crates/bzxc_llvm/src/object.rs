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
        &mut self,
        properties: Vec<(Token, Node)>,
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        let mut values = vec![];
        let mut types = vec![];
        let mut names = vec![];

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

        for (i, name) in names.iter().enumerate() {
            self.objects
                .insert((struct_val.get_type(), name.clone()), i);
        }

        Ok(struct_val.into())
    }

    pub(crate) fn obj_get(
        &mut self,
        object: Node,
        property: Token,
        pos: (Position, Position),
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        let struct_val = self.compile_node(object)?;
        if !struct_val.is_struct_value() {
            return Err(self.error(pos, "Expected 'object'"));
        }

        let prop = self
            .objects
            .get(&(
                struct_val.into_struct_value().get_type(),
                property.value.into_string(),
            ))
            .ok_or(self.error(pos, "Property not found on object"))?
            .clone();

        let val = self.builder.build_extract_value(
            struct_val.into_struct_value(),
            prop as u32,
            "extract_obj",
        );

        if let Some(v) = val {
            Ok(v)
        } else {
            Err(self.error(pos, "Property not found on object"))
        }
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

        if !struct_val.is_struct_value() {
            return Err(self.error(pos, "Expected 'object'"));
        }

        let i = self
            .objects
            .get(&(
                struct_val.into_struct_value().get_type(),
                property.value.into_string(),
            ))
            .ok_or(self.error(pos, "Property not found on object"))?
            .clone();

        let x = self.builder.build_extract_value(
            struct_val.into_struct_value(),
            i as u32,
            "extract_obj",
        );

        if x.is_none() {
            return Err(self.error(pos, "Property not found on object"));
        }

        if x.unwrap().get_type() != val.get_type() {
            return Err(self.error(pos, "Expected the type it was initialized with."));
        };

        Ok(self
            .builder
            .build_insert_value(struct_val.into_struct_value(), val.clone(), i as u32, "")
            .unwrap()
            .into_struct_value()
            .into())
    }
}