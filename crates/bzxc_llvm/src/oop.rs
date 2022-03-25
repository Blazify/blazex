use llvm_sys::core::{
    LLVMBuildInsertValue, LLVMBuildStore, LLVMBuildStructGEP, LLVMConstNull, LLVMFunctionType,
    LLVMGetArrayLength, LLVMGetElementType, LLVMGetReturnType, LLVMGetUndef, LLVMPointerType,
    LLVMStructGetTypeAtIndex, LLVMTypeOf,
};
use llvm_sys::prelude::{LLVMTypeRef, LLVMValueRef};

use bzxc_shared::{to_c_str, LLVMNode};

use crate::Compiler;

impl Compiler {
    pub(super) unsafe fn create_obj(
        &mut self,
        ty: LLVMTypeRef,
        properties: Vec<(String, LLVMNode)>,
    ) -> LLVMValueRef {
        let aligner_ty = LLVMGetElementType(LLVMStructGetTypeAtIndex(LLVMGetElementType(ty), 0));

        let aligner = LLVMGetArrayLength(aligner_ty) as usize as u32;

        let aligner_val = LLVMConstNull(aligner_ty);
        let aligner_ptr = self.create_entry_block_alloca("aligner", aligner_ty);
        LLVMBuildStore(self.builder, aligner_val, aligner_ptr);

        let struct_val = LLVMBuildInsertValue(
            self.builder,
            LLVMGetUndef(LLVMGetElementType(ty)),
            aligner_ptr,
            0,
            to_c_str("c_to_bzx_obj_load").as_ptr(),
        );

        let ptr = self.create_entry_block_alloca("obj", LLVMTypeOf(struct_val));
        LLVMBuildStore(self.builder, struct_val, ptr);

        for (i, (name, val)) in properties.iter().enumerate() {
            let idx = i + 1;
            self.objects.insert((name.clone(), aligner), idx);
            LLVMBuildStore(
                self.builder,
                self.compile(val.clone()),
                LLVMBuildStructGEP(
                    self.builder,
                    ptr,
                    idx as u32,
                    to_c_str("struct_gep").as_ptr(),
                ),
            );
        }

        ptr
    }

    pub(super) unsafe fn obj_property(
        &mut self,
        object: LLVMValueRef,
        property: String,
    ) -> LLVMValueRef {
        let i = self
            .objects
            .get(&(
                property,
                LLVMGetArrayLength(LLVMGetElementType(LLVMStructGetTypeAtIndex(
                    LLVMGetElementType(LLVMTypeOf(object)),
                    0,
                ))) as usize as u32,
            ))
            .unwrap();

        LLVMBuildStructGEP(
            self.builder,
            object,
            *i as u32,
            to_c_str("obj_load").as_ptr(),
        )
    }

    pub(super) unsafe fn class_method(
        &mut self,
        class: String,
        klass: LLVMTypeRef,
        method: LLVMNode,
    ) -> LLVMValueRef {
        match method {
            LLVMNode::Fun {
                body,
                name,
                params,
                ty,
            } => {
                let mut n_params = vec![("soul".to_string(), klass)];
                n_params.extend(params);

                let mut pty = n_params
                    .iter()
                    .map(|(_, ty)| ty.clone())
                    .collect::<Vec<_>>();

                let ty = LLVMFunctionType(
                    LLVMGetReturnType(LLVMGetElementType(ty)),
                    pty.as_mut_ptr(),
                    pty.len() as u32,
                    0,
                );
                self.compile(LLVMNode::Fun {
                    body,
                    name: format!("{}%{}", class, name),
                    params: n_params.clone(),
                    ty: LLVMPointerType(ty, 0),
                })
            }
            _ => unreachable!(),
        }
    }
}
