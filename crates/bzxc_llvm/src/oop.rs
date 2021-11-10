use bzxc_llvm_wrapper::{
    types::BasicTypeEnum,
    values::{BasicValueEnum, PointerValue},
};
use bzxc_shared::LLVMNode;

use crate::Compiler;

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub(super) fn create_obj(
        &mut self,
        ty: BasicTypeEnum<'ctx>,
        properties: Vec<(String, LLVMNode<'ctx>)>,
    ) -> BasicValueEnum<'ctx> {
        let ty = ty.into_pointer_type().get_element_type().into_struct_type();
        let mut struct_val = self
            .builder
            .build_insert_value(
                ty.get_undef(),
                ty.get_field_type_at_index(0).unwrap().const_zero(),
                0,
                "%alignment%",
            )
            .unwrap()
            .into_struct_value();
        for (i, (name, val)) in properties.iter().enumerate() {
            let idx = i + 1;
            self.objects.insert((name.clone(), ty), idx);
            struct_val = self
                .builder
                .build_insert_value(
                    struct_val,
                    self.compile(val.clone()),
                    idx as u32,
                    name.as_str(),
                )
                .unwrap()
                .into_struct_value();
        }

        let struct_ptr = self
            .builder
            .build_alloca(struct_val.get_type(), "struct_alloca");
        self.builder.build_store(struct_ptr, struct_val);
        struct_ptr.into()
    }

    pub(super) fn obj_property(
        &mut self,
        object: PointerValue<'ctx>,
        property: String,
    ) -> PointerValue<'ctx> {
        let i = self
            .objects
            .get(&(
                property,
                object.get_type().get_element_type().into_struct_type(),
            ))
            .unwrap();

        self.builder
            .build_struct_gep(object, *i as u32, "struct_gep")
            .ok()
            .unwrap()
    }
}