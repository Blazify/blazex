/*
   Copyright 2021 BlazifyOrg
   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at
       http://www.apache.org/licenses/LICENSE-2.0
   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

use bzs_shared::{ByteCode, Constants};
use std::{cell::RefCell, collections::HashMap, mem::MaybeUninit, rc::Rc};

const STACK_SIZE: usize = 512;
const SYM_ARR_SIZE: usize = 50;

#[derive(Debug, Clone, PartialEq)]
pub enum Konstants {
    None,
    Null,
    Int(i128),
    Float(f64),
    String(String),
    Char(char),
    Boolean(bool),
    Array(Vec<Konstants>),
    Object(HashMap<usize, Konstants>),
    Function(Vec<u16>, VM),
}

impl Konstants {
    pub fn property_edit(&mut self, i: usize, val: Konstants) {
        match self {
            Self::Object(map) => {
                map.insert(i, val);
            }
            _ => panic!("property_edit called on unexpected type"),
        }
    }
}

type K = Rc<RefCell<Konstants>>;

fn make_k(k: Konstants) -> K {
    Rc::new(RefCell::new(k))
}

type Symbol = Option<(K, bool)>;

pub fn convert_to_usize(int1: u8, int2: u8) -> usize {
    ((int1 as usize) << 8) | int2 as usize
}

#[derive(Debug, Clone, PartialEq)]
pub struct VM {
    bytecode: ByteCode,
    stack: [K; STACK_SIZE],
    stack_ptr: usize,
    symbols: Vec<[Symbol; SYM_ARR_SIZE]>,
    pub return_val: Rc<RefCell<Konstants>>,
}

impl VM {
    pub fn new(bytecode: ByteCode, symbols: Option<Vec<[Symbol; SYM_ARR_SIZE]>>) -> Self {
        Self {
            bytecode,
            stack: unsafe {
                let mut data: [MaybeUninit<K>; STACK_SIZE] = MaybeUninit::uninit().assume_init();

                for elem in &mut data[..] {
                    *elem = MaybeUninit::new(make_k(Konstants::None));
                }

                std::mem::transmute::<_, [K; STACK_SIZE]>(data)
            },
            stack_ptr: 0,
            symbols: if symbols.is_none() {
                const S: Symbol = None;
                vec![[S; SYM_ARR_SIZE]]
            } else {
                symbols.unwrap()
            },
            return_val: make_k(Konstants::None),
        }
    }

    pub fn run(&mut self) {
        let mut ip = 0;
        while ip < self.bytecode.instructions.len() {
            let address = ip;
            ip += 1;

            match self.bytecode.instructions[address] {
                0x01 => {
                    let idx = convert_to_usize(
                        self.bytecode.instructions[ip],
                        self.bytecode.instructions[ip + 1],
                    );
                    ip += 2;
                    let k = self.bytecode.constants[idx].clone();
                    let konstant = match k {
                        Constants::RawArray(e) => {
                            let mut arr = vec![];
                            let vm = VM::new(
                                ByteCode {
                                    instructions: vec![],
                                    constants: vec![],
                                },
                                Some(self.symbols.clone()),
                            );
                            for i in &e {
                                let mut v_cl = vm.clone();
                                v_cl.bytecode = i.clone();
                                v_cl.run();
                                arr.push(v_cl.stack[0].borrow().clone());
                            }
                            Konstants::Array(arr)
                        }
                        Constants::RawObject(map) => {
                            let mut props = HashMap::new();
                            let vm = VM::new(
                                ByteCode {
                                    instructions: vec![],
                                    constants: vec![],
                                },
                                Some(self.symbols.clone()),
                            );
                            for (k, v) in &map {
                                let mut v_clone = vm.clone();
                                v_clone.bytecode = v.clone();
                                v_clone.run();
                                props.insert(k.clone(), v_clone.stack[0].borrow().clone());
                            }
                            Konstants::Object(props)
                        }
                        Constants::Function(args, body) => {
                            let fun_vm = VM::new(body, Some(self.symbols.clone()));
                            Konstants::Function(args, fun_vm)
                        }
                        Constants::RawClass(constr, klass) => {
                            let mut vm = VM::new(
                                ByteCode {
                                    constants: vec![],
                                    instructions: vec![],
                                },
                                Some(self.symbols.clone()),
                            );
                            let mut args = vec![];
                            match constr.clone() {
                                Some((a, b)) => {
                                    vm.bytecode = b;
                                    args.extend(a);
                                }
                                None => (),
                            }

                            let soul = make_k(Konstants::Object(HashMap::new()));
                            for (k, v) in &klass {
                                let mut v_clone = vm.clone();
                                v_clone.bytecode = v.clone();
                                v_clone.symbols.last_mut().unwrap()[0] =
                                    Some((soul.clone(), false));
                                v_clone.run();
                                soul.borrow_mut()
                                    .property_edit(k.clone(), v_clone.stack[0].borrow().clone());
                            }
                            vm.symbols.last_mut().unwrap()[0] = Some((soul.clone(), false));
                            vm.return_val = soul.clone();
                            Konstants::Function(args, vm)
                        }
                        Constants::None => Konstants::None,
                        Constants::Null => Konstants::Null,
                        Constants::Int(x) => Konstants::Int(x),
                        Constants::Float(x) => Konstants::Float(x),
                        Constants::String(x) => Konstants::String(x),
                        Constants::Char(x) => Konstants::Char(x),
                        Constants::Boolean(x) => Konstants::Boolean(x),
                    };
                    self.push(make_k(konstant));
                }
                0x02 => {
                    self.pop();
                }
                0x03 => match (self.pop().borrow().clone(), self.pop().borrow().clone()) {
                    (Konstants::Int(rhs), Konstants::Int(lhs)) => {
                        self.push(make_k(Konstants::Int(lhs + rhs)))
                    }
                    (Konstants::Float(rhs), Konstants::Float(lhs)) => {
                        self.push(make_k(Konstants::Float(lhs + rhs)))
                    }
                    (Konstants::String(rhs), Konstants::String(lhs)) => {
                        self.push(make_k(Konstants::String(lhs + &rhs)))
                    }
                    _ => panic!("Unknown types to OpAdd"),
                },
                0x04 => match (self.pop().borrow().clone(), self.pop().borrow().clone()) {
                    (Konstants::Int(rhs), Konstants::Int(lhs)) => {
                        self.push(make_k(Konstants::Int(lhs - rhs)))
                    }
                    (Konstants::Float(rhs), Konstants::Float(lhs)) => {
                        self.push(make_k(Konstants::Float(lhs - rhs)))
                    }
                    _ => panic!("Unknown types to OpSub"),
                },
                0x05 => match (self.pop().borrow().clone(), self.pop().borrow().clone()) {
                    (Konstants::Int(rhs), Konstants::Int(lhs)) => {
                        self.push(make_k(Konstants::Int(lhs * rhs)))
                    }
                    (Konstants::Float(rhs), Konstants::Float(lhs)) => {
                        self.push(make_k(Konstants::Float(lhs * rhs)))
                    }
                    (Konstants::Int(rhs), Konstants::String(lhs)) => {
                        self.push(make_k(Konstants::String(lhs.repeat(rhs as usize))))
                    }
                    _ => panic!("Unknown types to OpMultiply"),
                },
                0x06 => match (self.pop().borrow().clone(), self.pop().borrow().clone()) {
                    (Konstants::Int(rhs), Konstants::Int(lhs)) => {
                        self.push(make_k(Konstants::Int(lhs / rhs)))
                    }
                    (Konstants::Float(rhs), Konstants::Float(lhs)) => {
                        self.push(make_k(Konstants::Float(lhs / rhs)))
                    }
                    (Konstants::Int(rhs), Konstants::String(lhs)) => self.push(make_k(
                        Konstants::String((lhs.as_bytes()[rhs as usize] as char).to_string()),
                    )),
                    _ => panic!("Unknown types to OpDivide"),
                },
                0x07 => match (self.pop().borrow().clone(), self.pop().borrow().clone()) {
                    (Konstants::Int(rhs), Konstants::Int(lhs)) => {
                        self.push(make_k(Konstants::Int(lhs.pow(rhs as u32))))
                    }
                    (Konstants::Float(rhs), Konstants::Float(lhs)) => {
                        self.push(make_k(Konstants::Float(lhs.powf(rhs))))
                    }
                    _ => panic!("Unknown types to OpPower"),
                },
                0x08 => {
                    ip = convert_to_usize(
                        self.bytecode.instructions[ip],
                        self.bytecode.instructions[ip + 1],
                    );
                }
                0x09 => match self.pop().borrow().clone() {
                    Konstants::Boolean(b) => {
                        if !b {
                            ip = convert_to_usize(
                                self.bytecode.instructions[ip],
                                self.bytecode.instructions[ip + 1],
                            );
                        } else {
                            ip += 2;
                        }
                    }
                    _ => panic!("Unknown types to OpJump"),
                },
                0x0A => match self.pop().borrow().clone() {
                    Konstants::Int(num) => self.push(make_k(Konstants::Int(num * 1))),
                    Konstants::Float(num) => self.push(make_k(Konstants::Float(num * 1.0))),
                    _ => panic!("Unknown arg type to OpPlus"),
                },
                0x0B => match self.pop().borrow().clone() {
                    Konstants::Int(num) => self.push(make_k(Konstants::Int(num * -1))),
                    Konstants::Float(num) => self.push(make_k(Konstants::Float(num * -1.0))),
                    _ => panic!("Unknown arg type to OpMinus"),
                },
                0x0C => match self.pop().borrow().clone() {
                    Konstants::Boolean(boolean) => self.push(make_k(Konstants::Boolean(!boolean))),
                    _ => panic!("Unknown arg type to OpNot"),
                },
                0x0D => match (self.pop().borrow().clone(), self.pop().borrow().clone()) {
                    (Konstants::Boolean(rhs), Konstants::Boolean(lhs)) => {
                        self.push(make_k(Konstants::Boolean(lhs && rhs)))
                    }
                    _ => panic!("Unknown types to OpAnd"),
                },
                0x0E => match (self.pop().borrow().clone(), self.pop().borrow().clone()) {
                    (Konstants::Boolean(rhs), Konstants::Boolean(lhs)) => {
                        self.push(make_k(Konstants::Boolean(lhs || rhs)))
                    }
                    _ => panic!("Unknown types to OpOr"),
                },
                0x0F => match (self.pop().borrow().clone(), self.pop().borrow().clone()) {
                    (Konstants::Int(rhs), Konstants::Int(lhs)) => {
                        self.push(make_k(Konstants::Boolean(lhs == rhs)))
                    }
                    (Konstants::Float(rhs), Konstants::Float(lhs)) => {
                        self.push(make_k(Konstants::Boolean(lhs == rhs)))
                    }
                    (Konstants::String(rhs), Konstants::String(lhs)) => {
                        self.push(make_k(Konstants::Boolean(lhs == rhs)))
                    }
                    (Konstants::Char(rhs), Konstants::Char(lhs)) => {
                        self.push(make_k(Konstants::Boolean(lhs == rhs)))
                    }
                    (Konstants::Boolean(rhs), Konstants::Boolean(lhs)) => {
                        self.push(make_k(Konstants::Boolean(lhs == rhs)))
                    }
                    _ => panic!("Unknown types to OpEquals"),
                },
                0x1A => match (self.pop().borrow().clone(), self.pop().borrow().clone()) {
                    (Konstants::Int(rhs), Konstants::Int(lhs)) => {
                        self.push(make_k(Konstants::Boolean(lhs != rhs)))
                    }
                    (Konstants::Float(rhs), Konstants::Float(lhs)) => {
                        self.push(make_k(Konstants::Boolean(lhs != rhs)))
                    }
                    (Konstants::String(rhs), Konstants::String(lhs)) => {
                        self.push(make_k(Konstants::Boolean(lhs != rhs)))
                    }
                    (Konstants::Char(rhs), Konstants::Char(lhs)) => {
                        self.push(make_k(Konstants::Boolean(lhs != rhs)))
                    }
                    (Konstants::Boolean(rhs), Konstants::Boolean(lhs)) => {
                        self.push(make_k(Konstants::Boolean(lhs != rhs)))
                    }
                    _ => panic!("Unknown types to OpNotEquals"),
                },
                0x1B => match (self.pop().borrow().clone(), self.pop().borrow().clone()) {
                    (Konstants::Int(rhs), Konstants::Int(lhs)) => {
                        self.push(make_k(Konstants::Boolean(lhs > rhs)))
                    }
                    (Konstants::Float(rhs), Konstants::Float(lhs)) => {
                        self.push(make_k(Konstants::Boolean(lhs > rhs)))
                    }
                    (Konstants::String(rhs), Konstants::String(lhs)) => {
                        self.push(make_k(Konstants::Boolean(lhs > rhs)))
                    }
                    (Konstants::Char(rhs), Konstants::Char(lhs)) => {
                        self.push(make_k(Konstants::Boolean(lhs > rhs)))
                    }
                    (Konstants::Boolean(rhs), Konstants::Boolean(lhs)) => {
                        self.push(make_k(Konstants::Boolean(lhs > rhs)))
                    }
                    _ => panic!("Unknown types to OpGreaterThan"),
                },
                0x1C => match (self.pop().borrow().clone(), self.pop().borrow().clone()) {
                    (Konstants::Int(rhs), Konstants::Int(lhs)) => {
                        self.push(make_k(Konstants::Boolean(lhs >= rhs)))
                    }
                    (Konstants::Float(rhs), Konstants::Float(lhs)) => {
                        self.push(make_k(Konstants::Boolean(lhs >= rhs)))
                    }
                    (Konstants::String(rhs), Konstants::String(lhs)) => {
                        self.push(make_k(Konstants::Boolean(lhs >= rhs)))
                    }
                    (Konstants::Char(rhs), Konstants::Char(lhs)) => {
                        self.push(make_k(Konstants::Boolean(lhs >= rhs)))
                    }
                    (Konstants::Boolean(rhs), Konstants::Boolean(lhs)) => {
                        self.push(make_k(Konstants::Boolean(lhs >= rhs)))
                    }
                    _ => panic!("Unknown types to OpGreaterThanEquals"),
                },
                0x1D => match (self.pop().borrow().clone(), self.pop().borrow().clone()) {
                    (Konstants::Int(rhs), Konstants::Int(lhs)) => {
                        self.push(make_k(Konstants::Boolean(lhs < rhs)))
                    }
                    (Konstants::Float(rhs), Konstants::Float(lhs)) => {
                        self.push(make_k(Konstants::Boolean(lhs < rhs)))
                    }
                    (Konstants::String(rhs), Konstants::String(lhs)) => {
                        self.push(make_k(Konstants::Boolean(lhs < rhs)))
                    }
                    (Konstants::Char(rhs), Konstants::Char(lhs)) => {
                        self.push(make_k(Konstants::Boolean(lhs < rhs)))
                    }
                    (Konstants::Boolean(rhs), Konstants::Boolean(lhs)) => {
                        self.push(make_k(Konstants::Boolean(lhs < rhs)))
                    }
                    _ => panic!("Unknown types to OpLessThan"),
                },
                0x1E => match (self.pop().borrow().clone(), self.pop().borrow().clone()) {
                    (Konstants::Int(rhs), Konstants::Int(lhs)) => {
                        self.push(make_k(Konstants::Boolean(lhs <= rhs)))
                    }
                    (Konstants::Float(rhs), Konstants::Float(lhs)) => {
                        self.push(make_k(Konstants::Boolean(lhs <= rhs)))
                    }
                    (Konstants::String(rhs), Konstants::String(lhs)) => {
                        self.push(make_k(Konstants::Boolean(lhs <= rhs)))
                    }
                    (Konstants::Char(rhs), Konstants::Char(lhs)) => {
                        self.push(make_k(Konstants::Boolean(lhs <= rhs)))
                    }
                    (Konstants::Boolean(rhs), Konstants::Boolean(lhs)) => {
                        self.push(make_k(Konstants::Boolean(lhs <= rhs)))
                    }
                    _ => panic!("Unknown types to OpLessThanEquals"),
                },
                0x1F => match self.pop().borrow().clone() {
                    Konstants::Boolean(b) => {
                        let i = convert_to_usize(
                            self.bytecode.instructions[ip],
                            self.bytecode.instructions[ip + 1],
                        );
                        ip += 2;
                        if self.get_symbol(i).is_some() {
                            panic!("Variable already assigned")
                        }
                        let n = self.pop();
                        self.symbols.last_mut().unwrap()[i] = Some((n, b));
                    }
                    _ => panic!("Unknown types to OpVarAssign"),
                },
                0x2A => {
                    let i = convert_to_usize(
                        self.bytecode.instructions[ip],
                        self.bytecode.instructions[ip + 1],
                    );
                    ip += 2;
                    self.push(
                        self.get_symbol(i)
                            .as_ref()
                            .expect("Variable not found")
                            .0
                            .clone(),
                    );
                }
                0x2B => {
                    let i = convert_to_usize(
                        self.bytecode.instructions[ip],
                        self.bytecode.instructions[ip + 1],
                    );
                    ip += 2;
                    if self.get_symbol(i).is_none() {
                        panic!("No variable found to be reassigned")
                    }

                    if !self.get_symbol(i).as_ref().unwrap().1 {
                        panic!("Variable not reassignable")
                    }

                    let n = self.pop();
                    self.get_set_symbols(i, Some((n, true)));
                }
                0x2C => {
                    const S: Symbol = None;
                    self.symbols.push([S; SYM_ARR_SIZE]);
                }
                0x2D => {
                    self.symbols.pop();
                }
                0x2E => match self.pop().borrow().clone() {
                    Konstants::Function(args, mut vm) => {
                        let eval_args = self.pop().borrow().clone();
                        if let Konstants::Array(a) = eval_args {
                            if a.len() != args.len() {
                                panic!("Expected {} args but found {}", args.len(), a.len());
                            }
                            let mut i = 0;
                            for arg in args {
                                vm.symbols.last_mut().unwrap()[arg as usize] =
                                    Some((make_k(a.get(i).unwrap().clone()), true));
                                i += 1;
                            }
                        } else {
                            panic!("Unknown args")
                        }
                        vm.run();
                        self.symbols = vm.symbols.clone();
                        self.push(vm.return_val.clone());
                    }
                    _ => panic!("Unknown Types applied to OpCall"),
                },
                0x2F => match (self.pop().borrow().clone(), self.pop().borrow().clone()) {
                    (Konstants::Int(i), Konstants::Array(a)) => self.push(make_k(
                        a.get(i as usize).expect("Index out of bound").clone(),
                    )),
                    _ => panic!("Unknown types applied to OpIndexArray"),
                },
                0x3A => match self.pop().borrow().clone() {
                    Konstants::Object(a) => {
                        let i = convert_to_usize(
                            self.bytecode.instructions[ip],
                            self.bytecode.instructions[ip + 1],
                        );
                        ip += 2;
                        self.push(make_k(a.get(&i).expect("Property not found").clone()));
                    }
                    _ => panic!("Unknown types applied to OpPropertyAcess"),
                },
                0x3B => {
                    let val = self.pop();
                    let obj = self.pop();

                    let i = convert_to_usize(
                        self.bytecode.instructions[ip],
                        self.bytecode.instructions[ip + 1],
                    );
                    ip += 2;

                    obj.borrow_mut().property_edit(i, val.borrow().clone());
                }
                0x3C => {
                    if self.return_val.borrow().clone() == Konstants::None {
                        self.return_val = self.pop().clone();
                    }
                    return;
                }
                _ => panic!(
                    "\nPrevious instruction {}\nCurrent Instruction: {}\nNext Instruction: {}\n",
                    self.bytecode.instructions[address - 1],
                    self.bytecode.instructions[address],
                    self.bytecode.instructions[address + 1]
                ),
            }
        }
    }

    pub fn push(&mut self, node: K) {
        self.stack[self.stack_ptr] = node;
        self.stack_ptr += 1;
    }

    pub fn pop(&mut self) -> K {
        let node = self.stack[if self.stack_ptr == 0 {
            self.stack_ptr
        } else {
            self.stack_ptr - 1
        }]
        .clone();
        self.stack_ptr -= if self.stack_ptr == 0 { 0 } else { 1 };
        node
    }

    pub fn pop_last(&self) -> K {
        self.stack[self.stack_ptr].clone()
    }

    pub fn get_symbol(&self, k: usize) -> &Symbol {
        for idx in (0..self.symbols.len()).rev() {
            let sym = &self.symbols.get(idx).unwrap()[k];
            if sym.is_some() {
                return sym;
            }
        }
        &None
    }

    pub fn get_set_symbols(&mut self, k: usize, n: Symbol) {
        for idx in (0..self.symbols.len()).rev() {
            let sym = &self.symbols.get(idx).unwrap()[k];
            if sym.is_some() {
                self.symbols.get_mut(idx).unwrap()[k] = n;
                break;
            }
        }
    }
}
