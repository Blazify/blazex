use crate::core::interpreter::{interpreter::Interpreter, runtime_result::RuntimeResult};
use crate::core::parser::nodes::Node;
use crate::core::token::Token;
use crate::utils::constants::DynType;
use crate::utils::{constants::Tokens, error::Error, position::Position};
use crate::utils::{context::Context, symbol::Symbol};
use std::collections::HashMap;
use std::convert::TryInto;
use std::fmt::{Display, Error as E, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int {
        val: i64,
        pos_start: Position,
        pos_end: Position,
    },
    Float {
        val: f32,
        pos_start: Position,
        pos_end: Position,
    },
    String {
        val: String,
        pos_start: Position,
        pos_end: Position,
    },
    Char {
        val: char,
        pos_start: Position,
        pos_end: Position,
    },
    Boolean {
        val: bool,
        pos_start: Position,
        pos_end: Position,
    },
    Function {
        name: Token,
        body_node: Node,
        args: Vec<Token>,
        pos_start: Position,
        pos_end: Position,
    },
    Array {
        elements: Vec<Value>,
        pos_start: Position,
        pos_end: Position,
    },
    Object {
        properties: HashMap<String, Value>,
        pos_start: Position,
        pos_end: Position,
    },
    Class {
        name: String,
        constructor: Option<Node>,
        properties: HashMap<String, Node>,
        methods: HashMap<String, Node>,
        pos_start: Position,
        pos_end: Position,
    },
    InBuiltFunction {
        name: String,
        fun: fn(Vec<Value>) -> Value,
        args_len: usize,
        pos_start: Position,
        pos_end: Position,
    },
    Null,
}

impl Value {
    pub fn get_int(self) -> i64 {
        match self {
            Self::Int { val, .. } => val,
            _ => panic!(),
        }
    }

    pub fn get_string(self) -> String {
        match self {
            Self::String { val, .. } => val,
            _ => panic!(),
        }
    }

    pub fn is_true(self) -> bool {
        match self.clone() {
            Self::Boolean { val, .. } => val,
            Self::Int { val, .. } => val != 0,
            Self::Float { val, .. } => val != 0.0,
            _ => panic!("Not bool convertible"),
        }
    }

    pub fn get_pos_start(&self) -> Position {
        match self {
            Value::Int { pos_start, .. } => *pos_start,
            Value::Float { pos_start, .. } => *pos_start,
            Value::String { pos_start, .. } => *pos_start,
            Value::Char { pos_start, .. } => *pos_start,
            Value::Boolean { pos_start, .. } => *pos_start,
            Value::Function { pos_start, .. } => *pos_start,
            Value::Array { pos_start, .. } => *pos_start,
            Value::Object { pos_start, .. } => *pos_start,
            Value::Class { pos_start, .. } => *pos_start,
            _ => panic!(),
        }
    }

    pub fn get_pos_end(&self) -> Position {
        match self {
            Value::Int { pos_end, .. } => *pos_end,
            Value::Float { pos_end, .. } => *pos_end,
            Value::String { pos_end, .. } => *pos_end,
            Value::Char { pos_end, .. } => *pos_end,
            Value::Boolean { pos_end, .. } => *pos_end,
            Value::Function { pos_end, .. } => *pos_end,
            Value::Array { pos_end, .. } => *pos_end,
            Value::Object { pos_end, .. } => *pos_end,
            Value::Class { pos_end, .. } => *pos_end,
            _ => panic!(),
        }
    }

    pub fn op(self, n: Value, op: Token) -> RuntimeResult {
        match self.clone() {
            Value::Int {
                val: v, pos_start, ..
            } => {
                if let Value::Int {
                    val: v1, pos_end, ..
                } = n
                {
                    return match op.r#type {
                        Tokens::Plus => RuntimeResult::new().success(Value::Int {
                            val: v + v1,
                            pos_start,
                            pos_end,
                        }),
                        Tokens::Minus => RuntimeResult::new().success(Value::Int {
                            val: v - v1,
                            pos_start,
                            pos_end,
                        }),
                        Tokens::Multiply => RuntimeResult::new().success(Value::Int {
                            val: v * v1,
                            pos_start,
                            pos_end,
                        }),
                        Tokens::Divide => RuntimeResult::new().success(Value::Int {
                            val: v / v1,
                            pos_start,
                            pos_end,
                        }),
                        Tokens::Power => RuntimeResult::new().success(Value::Int {
                            val: v.pow(v1.try_into().unwrap()),
                            pos_start,
                            pos_end,
                        }),
                        Tokens::DoubleEquals => RuntimeResult::new().success(Value::Boolean {
                            val: v == v1,
                            pos_start,
                            pos_end,
                        }),
                        Tokens::NotEquals => RuntimeResult::new().success(Value::Boolean {
                            val: v != v1,
                            pos_start,
                            pos_end,
                        }),
                        Tokens::LessThan => RuntimeResult::new().success(Value::Boolean {
                            val: v < v1,
                            pos_start,
                            pos_end,
                        }),
                        Tokens::LessThanEquals => RuntimeResult::new().success(Value::Boolean {
                            val: v <= v1,
                            pos_start,
                            pos_end,
                        }),
                        Tokens::GreaterThan => RuntimeResult::new().success(Value::Boolean {
                            val: v > v1,
                            pos_start,
                            pos_end,
                        }),
                        Tokens::GreaterThanEquals => RuntimeResult::new().success(Value::Boolean {
                            val: v >= v1,
                            pos_start,
                            pos_end,
                        }),
                        _ => RuntimeResult::new().failure(Error::new(
                            "Runtime Error",
                            pos_start,
                            self.clone().get_pos_end(),
                            "Unexpected token",
                        )),
                    };
                }
                RuntimeResult::new().failure(Error::new(
                    "Runtime Error",
                    pos_start,
                    self.clone().get_pos_end(),
                    "Unexpected type",
                ))
            }
            Value::Float {
                val: v, pos_start, ..
            } => {
                if let Value::Float {
                    val: v1, pos_end, ..
                } = n
                {
                    return match op.r#type {
                        Tokens::Plus => RuntimeResult::new().success(Value::Float {
                            val: v + v1,
                            pos_start,
                            pos_end,
                        }),
                        Tokens::Minus => RuntimeResult::new().success(Value::Float {
                            val: v - v1,
                            pos_start,
                            pos_end,
                        }),
                        Tokens::Multiply => RuntimeResult::new().success(Value::Float {
                            val: v * v1,
                            pos_start,
                            pos_end,
                        }),
                        Tokens::Divide => RuntimeResult::new().success(Value::Float {
                            val: v / v1,
                            pos_start,
                            pos_end,
                        }),
                        Tokens::Power => RuntimeResult::new().success(Value::Float {
                            val: v.powf(v1),
                            pos_start,
                            pos_end,
                        }),
                        Tokens::DoubleEquals => RuntimeResult::new().success(Value::Boolean {
                            val: v == v1,
                            pos_start,
                            pos_end,
                        }),
                        Tokens::NotEquals => RuntimeResult::new().success(Value::Boolean {
                            val: v != v1,
                            pos_start,
                            pos_end,
                        }),
                        Tokens::LessThan => RuntimeResult::new().success(Value::Boolean {
                            val: v < v1,
                            pos_start,
                            pos_end,
                        }),
                        Tokens::LessThanEquals => RuntimeResult::new().success(Value::Boolean {
                            val: v <= v1,
                            pos_start,
                            pos_end,
                        }),
                        Tokens::GreaterThan => RuntimeResult::new().success(Value::Boolean {
                            val: v > v1,
                            pos_start,
                            pos_end,
                        }),
                        Tokens::GreaterThanEquals => RuntimeResult::new().success(Value::Boolean {
                            val: v >= v1,
                            pos_start,
                            pos_end,
                        }),
                        _ => RuntimeResult::new().failure(Error::new(
                            "Runtime Error",
                            pos_start,
                            self.clone().get_pos_end(),
                            "Unexpected token",
                        )),
                    };
                }
                RuntimeResult::new().failure(Error::new(
                    "Runtime Error",
                    pos_start,
                    self.clone().get_pos_end(),
                    "Unexpected type",
                ))
            }
            Value::Boolean { val, pos_start, .. } => {
                if let Value::Boolean {
                    val: v1, pos_end, ..
                } = n
                {
                    if op
                        .clone()
                        .matches(Tokens::Keyword, DynType::String("and".to_string()))
                    {
                        return RuntimeResult::new().success(Value::Boolean {
                            val: val && v1,
                            pos_start,
                            pos_end,
                        });
                    } else if op
                        .clone()
                        .matches(Tokens::Keyword, DynType::String("or".to_string()))
                    {
                        return RuntimeResult::new().success(Value::Boolean {
                            val: val || v1,
                            pos_start,
                            pos_end,
                        });
                    }

                    match op.r#type {
                        Tokens::DoubleEquals => RuntimeResult::new().success(Value::Boolean {
                            val: val == v1,
                            pos_start,
                            pos_end,
                        }),
                        Tokens::NotEquals => RuntimeResult::new().success(Value::Boolean {
                            val: val == v1,
                            pos_start,
                            pos_end,
                        }),
                        _ => RuntimeResult::new().failure(Error::new(
                            "Runtime Error",
                            self.clone().get_pos_start(),
                            self.clone().get_pos_end(),
                            "Unexpected type",
                        )),
                    };
                }
                RuntimeResult::new().failure(Error::new(
                    "Runtime Error",
                    self.clone().get_pos_start(),
                    self.clone().get_pos_end(),
                    "Unexpected type",
                ))
            }
            _ => RuntimeResult::new().failure(Error::new(
                "Runtime Error",
                self.clone().get_pos_start(),
                self.clone().get_pos_end(),
                "Unexpected type",
            )),
        }
    }

    pub fn unary(self, u: Tokens) -> RuntimeResult {
        match self.clone() {
            Self::Int {
                val,
                pos_end,
                pos_start,
            } => {
                if let Tokens::Minus = u {
                    return RuntimeResult::new().success(Value::Int {
                        val: val * (-1),
                        pos_end,
                        pos_start,
                    });
                } else if let Tokens::Plus = u {
                    return RuntimeResult::new().success(Value::Int {
                        val: val * (1),
                        pos_end,
                        pos_start,
                    });
                }

                RuntimeResult::new().failure(Error::new(
                    "Runtime Error",
                    pos_start,
                    pos_end,
                    "Unexpected token",
                ))
            }
            _ => RuntimeResult::new().failure(Error::new(
                "Runtime Error",
                self.clone().get_pos_start(),
                self.clone().get_pos_end(),
                "Unexpected type",
            )),
        }
    }

    pub fn execute(self, eval_args: Vec<Value>, inter: &mut Interpreter) -> RuntimeResult {
        let mut res = RuntimeResult::new();
        if let Self::Function {
            args,
            body_node,
            name,
            pos_start,
            pos_end,
        } = self.clone()
        {
            let ctx = Context::new(name.value.into_string());
            inter.ctx.push(ctx);

            if args.len() > eval_args.len() {
                return res.failure(
                    Error::new(
                        "Runtime Error",
                        pos_start,
                        pos_end,
                        "Too less args supplied!",
                    )
                    .set_ctx(inter.ctx.clone()),
                );
            }

            if args.len() < eval_args.len() {
                return res.failure(
                    Error::new(
                        "Runtime Error",
                        pos_start,
                        pos_end,
                        "Too many args supplied!",
                    )
                    .set_ctx(inter.ctx.clone()),
                );
            }

            for x in 0..args.len() {
                let name = &args[x];
                let val = &eval_args[x];

                inter.ctx.last_mut().unwrap().symbols.insert(
                    name.clone().value.into_string(),
                    Symbol::new(val.clone(), true),
                );
            }
            let result = inter.interpret_node(body_node);
            if result.clone().should_return() {
                return result;
            }

            inter.ctx.pop();

            return res.success(result.val.unwrap());
        } else if let Self::InBuiltFunction {
            args_len,
            fun,
            pos_start,
            pos_end,
            ..
        } = self
        {
            if args_len > eval_args.len() {
                return res.failure(Error::new(
                    "Runtime Error",
                    pos_start,
                    pos_end,
                    "Too less args supplied!",
                ));
            }

            if args_len < eval_args.len() {
                return res.failure(Error::new(
                    "Runtime Error",
                    pos_start,
                    pos_end,
                    "Too many args supplied!",
                ));
            }

            return res.success(fun(eval_args));
        }
        res.success(Value::Null)
    }

    pub fn get_obj_prop_val(self, k: String) -> RuntimeResult {
        let mut res = RuntimeResult::new();
        match self {
            Self::Object {
                properties,
                pos_start,
                pos_end,
            } => {
                if !properties.contains_key(&k) {
                    return res.failure(Error::new(
                        "Runtime Error",
                        pos_start,
                        pos_end,
                        "No value found!",
                    ));
                };

                res.success(properties.get(&k).unwrap().clone())
            }
            _ => res.failure(Error::new(
                "Runtime Error",
                self.get_pos_start(),
                self.get_pos_end(),
                "Not a object!",
            )),
        }
    }

    pub fn init_class(
        self,
        constructor_params_e: Vec<Value>,
        inter: &mut Interpreter,
    ) -> RuntimeResult {
        let mut res = RuntimeResult::new();
        return if let Self::Class {
            constructor,
            methods,
            properties,
            pos_start,
            pos_end,
            name,
        } = self.clone()
        {
            let ctx = Context::new(name);
            inter.ctx.push(ctx);

            let mut properties_e: HashMap<String, Value> = HashMap::new();

            for (k, v) in &properties {
                let e_v = inter.interpret_node(v.clone());
                if e_v.clone().should_return() {
                    return e_v;
                }
                properties_e.insert(k.to_string(), e_v.val.unwrap());
            }

            let mut methods_e: HashMap<String, Value> = HashMap::new();

            for (k, v) in &methods {
                let e_v = inter.interpret_node(v.clone());
                if e_v.clone().should_return() {
                    return e_v;
                }
                methods_e.insert(k.to_string(), e_v.val.unwrap());
            }

            let mut props = properties_e.clone();
            props.extend(methods_e);

            if constructor.is_some() {
                let e_c = inter.interpret_node(constructor.unwrap());
                if e_c.clone().should_return() {
                    return e_c;
                }

                e_c.val.unwrap().execute(constructor_params_e, inter);
            }

            for (k, v) in props.clone() {
                let n_v = inter
                    .ctx
                    .last_mut()
                    .unwrap()
                    .symbols
                    .get(&k)
                    .unwrap()
                    .value
                    .clone();
                if v != n_v {
                    props.insert(k.to_string(), n_v);
                }
            }

            let soul = Self::Object {
                pos_start,
                pos_end,
                properties: props,
            };

            inter.ctx.pop();

            res.success(soul)
        } else {
            res.success(Value::Null)
        };
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), E> {
        match self {
            Self::Int { val, .. } => write!(f, "{}", val),
            Self::Float { val, .. } => write!(f, "{}", val),
            Self::Char { val, .. } => write!(f, "{}", val),
            Self::String { val, .. } => write!(f, "{}", val),
            Self::Boolean { val, .. } => write!(f, "{}", val),
            Self::Function { name, args, .. } => write!(
                f,
                "fun {}({})",
                name.clone().value.into_string(),
                args.iter()
                    .map(|x| x.clone().value.into_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Self::InBuiltFunction { name, .. } => write!(f, "fun <std.{}>", name),
            Self::Array { elements, .. } => write!(
                f,
                "[{}]",
                elements
                    .iter()
                    .map(|x| format!("{}", x))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Self::Object { properties, .. } => write!(
                f,
                "{}{}{}",
                "{\n",
                properties
                    .iter()
                    .map(|(k, v)| format!("     {}: {}", k, v))
                    .collect::<Vec<String>>()
                    .join(",\n"),
                "\n}"
            ),
            Self::Class { name, .. } => write!(f, "class {}", name),
            Self::Null => write!(f, "Null"),
        }
    }
}
