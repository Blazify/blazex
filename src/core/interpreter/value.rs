use crate::core::interpreter::{interpreter::Interpreter, runtime_result::RuntimeResult};
use crate::core::parser::nodes::Node;
use crate::core::token::Token;
use crate::utils::constants::DynType;
use crate::utils::{
    constants::Tokens, context::Context, error::Error, position::Position, symbol::Symbol,
    symbol_table::SymbolTable,
};
use std::collections::HashMap;
use std::convert::TryInto;
use std::fmt::{Display, Error as E, Formatter};

#[derive(Debug, Clone)]
pub enum Value {
    Int {
        val: i64,
        pos_start: Position,
        pos_end: Position,
        ctx: Context,
    },
    Float {
        val: f32,
        pos_start: Position,
        pos_end: Position,
        ctx: Context,
    },
    String {
        val: String,
        pos_start: Position,
        pos_end: Position,
        ctx: Context,
    },
    Char {
        val: char,
        pos_start: Position,
        pos_end: Position,
        ctx: Context,
    },
    Boolean {
        val: bool,
        pos_start: Position,
        pos_end: Position,
        ctx: Context,
    },
    Function {
        name: Token,
        body_node: Node,
        args: Vec<Token>,
        pos_start: Position,
        pos_end: Position,
        ctx: Context,
    },
    Array {
        elements: Vec<Value>,
        pos_start: Position,
        pos_end: Position,
        ctx: Context,
    },
    Object {
        properties: HashMap<String, Value>,
        pos_start: Position,
        pos_end: Position,
        ctx: Context,
    },
    Class {
        name: String,
        constructor: Box<Option<Value>>,
        properties: HashMap<String, Value>,
        methods: HashMap<String, Value>,
        pos_start: Position,
        pos_end: Position,
        ctx: Context,
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

    pub fn get_ctx(self) -> Context {
        match self {
            Value::Int { ctx, .. } => ctx,
            Value::Float { ctx, .. } => ctx,
            Value::String { ctx, .. } => ctx,
            Value::Char { ctx, .. } => ctx,
            Value::Boolean { ctx, .. } => ctx,
            Value::Function { ctx, .. } => ctx,
            Value::Array { ctx, .. } => ctx,
            Value::Object { ctx, .. } => ctx,
            Value::Class { ctx, .. } => ctx,
            _ => panic!(),
        }
    }

    pub fn op(self, n: Value, op: Token) -> RuntimeResult {
        match self.clone() {
            Value::Int {
                val: v,
                pos_start,
                ctx,
                ..
            } => {
                if let Value::Int {
                    val: v1, pos_end, ..
                } = n
                {
                    return match op.r#type {
                        Tokens::Plus => RuntimeResult::new().success(Value::Int {
                            val: v + v1,
                            ctx,
                            pos_start,
                            pos_end,
                        }),
                        Tokens::Minus => RuntimeResult::new().success(Value::Int {
                            val: v - v1,
                            ctx,
                            pos_start,
                            pos_end,
                        }),
                        Tokens::Multiply => RuntimeResult::new().success(Value::Int {
                            val: v * v1,
                            ctx,
                            pos_start,
                            pos_end,
                        }),
                        Tokens::Divide => RuntimeResult::new().success(Value::Int {
                            val: v / v1,
                            ctx,
                            pos_start,
                            pos_end,
                        }),
                        Tokens::Power => RuntimeResult::new().success(Value::Int {
                            val: v.pow(v1.try_into().unwrap()),
                            pos_start,
                            pos_end,
                            ctx,
                        }),
                        Tokens::DoubleEquals => RuntimeResult::new().success(Value::Boolean {
                            val: v == v1,
                            pos_start,
                            pos_end,
                            ctx,
                        }),
                        Tokens::NotEquals => RuntimeResult::new().success(Value::Boolean {
                            val: v != v1,
                            pos_start,
                            pos_end,
                            ctx,
                        }),
                        Tokens::LessThan => RuntimeResult::new().success(Value::Boolean {
                            val: v < v1,
                            pos_start,
                            pos_end,
                            ctx,
                        }),
                        Tokens::LessThanEquals => RuntimeResult::new().success(Value::Boolean {
                            val: v <= v1,
                            pos_start,
                            pos_end,
                            ctx,
                        }),
                        Tokens::GreaterThan => RuntimeResult::new().success(Value::Boolean {
                            val: v > v1,
                            pos_start,
                            pos_end,
                            ctx,
                        }),
                        Tokens::GreaterThanEquals => RuntimeResult::new().success(Value::Boolean {
                            val: v >= v1,
                            pos_start,
                            pos_end,
                            ctx,
                        }),
                        _ => RuntimeResult::new().failure(
                            Error::new(
                                "Runtime Error",
                                pos_start,
                                self.clone().get_pos_end(),
                                "Unexpected token",
                            )
                            .set_ctx(ctx),
                        ),
                    };
                }
                RuntimeResult::new().failure(
                    Error::new(
                        "Runtime Error",
                        pos_start,
                        self.clone().get_pos_end(),
                        "Unexpected type",
                    )
                    .set_ctx(ctx),
                )
            }
            Value::Float {
                val: v,
                pos_start,
                ctx,
                ..
            } => {
                if let Value::Float {
                    val: v1, pos_end, ..
                } = n
                {
                    return match op.r#type {
                        Tokens::Plus => RuntimeResult::new().success(Value::Float {
                            val: v + v1,
                            ctx,
                            pos_start,
                            pos_end,
                        }),
                        Tokens::Minus => RuntimeResult::new().success(Value::Float {
                            val: v - v1,
                            ctx,
                            pos_start,
                            pos_end,
                        }),
                        Tokens::Multiply => RuntimeResult::new().success(Value::Float {
                            val: v * v1,
                            ctx,
                            pos_start,
                            pos_end,
                        }),
                        Tokens::Divide => RuntimeResult::new().success(Value::Float {
                            val: v / v1,
                            ctx,
                            pos_start,
                            pos_end,
                        }),
                        Tokens::Power => RuntimeResult::new().success(Value::Float {
                            val: v.powf(v1),
                            pos_start,
                            pos_end,
                            ctx,
                        }),
                        Tokens::DoubleEquals => RuntimeResult::new().success(Value::Boolean {
                            val: v == v1,
                            pos_start,
                            pos_end,
                            ctx,
                        }),
                        Tokens::NotEquals => RuntimeResult::new().success(Value::Boolean {
                            val: v != v1,
                            pos_start,
                            pos_end,
                            ctx,
                        }),
                        Tokens::LessThan => RuntimeResult::new().success(Value::Boolean {
                            val: v < v1,
                            pos_start,
                            pos_end,
                            ctx,
                        }),
                        Tokens::LessThanEquals => RuntimeResult::new().success(Value::Boolean {
                            val: v <= v1,
                            pos_start,
                            pos_end,
                            ctx,
                        }),
                        Tokens::GreaterThan => RuntimeResult::new().success(Value::Boolean {
                            val: v > v1,
                            pos_start,
                            pos_end,
                            ctx,
                        }),
                        Tokens::GreaterThanEquals => RuntimeResult::new().success(Value::Boolean {
                            val: v >= v1,
                            pos_start,
                            pos_end,
                            ctx,
                        }),
                        _ => RuntimeResult::new().failure(
                            Error::new(
                                "Runtime Error",
                                pos_start,
                                self.clone().get_pos_end(),
                                "Unexpected token",
                            )
                            .set_ctx(ctx),
                        ),
                    };
                }
                RuntimeResult::new().failure(
                    Error::new(
                        "Runtime Error",
                        pos_start,
                        self.clone().get_pos_end(),
                        "Unexpected type",
                    )
                    .set_ctx(ctx),
                )
            }
            Value::Boolean {
                val,
                ctx,
                pos_start,
                ..
            } => {
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
                            ctx,
                        });
                    } else if op
                        .clone()
                        .matches(Tokens::Keyword, DynType::String("or".to_string()))
                    {
                        return RuntimeResult::new().success(Value::Boolean {
                            val: val || v1,
                            pos_start,
                            pos_end,
                            ctx,
                        });
                    }

                    match op.r#type {
                        Tokens::DoubleEquals => RuntimeResult::new().success(Value::Boolean {
                            val: val == v1,
                            pos_start,
                            pos_end,
                            ctx,
                        }),
                        Tokens::NotEquals => RuntimeResult::new().success(Value::Boolean {
                            val: val == v1,
                            pos_start,
                            pos_end,
                            ctx,
                        }),
                        _ => RuntimeResult::new().failure(
                            Error::new(
                                "Runtime Error",
                                self.clone().get_pos_start(),
                                self.clone().get_pos_end(),
                                "Unexpected type",
                            )
                            .set_ctx(self.clone().get_ctx()),
                        ),
                    };
                }
                RuntimeResult::new().failure(
                    Error::new(
                        "Runtime Error",
                        self.clone().get_pos_start(),
                        self.clone().get_pos_end(),
                        "Unexpected type",
                    )
                    .set_ctx(self.clone().get_ctx()),
                )
            }
            _ => RuntimeResult::new().failure(
                Error::new(
                    "Runtime Error",
                    self.clone().get_pos_start(),
                    self.clone().get_pos_end(),
                    "Unexpected type",
                )
                .set_ctx(self.clone().get_ctx()),
            ),
        }
    }

    pub fn unary(self, u: Tokens) -> RuntimeResult {
        match self.clone() {
            Self::Int {
                val,
                ctx,
                pos_end,
                pos_start,
            } => {
                if let Tokens::Minus = u {
                    return RuntimeResult::new().success(Value::Int {
                        val: val * (-1),
                        ctx,
                        pos_end,
                        pos_start,
                    });
                } else if let Tokens::Plus = u {
                    return RuntimeResult::new().success(Value::Int {
                        val: val * (1),
                        ctx,
                        pos_end,
                        pos_start,
                    });
                }

                RuntimeResult::new().failure(
                    Error::new("Runtime Error", pos_start, pos_end, "Unexpected token")
                        .set_ctx(self.clone().get_ctx()),
                )
            }
            _ => RuntimeResult::new().failure(
                Error::new(
                    "Runtime Error",
                    self.clone().get_pos_start(),
                    self.clone().get_pos_end(),
                    "Unexpected type",
                )
                .set_ctx(self.clone().get_ctx()),
            ),
        }
    }

    pub fn execute(self, eval_args: Vec<Value>) -> RuntimeResult {
        let mut res = RuntimeResult::new();
        if let Self::Function {
            args,
            body_node,
            name,
            pos_start,
            pos_end,
            ctx: ctx_,
        } = self
        {
            let parent = ctx_;
            let mut ctx = Context::new(
                name.value.into_string(),
                SymbolTable::new(Some(Box::new(parent.clone().symbol_table))),
                Box::new(Some(parent.clone())),
                Some(pos_start.clone()),
            );

            if args.len() > eval_args.len() {
                return res.failure(
                    Error::new(
                        "Runtime Error",
                        pos_start,
                        pos_end,
                        "Too less args supplied!",
                    )
                    .set_ctx(ctx.clone()),
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
                    .set_ctx(ctx.clone()),
                );
            }

            for x in 0..args.len() {
                let name = &args[x];
                let val = &eval_args[x];

                ctx.symbol_table = ctx.clone().symbol_table.set(
                    name.clone().value.into_string(),
                    Symbol::new(val.clone(), true),
                );
            }
            let result = Interpreter::interpret_node(body_node, &mut ctx);
            if result.clone().should_return() {
                return result;
            }
            return res.success(result.val.unwrap());
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
                ctx,
            } => {
                if !properties.contains_key(&k) {
                    return res.failure(
                        Error::new("Runtime Error", pos_start, pos_end, "No value found!")
                            .set_ctx(ctx.clone()),
                    );
                };

                res.success(properties.get(&k).unwrap().clone())
            }
            _ => res.failure(
                Error::new(
                    "Runtime Error",
                    self.get_pos_start(),
                    self.get_pos_end(),
                    "Not a object!",
                )
                .set_ctx(self.get_ctx().clone()),
            ),
        }
    }

    pub fn init_class(self, constructor_params_e: Vec<Value>) -> RuntimeResult {
        let mut res = RuntimeResult::new();
        return if let Self::Class {
            constructor,
            methods,
            properties: prop,
            ref mut ctx,
            pos_start,
            pos_end,
            ..
        } = self.clone()
        {
            if constructor.is_some() {
                let e_c = constructor.unwrap().execute(constructor_params_e);
                if e_c.clone().should_return() && e_c.return_val != true {
                    return e_c;
                }
            }
            let mut properties = prop.clone();
            properties.extend(methods);
            let soul = Self::Object {
                ctx: ctx.clone(),
                pos_start,
                pos_end,
                properties,
            };

            res.success(soul)
        } else {
            res.success(Value::Null)
        };
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), E> {
        match self {
            Self::Int { val, .. } => write!(f, "Int: {}", val),
            Self::Float { val, .. } => write!(f, "Float: {}", val),
            Self::Char { val, .. } => write!(f, "Char: {}", val),
            Self::String { val, .. } => write!(f, "String: {}", val),
            Self::Boolean { val, .. } => write!(f, "Boolean: {}", val),
            Self::Function { name, args, .. } => write!(
                f,
                "fun {}({})",
                name.clone().value.into_string(),
                args.iter()
                    .map(|x| x.clone().value.into_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
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
            Self::Class {
                name,
                properties,
                methods,
                constructor,
                ..
            } => write!(
                f,
                "class {} {}constructor: {}\n{}{}{}",
                name,
                "{\n     ",
                if constructor.is_some() {
                    constructor.as_ref().as_ref().unwrap()
                } else {
                    &Value::Null
                },
                properties
                    .iter()
                    .map(|(k, v)| format!("     {}: {}", k, v))
                    .collect::<Vec<String>>()
                    .join(",\n"),
                methods
                    .iter()
                    .map(|(k, v)| format!("     {}: {}", k, v))
                    .collect::<Vec<String>>()
                    .join(",\n"),
                "\n}"
            ),
            Self::Null => write!(f, "Null"),
        }
    }
}
