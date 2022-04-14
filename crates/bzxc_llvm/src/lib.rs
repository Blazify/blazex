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

use llvm_sys::analysis::LLVMVerifierFailureAction::LLVMPrintMessageAction;
use llvm_sys::analysis::LLVMVerifyFunction;
use llvm_sys::core::{
    LLVMAddFunction, LLVMAppendBasicBlockInContext, LLVMBuildAdd, LLVMBuildAlloca, LLVMBuildAnd,
    LLVMBuildBr, LLVMBuildCall, LLVMBuildCondBr, LLVMBuildFAdd, LLVMBuildFCmp, LLVMBuildFDiv,
    LLVMBuildFMul, LLVMBuildFRem, LLVMBuildFSub, LLVMBuildGEP, LLVMBuildGlobalString,
    LLVMBuildICmp, LLVMBuildInsertValue, LLVMBuildIntCast, LLVMBuildLoad, LLVMBuildMul,
    LLVMBuildOr, LLVMBuildPointerCast, LLVMBuildRet, LLVMBuildStore, LLVMBuildStructGEP,
    LLVMBuildSub, LLVMBuildUDiv, LLVMBuildURem, LLVMConstInt, LLVMConstNeg, LLVMConstNot,
    LLVMConstNull, LLVMConstReal, LLVMCountStructElementTypes, LLVMCreateBuilderInContext,
    LLVMDeleteFunction, LLVMDumpModule, LLVMDumpValue, LLVMFunctionType, LLVMGetArrayLength,
    LLVMGetElementType, LLVMGetFirstBasicBlock, LLVMGetFirstInstruction, LLVMGetInsertBlock,
    LLVMGetNamedFunction, LLVMGetParam, LLVMGetStructElementTypes, LLVMGetUndef,
    LLVMInsertBasicBlockInContext, LLVMInt1TypeInContext, LLVMInt32TypeInContext,
    LLVMIsAConstantInt, LLVMPositionBuilderAtEnd, LLVMPositionBuilderBefore,
    LLVMRunFunctionPassManager, LLVMSetLinkage, LLVMSetValueName2, LLVMStructGetTypeAtIndex,
    LLVMStructTypeInContext, LLVMTypeOf,
};
use llvm_sys::prelude::{
    LLVMBuilderRef, LLVMContextRef, LLVMModuleRef, LLVMPassManagerRef, LLVMTypeRef, LLVMValueRef,
};
use llvm_sys::LLVMIntPredicate::{
    LLVMIntEQ, LLVMIntNE, LLVMIntUGE, LLVMIntUGT, LLVMIntULE, LLVMIntULT,
};
use llvm_sys::LLVMLinkage::LLVMExternalLinkage;
use llvm_sys::LLVMRealPredicate::{
    LLVMRealOEQ, LLVMRealONE, LLVMRealUGE, LLVMRealUGT, LLVMRealULE, LLVMRealULT,
};
use std::collections::HashMap;
use std::convert::TryInto;
use std::mem::forget;

use bzxc_shared::{to_c_str, LLVMNode, Tokens};

mod oop;

#[derive(Debug, Clone)]
pub struct Compiler {
    pub context: LLVMContextRef,
    pub builder: LLVMBuilderRef,
    pub module: LLVMModuleRef,
    pub fpm: LLVMPassManagerRef,
    pub main: LLVMNode,

    fn_value_opt: Option<LLVMValueRef>,
    variables: HashMap<String, LLVMValueRef>,
    objects: HashMap<(String, u32), usize>,
    classes: HashMap<u32, (LLVMValueRef, LLVMValueRef, HashMap<String, LLVMValueRef>)>,
    ret: bool,
}

impl Compiler {
    pub fn init(
        context: LLVMContextRef,
        builder: LLVMBuilderRef,
        module: LLVMModuleRef,
        fpm: LLVMPassManagerRef,
        main: LLVMNode,
    ) -> Self {
        Self {
            builder,
            context,
            fpm,
            module,
            main,
            fn_value_opt: None,
            variables: HashMap::new(),
            objects: HashMap::new(),
            classes: HashMap::new(),
            ret: false,
        }
    }

    fn fn_value(&self) -> LLVMValueRef {
        self.fn_value_opt.unwrap()
    }

    unsafe fn create_entry_block_alloca(&self, name: &str, ty: LLVMTypeRef) -> LLVMValueRef {
        let builder = LLVMCreateBuilderInContext(self.context);

        let entry = LLVMGetFirstBasicBlock(self.fn_value());
        let instr = LLVMGetFirstInstruction(entry);

        if instr.is_null() {
            LLVMPositionBuilderAtEnd(builder, entry);
        } else {
            LLVMPositionBuilderBefore(builder, instr);
        }

        LLVMBuildAlloca(builder, ty, to_c_str(name).as_ptr())
    }

    unsafe fn null(&self) -> LLVMValueRef {
        let null = LLVMGetUndef(LLVMStructTypeInContext(self.context, [].as_mut_ptr(), 0, 0));
        let ptr = self.create_entry_block_alloca("null", LLVMTypeOf(null));
        LLVMBuildStore(self.builder, null, ptr);

        ptr
    }

    pub unsafe fn compile_main(&mut self) {
        let func = LLVMAddFunction(
            self.module,
            to_c_str("main").as_ptr(),
            LLVMFunctionType(LLVMInt32TypeInContext(self.context), [].as_mut_ptr(), 0, 0),
        );

        let entry = LLVMAppendBasicBlockInContext(self.context, func, to_c_str("entry").as_ptr());
        LLVMPositionBuilderAtEnd(self.builder, entry);

        self.fn_value_opt = Some(func);
        self.compile(self.main.clone());

        LLVMBuildRet(
            self.builder,
            LLVMConstInt(
                LLVMInt32TypeInContext(self.context),
                0.try_into().unwrap(),
                0,
            ),
        );

        self.ret = true;

        if LLVMVerifyFunction(func, LLVMPrintMessageAction) == 0 {
            LLVMRunFunctionPassManager(self.fpm, func);
        } else {
            LLVMDumpModule(self.module);
            LLVMDeleteFunction(func);
        }
    }

    unsafe fn compile(&mut self, node: LLVMNode) -> LLVMValueRef {
        match node {
            LLVMNode::Statements(stmts) => {
                let mut last = None;
                for statement in stmts {
                    last = Some(self.compile(statement));
                    if self.ret {
                        break;
                    }
                }

                last.unwrap_or_else(|| self.null())
            }
            LLVMNode::Int { ty, val } => LLVMConstInt(ty, val.try_into().unwrap(), 0),
            LLVMNode::Float { ty, val } => LLVMConstReal(ty, val.try_into().unwrap()),
            LLVMNode::Boolean { ty, val } => LLVMConstInt(ty, val.try_into().unwrap(), 0),
            LLVMNode::Char { ty, val } => LLVMConstInt(ty, val.try_into().unwrap(), 0),
            LLVMNode::String { ty, val } => LLVMBuildPointerCast(
                self.builder,
                LLVMBuildGlobalString(
                    self.builder,
                    to_c_str(val.as_str()).as_ptr(),
                    to_c_str("glob_str").as_ptr(),
                ),
                ty,
                to_c_str("str").as_ptr(),
            ),
            LLVMNode::Unary {
                ty: _,
                val,
                op_token,
            } => {
                let val = self.compile(*val);
                if !LLVMIsAConstantInt(val).is_null() {
                    match op_token.value {
                        Tokens::Plus => val,
                        Tokens::Minus => LLVMConstNeg(val),
                        Tokens::Keyword("not") => LLVMConstNot(val),
                        _ => unreachable!(),
                    }
                } else {
                    match op_token.value {
                        Tokens::Plus => val,
                        Tokens::Minus => LLVMConstNeg(val),
                        _ => unreachable!(),
                    }
                }
            }
            LLVMNode::Binary {
                ty: _,
                left,
                right,
                op_token,
            } => {
                let lhs = self.compile(*left);
                let rhs = self.compile(*right);
                if !LLVMIsAConstantInt(lhs).is_null() {
                    match op_token.value {
                        Tokens::Plus => {
                            LLVMBuildAdd(self.builder, lhs, rhs, to_c_str("tmpadd").as_ptr())
                        }
                        Tokens::Minus => {
                            LLVMBuildSub(self.builder, lhs, rhs, to_c_str("tmpsub").as_ptr())
                        }
                        Tokens::Multiply => {
                            LLVMBuildMul(self.builder, lhs, rhs, to_c_str("tmpmul").as_ptr())
                        }
                        Tokens::Divide => {
                            LLVMBuildUDiv(self.builder, lhs, rhs, to_c_str("tmpdiv").as_ptr())
                        }
                        Tokens::Modulo => {
                            LLVMBuildURem(self.builder, lhs, rhs, to_c_str("tmpmod").as_ptr())
                        }
                        Tokens::LessThan => LLVMBuildIntCast(
                            self.builder,
                            LLVMBuildICmp(
                                self.builder,
                                LLVMIntULT,
                                lhs,
                                rhs,
                                to_c_str("tmpcmp").as_ptr(),
                            ),
                            LLVMInt1TypeInContext(self.context),
                            to_c_str("bool_cast").as_ptr(),
                        ),
                        Tokens::GreaterThan => LLVMBuildIntCast(
                            self.builder,
                            LLVMBuildICmp(
                                self.builder,
                                LLVMIntUGT,
                                lhs,
                                rhs,
                                to_c_str("tmpcmp").as_ptr(),
                            ),
                            LLVMInt1TypeInContext(self.context),
                            to_c_str("bool_cast").as_ptr(),
                        ),
                        Tokens::LessThanEquals => LLVMBuildIntCast(
                            self.builder,
                            LLVMBuildICmp(
                                self.builder,
                                LLVMIntULE,
                                lhs,
                                rhs,
                                to_c_str("tmpcmp").as_ptr(),
                            ),
                            LLVMInt1TypeInContext(self.context),
                            to_c_str("bool_cast").as_ptr(),
                        ),
                        Tokens::GreaterThanEquals => LLVMBuildIntCast(
                            self.builder,
                            LLVMBuildICmp(
                                self.builder,
                                LLVMIntUGE,
                                lhs,
                                rhs,
                                to_c_str("tmpcmp").as_ptr(),
                            ),
                            LLVMInt1TypeInContext(self.context),
                            to_c_str("bool_cast").as_ptr(),
                        ),
                        Tokens::DoubleEquals => LLVMBuildIntCast(
                            self.builder,
                            LLVMBuildICmp(
                                self.builder,
                                LLVMIntEQ,
                                lhs,
                                rhs,
                                to_c_str("tmpcmp").as_ptr(),
                            ),
                            LLVMInt1TypeInContext(self.context),
                            to_c_str("bool_cast").as_ptr(),
                        ),
                        Tokens::NotEquals => LLVMBuildIntCast(
                            self.builder,
                            LLVMBuildICmp(
                                self.builder,
                                LLVMIntNE,
                                lhs,
                                rhs,
                                to_c_str("tmpcmp").as_ptr(),
                            ),
                            LLVMInt1TypeInContext(self.context),
                            to_c_str("bool_cast").as_ptr(),
                        ),
                        _ => {
                            if op_token.value == Tokens::Keyword("and") {
                                LLVMBuildAnd(self.builder, lhs, rhs, to_c_str("tmpand").as_ptr())
                            } else if op_token.value == Tokens::Keyword("or") {
                                LLVMBuildOr(self.builder, lhs, rhs, to_c_str("tmpor").as_ptr())
                            } else {
                                unreachable!();
                            }
                        }
                    }
                } else {
                    match op_token.value {
                        Tokens::Plus => {
                            LLVMBuildFAdd(self.builder, lhs, rhs, to_c_str("tmpadd").as_ptr())
                        }
                        Tokens::Minus => {
                            LLVMBuildFSub(self.builder, lhs, rhs, to_c_str("tmpsub").as_ptr())
                        }
                        Tokens::Multiply => {
                            LLVMBuildFMul(self.builder, lhs, rhs, to_c_str("tmpmul").as_ptr())
                        }
                        Tokens::Divide => {
                            LLVMBuildFDiv(self.builder, lhs, rhs, to_c_str("tmpdiv").as_ptr())
                        }
                        Tokens::Modulo => {
                            LLVMBuildFRem(self.builder, lhs, rhs, to_c_str("tmpmod").as_ptr())
                        }
                        Tokens::LessThan => LLVMBuildIntCast(
                            self.builder,
                            LLVMBuildFCmp(
                                self.builder,
                                LLVMRealULT,
                                lhs,
                                rhs,
                                to_c_str("tmpcmp").as_ptr(),
                            ),
                            LLVMInt1TypeInContext(self.context),
                            to_c_str("bool_cast").as_ptr(),
                        ),
                        Tokens::GreaterThan => LLVMBuildIntCast(
                            self.builder,
                            LLVMBuildFCmp(
                                self.builder,
                                LLVMRealUGT,
                                lhs,
                                rhs,
                                to_c_str("tmpcmp").as_ptr(),
                            ),
                            LLVMInt1TypeInContext(self.context),
                            to_c_str("bool_cast").as_ptr(),
                        ),
                        Tokens::LessThanEquals => LLVMBuildIntCast(
                            self.builder,
                            LLVMBuildFCmp(
                                self.builder,
                                LLVMRealULE,
                                lhs,
                                rhs,
                                to_c_str("tmpcmp").as_ptr(),
                            ),
                            LLVMInt1TypeInContext(self.context),
                            to_c_str("bool_cast").as_ptr(),
                        ),
                        Tokens::GreaterThanEquals => LLVMBuildIntCast(
                            self.builder,
                            LLVMBuildFCmp(
                                self.builder,
                                LLVMRealUGE,
                                lhs,
                                rhs,
                                to_c_str("tmpcmp").as_ptr(),
                            ),
                            LLVMInt1TypeInContext(self.context),
                            to_c_str("bool_cast").as_ptr(),
                        ),
                        Tokens::DoubleEquals => LLVMBuildIntCast(
                            self.builder,
                            LLVMBuildFCmp(
                                self.builder,
                                LLVMRealOEQ,
                                lhs,
                                rhs,
                                to_c_str("tmpcmp").as_ptr(),
                            ),
                            LLVMInt1TypeInContext(self.context),
                            to_c_str("bool_cast").as_ptr(),
                        ),
                        Tokens::NotEquals => LLVMBuildIntCast(
                            self.builder,
                            LLVMBuildFCmp(
                                self.builder,
                                LLVMRealONE,
                                lhs,
                                rhs,
                                to_c_str("tmpcmp").as_ptr(),
                            ),
                            LLVMInt1TypeInContext(self.context),
                            to_c_str("bool_cast").as_ptr(),
                        ),
                        _ => unreachable!(),
                    }
                }
            }
            LLVMNode::Fun {
                ty,
                name,
                params,
                body,
            } => {
                let func = LLVMAddFunction(
                    self.module,
                    to_c_str(name.as_str()).as_ptr(),
                    LLVMGetElementType(ty),
                );

                let parent = self.fn_value_opt.clone();

                let parental_block = LLVMGetInsertBlock(self.builder);

                let entry =
                    LLVMAppendBasicBlockInContext(self.context, func, to_c_str("entry").as_ptr());
                LLVMPositionBuilderAtEnd(self.builder, entry);

                self.fn_value_opt = Some(func);

                self.variables.reserve(params.len());

                for i in 0..params.len() {
                    let arg = LLVMGetParam(func, i as u32);
                    let arg_name = params.get(i).unwrap().0.as_str().clone();
                    LLVMSetValueName2(arg, to_c_str(arg_name).as_ptr(), arg_name.len());
                    let alloca = self.create_entry_block_alloca(arg_name, LLVMTypeOf(arg));

                    LLVMBuildStore(self.builder, arg, alloca);
                    self.variables.insert(arg_name.to_string(), alloca);
                }

                let ret = self.ret.clone();
                self.ret = false;
                self.compile(*body);

                if !self.ret {
                    LLVMBuildRet(self.builder, self.null());
                }

                LLVMPositionBuilderAtEnd(self.builder, parental_block);
                self.fn_value_opt = parent;

                self.ret = ret;

                if LLVMVerifyFunction(func, LLVMPrintMessageAction) == 0 {
                    LLVMRunFunctionPassManager(self.fpm, func);
                } else {
                    LLVMDumpValue(func);
                    LLVMDeleteFunction(func);
                }

                let alloca = self.create_entry_block_alloca(name.as_str(), LLVMTypeOf(func));
                LLVMBuildStore(self.builder, func, alloca);
                self.variables.insert(name, alloca);
                func
            }
            LLVMNode::Extern {
                ty: _,
                name,
                return_type,
                args,
                var_args,
            } => {
                let ret = LLVMTypeOf(self.compile(*return_type));
                let mut args = args
                    .iter()
                    .map(|arg| LLVMTypeOf(self.compile(arg.clone())))
                    .collect::<Vec<_>>();

                let func = LLVMAddFunction(
                    self.module,
                    to_c_str(name.as_str()).as_ptr(),
                    LLVMFunctionType(ret, args.as_mut_ptr(), args.len() as u32, var_args as i32),
                );

                LLVMSetLinkage(func, LLVMExternalLinkage);

                self.null()
            }
            LLVMNode::Let { ty: _, name, val } => {
                let val = self.compile(*val);
                let alloca = self.create_entry_block_alloca(name.as_str(), LLVMTypeOf(val));
                LLVMBuildStore(self.builder, val, alloca);
                self.variables.insert(name.to_string(), alloca);
                self.null()
            }
            LLVMNode::Var { ty: _, name } => match self.variables.get(name.as_str()) {
                Some(val) => LLVMBuildLoad(self.builder, *val, to_c_str(name.as_str()).as_ptr()),
                None => LLVMGetNamedFunction(self.module, to_c_str(name.as_str()).as_ptr()),
            },
            LLVMNode::Call { ty: _, fun, args } => {
                let mut args = args
                    .iter()
                    .map(|arg| self.compile(arg.clone()))
                    .collect::<Vec<_>>();

                LLVMBuildCall(
                    self.builder,
                    self.compile(*fun),
                    args.as_mut_ptr(),
                    args.len() as u32,
                    to_c_str("call_fun").as_ptr(),
                )
            }
            LLVMNode::Return { ty: _, val } => {
                let rett = self.compile(*val);
                LLVMBuildRet(self.builder, rett);
                self.ret = true;
                rett
            }
            LLVMNode::Null { ty } => LLVMGetUndef(ty),
            LLVMNode::If {
                ty: _,
                cases,
                else_case,
            } => {
                let mut blocks = vec![LLVMGetInsertBlock(self.builder)];

                let parent = self.fn_value();
                for _ in 1..cases.len() {
                    blocks.push(LLVMAppendBasicBlockInContext(
                        self.context,
                        parent,
                        to_c_str("if_start").as_ptr(),
                    ));
                }

                let else_block = if else_case.is_some() {
                    let result = LLVMAppendBasicBlockInContext(
                        self.context,
                        parent,
                        to_c_str("else").as_ptr(),
                    );
                    blocks.push(result);
                    Some(result)
                } else {
                    None
                };

                let after_block =
                    LLVMAppendBasicBlockInContext(self.context, parent, to_c_str("after").as_ptr());
                blocks.push(after_block);

                for (i, (cond, body)) in cases.iter().enumerate() {
                    let then_block = blocks[i];
                    let else_block = blocks[i + 1];

                    LLVMPositionBuilderAtEnd(self.builder, then_block);

                    let condition = self.compile(cond.clone());
                    let conditional_block = LLVMInsertBasicBlockInContext(
                        self.context,
                        else_block,
                        to_c_str("if_body").as_ptr(),
                    );

                    LLVMBuildCondBr(self.builder, condition, conditional_block, else_block);

                    LLVMPositionBuilderAtEnd(self.builder, conditional_block);
                    self.compile(body.clone());
                    if !self.ret {
                        LLVMBuildBr(self.builder, after_block);
                    };
                }

                if let Some(else_block) = else_block {
                    LLVMPositionBuilderAtEnd(self.builder, else_block);
                    self.compile(*else_case.unwrap());
                    LLVMBuildBr(self.builder, after_block);
                }

                LLVMPositionBuilderAtEnd(self.builder, after_block);
                self.ret = false;

                self.null()
            }
            LLVMNode::While { ty: _, cond, body } => {
                let parent = self.fn_value();
                let cond_block = LLVMAppendBasicBlockInContext(
                    self.context,
                    parent,
                    to_c_str("while_cond").as_ptr(),
                );
                let body_block = LLVMAppendBasicBlockInContext(
                    self.context,
                    parent,
                    to_c_str("while_body").as_ptr(),
                );
                let after_block = LLVMAppendBasicBlockInContext(
                    self.context,
                    parent,
                    to_c_str("while_after").as_ptr(),
                );
                LLVMBuildBr(self.builder, cond_block);
                LLVMPositionBuilderAtEnd(self.builder, cond_block);

                let cond = self.compile(*cond.clone());
                LLVMBuildCondBr(self.builder, cond, body_block, after_block);
                LLVMPositionBuilderAtEnd(self.builder, body_block);
                self.compile(*body.clone());
                LLVMBuildBr(self.builder, cond_block);

                LLVMPositionBuilderAtEnd(self.builder, after_block);

                self.ret = false;
                self.null()
            }
            LLVMNode::For {
                ty: _,
                var: var_,
                start,
                end,
                step,
                body,
            } => {
                let parent = self.fn_value();
                let start = self.compile(*start);
                let step = self.compile(*step);
                let end = LLVMBuildAdd(
                    self.builder,
                    self.compile(*end),
                    step,
                    to_c_str("end").as_ptr(),
                );

                let var = self.create_entry_block_alloca(var_.as_str(), LLVMTypeOf(start));
                LLVMBuildStore(self.builder, start, var);
                self.variables.insert(var_, var);

                let cond_block = LLVMAppendBasicBlockInContext(
                    self.context,
                    parent,
                    to_c_str("for_cond").as_ptr(),
                );
                let body_block = LLVMAppendBasicBlockInContext(
                    self.context,
                    parent,
                    to_c_str("for_body").as_ptr(),
                );
                let after_block = LLVMAppendBasicBlockInContext(
                    self.context,
                    parent,
                    to_c_str("for_after").as_ptr(),
                );
                LLVMBuildBr(self.builder, cond_block);
                LLVMPositionBuilderAtEnd(self.builder, cond_block);

                let cond = LLVMBuildICmp(
                    self.builder,
                    LLVMIntNE,
                    LLVMBuildLoad(self.builder, var, to_c_str("for_cond").as_ptr()),
                    end,
                    to_c_str("cond").as_ptr(),
                );
                LLVMBuildCondBr(self.builder, cond, body_block, after_block);
                LLVMPositionBuilderAtEnd(self.builder, body_block);

                self.compile(*body.clone());

                let curr = LLVMBuildLoad(self.builder, var, to_c_str("curr_val").as_ptr());
                let new = LLVMBuildAdd(self.builder, curr, step, to_c_str("new_val").as_ptr());
                LLVMBuildStore(self.builder, new, var);

                LLVMBuildBr(self.builder, cond_block);

                LLVMPositionBuilderAtEnd(self.builder, after_block);

                self.ret = false;

                self.null()
            }
            LLVMNode::Array { ty, elements } => {
                let arr = self.create_entry_block_alloca("array_alloca", LLVMGetElementType(ty));

                for (i, element) in elements.iter().enumerate() {
                    let element = self.compile(element.clone());
                    let element_ptr = LLVMBuildGEP(
                        self.builder,
                        arr,
                        [
                            LLVMConstInt(LLVMInt32TypeInContext(self.context), 0, 0),
                            LLVMConstInt(LLVMInt32TypeInContext(self.context), i as u64, 0),
                        ]
                        .as_mut_ptr(),
                        2,
                        to_c_str("array_element").as_ptr(),
                    );
                    LLVMBuildStore(self.builder, element, element_ptr);
                }

                arr.into()
            }
            LLVMNode::Index { ty: _, array, idx } => {
                let arr = self.compile(*array);

                let idx = self.compile(*idx);

                let element_ptr = LLVMBuildGEP(
                    self.builder,
                    arr,
                    [
                        LLVMConstInt(LLVMInt32TypeInContext(self.context), 0, 0),
                        idx,
                    ]
                    .as_mut_ptr(),
                    2,
                    to_c_str("array_element").as_ptr(),
                );

                LLVMBuildLoad(self.builder, element_ptr, to_c_str("load").as_ptr())
            }
            LLVMNode::Object { ty, properties } => self.create_obj(ty, properties),
            LLVMNode::CObject { ty: _ty, object } => {
                let obj = self.compile(*object);
                let object = LLVMBuildLoad(self.builder, obj, to_c_str("load").as_ptr());
                let count = LLVMCountStructElementTypes(LLVMTypeOf(object));
                let mut raw_vec: Vec<LLVMTypeRef> = Vec::with_capacity(count as usize);
                let ptr = raw_vec.as_mut_ptr();
                forget(raw_vec);

                let mut c_obj_fields = {
                    LLVMGetStructElementTypes(LLVMTypeOf(object), ptr);
                    Vec::from_raw_parts(ptr, count as usize, count as usize)
                };
                c_obj_fields.remove(0);
                let struct_val = LLVMGetUndef(LLVMStructTypeInContext(
                    self.context,
                    c_obj_fields.as_mut_ptr(),
                    c_obj_fields.len() as u32,
                    0,
                ));

                let alloc = self.create_entry_block_alloca("alloc", LLVMTypeOf(struct_val));
                LLVMBuildStore(self.builder, struct_val, alloc);

                for i in 1..count {
                    let val = LLVMBuildLoad(
                        self.builder,
                        LLVMBuildStructGEP(self.builder, obj, i, to_c_str("struct_gep").as_ptr()),
                        to_c_str("struct_load").as_ptr(),
                    );

                    LLVMBuildStore(
                        self.builder,
                        val,
                        LLVMBuildStructGEP(
                            self.builder,
                            alloc,
                            i - 1,
                            to_c_str("struct_gep").as_ptr(),
                        ),
                    );
                }

                alloc
            }
            LLVMNode::CToBzxObject { ty, object } => {
                let ty = LLVMGetElementType(ty);
                let struct_val = LLVMBuildInsertValue(
                    self.builder,
                    LLVMGetUndef(ty),
                    LLVMConstNull(LLVMStructGetTypeAtIndex(ty, 0)),
                    0,
                    to_c_str("c_to_bzx_obj_load").as_ptr(),
                );

                let obj = self.compile(*object);
                let object = LLVMBuildLoad(self.builder, obj, to_c_str("load").as_ptr());

                let alloc = self.create_entry_block_alloca("alloc", LLVMTypeOf(struct_val));
                LLVMBuildStore(self.builder, struct_val, alloc);

                for i in 0..LLVMCountStructElementTypes(LLVMTypeOf(object)) {
                    let val = LLVMBuildLoad(
                        self.builder,
                        LLVMBuildStructGEP(self.builder, obj, i, to_c_str("struct_gep").as_ptr()),
                        to_c_str("struct_load").as_ptr(),
                    );

                    LLVMBuildStore(
                        self.builder,
                        val,
                        LLVMBuildStructGEP(
                            self.builder,
                            alloc,
                            i + 1,
                            to_c_str("struct_gep").as_ptr(),
                        ),
                    );
                }

                alloc
            }
            LLVMNode::ObjectAccess {
                ty: _,
                object,
                property,
            } => {
                let obj = self.compile(*object);
                let ptr = self.obj_property(obj, property.clone());

                LLVMBuildLoad(self.builder, ptr, to_c_str(property.as_str()).as_ptr())
            }
            LLVMNode::ObjectEdit {
                ty: _,
                object,
                property,
                new_val,
            } => {
                let val = self.compile(*new_val);

                let struct_ty = self.compile(*object);
                let ptr = self.obj_property(struct_ty, property);
                LLVMBuildStore(self.builder, val, ptr);

                struct_ty
            }
            LLVMNode::ObjectMethodCall {
                ty: _,
                property,
                object,
                args,
            } => {
                let ptr = self.compile(*object.clone());

                let class = self
                    .classes
                    .get(
                        &(LLVMGetArrayLength(LLVMGetElementType(LLVMStructGetTypeAtIndex(
                            LLVMGetElementType(LLVMTypeOf(ptr)),
                            0,
                        ))) as usize as u32),
                    )
                    .clone();
                let is_class = class.is_some();

                let mut compiled_args = Vec::with_capacity(args.len());

                if is_class {
                    compiled_args.push(ptr);
                }
                for arg in args {
                    let compiled = self.clone().compile(arg.clone());
                    compiled_args.push(compiled);
                }

                if !is_class {
                    let func = self.obj_property(ptr, property.clone());
                    let call = LLVMBuildCall(
                        self.builder,
                        func,
                        compiled_args.as_mut_ptr(),
                        compiled_args.len() as u32,
                        to_c_str(property.as_str()).as_ptr(),
                    );

                    return call;
                }

                let func = class.clone().unwrap().2.get(&property).unwrap().clone();

                let call = LLVMBuildCall(
                    self.builder,
                    func,
                    compiled_args.as_mut_ptr(),
                    compiled_args.len() as u32,
                    to_c_str(property.as_str()).as_ptr(),
                );

                call
            }
            LLVMNode::Class {
                ty,
                properties,
                methods,
                constructor,
                name: class,
                static_obj,
            } => {
                let static_obj = self.compile(*static_obj);
                let alloca = self.create_entry_block_alloca("class", LLVMTypeOf(static_obj));
                LLVMBuildStore(self.builder, static_obj, alloca);
                self.variables.insert(class.clone(), alloca);

                let klass = self.create_obj(ty, properties);
                let constructor = self.class_method(class.clone(), LLVMTypeOf(klass), *constructor);

                let mut n_methods = HashMap::new();

                for (name, method) in methods {
                    n_methods.insert(name, self.class_method(class.clone(), ty, method));
                }
                self.classes.insert(
                    LLVMGetArrayLength(LLVMGetElementType(LLVMStructGetTypeAtIndex(
                        LLVMGetElementType(ty),
                        0,
                    ))) as usize as u32,
                    (klass, constructor, n_methods),
                );

                self.null()
            }
            LLVMNode::ClassInit {
                ty: _,
                class,
                constructor_params,
            } => {
                let (base, constructor, _) = self
                    .classes
                    .get(
                        &(LLVMGetArrayLength(LLVMGetElementType(LLVMStructGetTypeAtIndex(
                            LLVMGetElementType(class),
                            0,
                        ))) as usize as u32),
                    )
                    .unwrap()
                    .clone();

                let base = LLVMBuildLoad(self.builder, base, to_c_str("base").as_ptr());

                let ptr = self.create_entry_block_alloca("class", LLVMTypeOf(base));
                LLVMBuildStore(self.builder, base, ptr);

                let mut params = vec![ptr];
                params.extend(
                    constructor_params
                        .iter()
                        .map(|x| self.compile(x.clone()))
                        .collect::<Vec<_>>(),
                );
                LLVMBuildCall(
                    self.builder,
                    constructor,
                    params.as_mut_ptr(),
                    params.len() as u32,
                    to_c_str("constructor").as_ptr(),
                );

                ptr.into()
            }
        }
    }
}
