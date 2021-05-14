use bzs_shared::{ByteCode, Constants};

const STACK_SIZE: usize = 512;
const SYM_ARR_SIZE: usize = 50;
type Symbol = Option<(Constants, bool)>;

pub fn convert_to_usize(int1: u8, int2: u8) -> usize {
    ((int1 as usize) << 8) | int2 as usize
}

#[derive(Debug, Clone)]
pub struct VM {
    bytecode: ByteCode,
    stack: [Constants; STACK_SIZE],
    stack_ptr: usize,
    symbols: Vec<[Symbol; SYM_ARR_SIZE]>,
}

impl VM {
    pub fn new(bytecode: ByteCode, symbols: Option<Vec<[Symbol; SYM_ARR_SIZE]>>) -> Self {
        Self {
            bytecode,
            stack: unsafe { std::mem::zeroed() },
            stack_ptr: 0,
            symbols: if symbols.is_none() {
                const S: Symbol = None;
                vec![[S; SYM_ARR_SIZE]]
            } else {
                symbols.unwrap()
            },
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
                    self.push(k);
                }
                0x02 => {
                    self.pop();
                }
                0x03 => match (self.pop(), self.pop()) {
                    (Constants::Int(rhs), Constants::Int(lhs)) => {
                        self.push(Constants::Int(lhs + rhs))
                    }
                    (Constants::Float(rhs), Constants::Float(lhs)) => {
                        self.push(Constants::Float(lhs + rhs))
                    }
                    (Constants::String(rhs), Constants::String(lhs)) => {
                        self.push(Constants::String(lhs + &rhs))
                    }
                    _ => panic!("Unknown types to OpAdd"),
                },
                0x04 => match (self.pop(), self.pop()) {
                    (Constants::Int(rhs), Constants::Int(lhs)) => {
                        self.push(Constants::Int(lhs - rhs))
                    }
                    (Constants::Float(rhs), Constants::Float(lhs)) => {
                        self.push(Constants::Float(lhs - rhs))
                    }
                    _ => panic!("Unknown types to OpSub"),
                },
                0x05 => match (self.pop(), self.pop()) {
                    (Constants::Int(rhs), Constants::Int(lhs)) => {
                        self.push(Constants::Int(lhs * rhs))
                    }
                    (Constants::Float(rhs), Constants::Float(lhs)) => {
                        self.push(Constants::Float(lhs * rhs))
                    }
                    (Constants::Int(rhs), Constants::String(lhs)) => {
                        self.push(Constants::String(lhs.repeat(rhs as usize)))
                    }
                    _ => panic!("Unknown types to OpMultiply"),
                },
                0x06 => match (self.pop(), self.pop()) {
                    (Constants::Int(rhs), Constants::Int(lhs)) => {
                        self.push(Constants::Int(lhs / rhs))
                    }
                    (Constants::Float(rhs), Constants::Float(lhs)) => {
                        self.push(Constants::Float(lhs / rhs))
                    }
                    (Constants::Int(rhs), Constants::String(lhs)) => self.push(Constants::String(
                        (lhs.as_bytes()[rhs as usize] as char).to_string(),
                    )),
                    _ => panic!("Unknown types to OpDivide"),
                },
                0x07 => match (self.pop(), self.pop()) {
                    (Constants::Int(rhs), Constants::Int(lhs)) => {
                        self.push(Constants::Int(lhs.pow(rhs as u32)))
                    }
                    (Constants::Float(rhs), Constants::Float(lhs)) => {
                        self.push(Constants::Float(lhs.powf(rhs)))
                    }
                    _ => panic!("Unknown types to OpPower"),
                },
                0x08 => {
                    ip = convert_to_usize(
                        self.bytecode.instructions[ip],
                        self.bytecode.instructions[ip + 1],
                    );
                }
                0x09 => match self.pop() {
                    Constants::Boolean(b) => {
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
                0x0A => match self.pop() {
                    Constants::Int(num) => self.push(Constants::Int(num * 1)),
                    Constants::Float(num) => self.push(Constants::Float(num * 1.0)),
                    _ => panic!("Unknown arg type to OpPlus"),
                },
                0x0B => match self.pop() {
                    Constants::Int(num) => self.push(Constants::Int(num * -1)),
                    Constants::Float(num) => self.push(Constants::Float(num * -1.0)),
                    _ => panic!("Unknown arg type to OpMinus"),
                },
                0x0C => match self.pop() {
                    Constants::Boolean(boolean) => self.push(Constants::Boolean(!boolean)),
                    _ => panic!("Unknown arg type to OpNot"),
                },
                0x0D => match (self.pop(), self.pop()) {
                    (Constants::Boolean(rhs), Constants::Boolean(lhs)) => {
                        self.push(Constants::Boolean(lhs && rhs))
                    }
                    _ => panic!("Unknown types to OpAnd"),
                },
                0x0E => match (self.pop(), self.pop()) {
                    (Constants::Boolean(rhs), Constants::Boolean(lhs)) => {
                        self.push(Constants::Boolean(lhs || rhs))
                    }
                    _ => panic!("Unknown types to OpOr"),
                },
                0x0F => match (self.pop(), self.pop()) {
                    (Constants::Int(rhs), Constants::Int(lhs)) => {
                        self.push(Constants::Boolean(lhs == rhs))
                    }
                    (Constants::Float(rhs), Constants::Float(lhs)) => {
                        self.push(Constants::Boolean(lhs == rhs))
                    }
                    (Constants::String(rhs), Constants::String(lhs)) => {
                        self.push(Constants::Boolean(lhs == rhs))
                    }
                    (Constants::Char(rhs), Constants::Char(lhs)) => {
                        self.push(Constants::Boolean(lhs == rhs))
                    }
                    (Constants::Boolean(rhs), Constants::Boolean(lhs)) => {
                        self.push(Constants::Boolean(lhs == rhs))
                    }
                    _ => panic!("Unknown types to OpEquals"),
                },
                0x1A => match (self.pop(), self.pop()) {
                    (Constants::Int(rhs), Constants::Int(lhs)) => {
                        self.push(Constants::Boolean(lhs != rhs))
                    }
                    (Constants::Float(rhs), Constants::Float(lhs)) => {
                        self.push(Constants::Boolean(lhs != rhs))
                    }
                    (Constants::String(rhs), Constants::String(lhs)) => {
                        self.push(Constants::Boolean(lhs != rhs))
                    }
                    (Constants::Char(rhs), Constants::Char(lhs)) => {
                        self.push(Constants::Boolean(lhs != rhs))
                    }
                    (Constants::Boolean(rhs), Constants::Boolean(lhs)) => {
                        self.push(Constants::Boolean(lhs != rhs))
                    }
                    _ => panic!("Unknown types to OpNotEquals"),
                },
                0x1B => match (self.pop(), self.pop()) {
                    (Constants::Int(rhs), Constants::Int(lhs)) => {
                        self.push(Constants::Boolean(lhs > rhs))
                    }
                    (Constants::Float(rhs), Constants::Float(lhs)) => {
                        self.push(Constants::Boolean(lhs > rhs))
                    }
                    (Constants::String(rhs), Constants::String(lhs)) => {
                        self.push(Constants::Boolean(lhs > rhs))
                    }
                    (Constants::Char(rhs), Constants::Char(lhs)) => {
                        self.push(Constants::Boolean(lhs > rhs))
                    }
                    (Constants::Boolean(rhs), Constants::Boolean(lhs)) => {
                        self.push(Constants::Boolean(lhs > rhs))
                    }
                    _ => panic!("Unknown types to OpGreaterThan"),
                },
                0x1C => match (self.pop(), self.pop()) {
                    (Constants::Int(rhs), Constants::Int(lhs)) => {
                        self.push(Constants::Boolean(lhs >= rhs))
                    }
                    (Constants::Float(rhs), Constants::Float(lhs)) => {
                        self.push(Constants::Boolean(lhs >= rhs))
                    }
                    (Constants::String(rhs), Constants::String(lhs)) => {
                        self.push(Constants::Boolean(lhs >= rhs))
                    }
                    (Constants::Char(rhs), Constants::Char(lhs)) => {
                        self.push(Constants::Boolean(lhs >= rhs))
                    }
                    (Constants::Boolean(rhs), Constants::Boolean(lhs)) => {
                        self.push(Constants::Boolean(lhs >= rhs))
                    }
                    _ => panic!("Unknown types to OpGreaterThanEquals"),
                },
                0x1D => match (self.pop(), self.pop()) {
                    (Constants::Int(rhs), Constants::Int(lhs)) => {
                        self.push(Constants::Boolean(lhs < rhs))
                    }
                    (Constants::Float(rhs), Constants::Float(lhs)) => {
                        self.push(Constants::Boolean(lhs < rhs))
                    }
                    (Constants::String(rhs), Constants::String(lhs)) => {
                        self.push(Constants::Boolean(lhs < rhs))
                    }
                    (Constants::Char(rhs), Constants::Char(lhs)) => {
                        self.push(Constants::Boolean(lhs < rhs))
                    }
                    (Constants::Boolean(rhs), Constants::Boolean(lhs)) => {
                        self.push(Constants::Boolean(lhs < rhs))
                    }
                    _ => panic!("Unknown types to OpLessThan"),
                },
                0x1E => match (self.pop(), self.pop()) {
                    (Constants::Int(rhs), Constants::Int(lhs)) => {
                        self.push(Constants::Boolean(lhs <= rhs))
                    }
                    (Constants::Float(rhs), Constants::Float(lhs)) => {
                        self.push(Constants::Boolean(lhs <= rhs))
                    }
                    (Constants::String(rhs), Constants::String(lhs)) => {
                        self.push(Constants::Boolean(lhs <= rhs))
                    }
                    (Constants::Char(rhs), Constants::Char(lhs)) => {
                        self.push(Constants::Boolean(lhs <= rhs))
                    }
                    (Constants::Boolean(rhs), Constants::Boolean(lhs)) => {
                        self.push(Constants::Boolean(lhs <= rhs))
                    }
                    _ => panic!("Unknown types to OpLessThanEquals"),
                },
                0x1F => match self.pop() {
                    Constants::Boolean(b) => {
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
                0x2E => match self.pop() {
                    Constants::Function(args, body) => {
                        for arg in args {
                            let eval_arg = self.pop();
                            self.symbols.last_mut().unwrap()[arg as usize] = Some((eval_arg, true));
                        }
                        let mut fun_vm = VM::new(body, Some(self.symbols.clone()));
                        fun_vm.run();
                        self.push(fun_vm.pop_last().clone());
                        self.symbols = fun_vm.symbols;
                    }
                    _ => panic!("Unknown Types applied to OpCall"),
                },
                0x2F => match (self.pop(), self.pop()) {
                    (Constants::Int(i), Constants::Array(a)) => {
                        self.push(a.get(i as usize).unwrap().clone());
                    }
                    _ => panic!("Unknown types applied to OpIndexArray"),
                },
                0x3A => match self.pop() {
                    Constants::Object(a) => {
                        let i = convert_to_usize(
                            self.bytecode.instructions[ip],
                            self.bytecode.instructions[ip + 1],
                        );
                        ip += 2;

                        let btc = a.get(&i).unwrap().clone();
                        self.push(btc);
                    }
                    _ => panic!("Unknown types applied to OpPropertyAcess"),
                },
                _ => panic!(
                    "\nPrevious instruction {}\nCurrent Instruction: {}\nNext Instruction: {}\n",
                    self.bytecode.instructions[address - 1],
                    self.bytecode.instructions[address],
                    self.bytecode.instructions[address + 1]
                ),
            }
        }
    }

    pub fn push(&mut self, node: Constants) {
        self.stack[self.stack_ptr] = node;
        self.stack_ptr += 1;
    }

    pub fn pop(&mut self) -> Constants {
        let node = self.stack[if self.stack_ptr == 0 {
            self.stack_ptr
        } else {
            self.stack_ptr - 1
        }]
        .clone();
        self.stack_ptr -= if self.stack_ptr == 0 { 0 } else { 1 };
        node
    }

    pub fn pop_last(&self) -> &Constants {
        &self.stack[self.stack_ptr]
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