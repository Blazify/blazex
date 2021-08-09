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

use bzxc_llvm_wrapper::values::{BasicValueEnum, PointerValue};
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
        let ptr = self.obj_prop_pointer(struct_val, property.value.into_string(), pos)?;

        Ok(self.builder.build_load(ptr, "obj_prop"))
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
        let ptr = self.obj_prop_pointer(struct_val, property.value.into_string(), pos)?;
        self.builder.build_store(ptr, val);

        Ok(struct_val.into())
    }

    pub(crate) fn obj_method_call(
        &mut self,
        object: Node,
        property: Token,
        args: Vec<Node>,
        pos: (Position, Position),
    ) -> Result<BasicValueEnum<'ctx>, Error> {
        let struct_val = self.compile_node(object)?;
        if !struct_val.is_pointer_value() {
            return Err(self.error(pos, "Expected 'object'"));
        }
        let prop = property.value.into_string();

        let ptr = struct_val.into_pointer_value();
        if self
            .builder
            .build_load(ptr, "struct_check")
            .is_struct_value()
        {
            return Err(self.error(pos, "Expected 'object'"));
        }
        let class_name = self.classes.get(&ptr.get_type()).clone();
        let is_class = class_name.is_some();
        let method = if is_class {
            Some(class_name.unwrap().to_owned() + &prop)
        } else {
            None
        };

        let mut compiled_args = Vec::with_capacity(args.len());

        if is_class {
            compiled_args.push(ptr.into());
        }
        for arg in args {
            compiled_args.push(self.compile_node(arg)?);
        }

        if !is_class {
            let func = self.obj_prop_pointer(struct_val, prop, pos)?;
            let call = self
                .builder
                .build_call(func, &compiled_args[..], "obj_func_call")
                .ok()
                .unwrap();

            return Ok(call
                .try_as_basic_value()
                .left_or(self.context.i128_type().const_int(0, false).into()));
        }

        let fun = self.get_function(&method.unwrap());
        if fun.is_none() {
            return Err(self.error(pos, "No method found"));
        }

        let call = self
            .builder
            .build_call(fun.unwrap(), &compiled_args[..], "tmpcall")
            .ok()
            .unwrap();

        Ok(call
            .try_as_basic_value()
            .left_or(self.context.i128_type().const_int(0, false).into()))
    }

    fn obj_prop_pointer(
        &mut self,
        struct_val: BasicValueEnum<'ctx>,
        prop: String,
        pos: (Position, Position),
    ) -> Result<PointerValue<'ctx>, Error> {
        if !struct_val.is_pointer_value() {
            return Err(self.error(pos, "Expected 'object'"));
        }

        let struct_ptr = struct_val.into_pointer_value();

        if self
            .builder
            .build_load(struct_ptr, "struct_check")
            .is_struct_value()
        {
            return Err(self.error(pos, "Expected 'object'"));
        }

        let i = self
            .objects
            .get(&(struct_ptr.get_type(), prop))
            .ok_or(self.error(pos, "Property not found on object"))?;

        let ptr = self
            .builder
            .build_struct_gep(struct_ptr, *i, "struct_gep")
            .ok()
            .ok_or(self.error(pos, "Property not found on object"))?;

        Ok(ptr)
    }
}
