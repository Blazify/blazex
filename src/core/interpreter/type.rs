use crate::core::interpreter::runtime_result::RuntimeResult;
use crate::core::token::Token;
use crate::utils::constants::DynType;
use crate::utils::{constants::Tokens, context::Context, error::Error, position::Position};
use std::ops::Neg;

#[derive(Debug, Clone)]
pub enum Type {
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
}

impl Type {
    pub fn is_true(self) -> bool {
        match self.clone() {
            Self::Boolean { val, .. } => val,
            Self::Int { val, .. } => val != 0,
            Self::Float { val, .. } => val != 0.0,
            _ => panic!("Not bool convertible"),
        }
    }

    pub fn get_pos_start(self) -> Position {
        match self {
            Type::Int { pos_start, .. } => pos_start,
            Type::Float { pos_start, .. } => pos_start,
            Type::String { pos_start, .. } => pos_start,
            Type::Char { pos_start, .. } => pos_start,
            Type::Boolean { pos_start, .. } => pos_start,
        }
    }

    pub fn get_pos_end(self) -> Position {
        match self {
            Type::Int { pos_end, .. } => pos_end,
            Type::Float { pos_end, .. } => pos_end,
            Type::String { pos_end, .. } => pos_end,
            Type::Char { pos_end, .. } => pos_end,
            Type::Boolean { pos_end, .. } => pos_end,
        }
    }

    pub fn get_ctx(self) -> Context {
        match self {
            Type::Int { ctx, .. } => ctx,
            Type::Float { ctx, .. } => ctx,
            Type::String { ctx, .. } => ctx,
            Type::Char { ctx, .. } => ctx,
            Type::Boolean { ctx, .. } => ctx,
        }
    }

    pub fn op(self, n: Type, op: Token) -> RuntimeResult {
        match self.clone() {
            Type::Int {
                val: v,
                pos_start,
                ctx,
                ..
            } => {
                if let Type::Int {
                    val: v1, pos_end, ..
                } = n
                {
                    return match op.r#type {
                        Tokens::Plus => RuntimeResult::new().success(Type::Int {
                            val: v + v1,
                            ctx,
                            pos_start,
                            pos_end,
                        }),
                        Tokens::Minus => RuntimeResult::new().success(Type::Int {
                            val: v - v1,
                            ctx,
                            pos_start,
                            pos_end,
                        }),
                        Tokens::Multiply => RuntimeResult::new().success(Type::Int {
                            val: v * v1,
                            ctx,
                            pos_start,
                            pos_end,
                        }),
                        Tokens::Divide => RuntimeResult::new().success(Type::Int {
                            val: v / v1,
                            ctx,
                            pos_start,
                            pos_end,
                        }),
                        Tokens::DoubleEquals => RuntimeResult::new().success(Type::Boolean {
                            val: v == v1,
                            pos_start,
                            pos_end,
                            ctx,
                        }),
                        Tokens::NotEquals => RuntimeResult::new().success(Type::Boolean {
                            val: v == v1,
                            pos_start,
                            pos_end,
                            ctx,
                        }),
                        Tokens::LessThan => RuntimeResult::new().success(Type::Boolean {
                            val: v < v1,
                            pos_start,
                            pos_end,
                            ctx,
                        }),
                        Tokens::LessThanEquals => RuntimeResult::new().success(Type::Boolean {
                            val: v <= v1,
                            pos_start,
                            pos_end,
                            ctx,
                        }),
                        Tokens::GreaterThan => RuntimeResult::new().success(Type::Boolean {
                            val: v > v1,
                            pos_start,
                            pos_end,
                            ctx,
                        }),
                        Tokens::GreaterThanEquals => RuntimeResult::new().success(Type::Boolean {
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
            Type::Float {
                val: v,
                pos_start,
                ctx,
                ..
            } => {
                if let Type::Float {
                    val: v1, pos_end, ..
                } = n
                {
                    return match op.r#type {
                        Tokens::Plus => RuntimeResult::new().success(Type::Float {
                            val: v + v1,
                            ctx,
                            pos_start,
                            pos_end,
                        }),
                        Tokens::Minus => RuntimeResult::new().success(Type::Float {
                            val: v - v1,
                            ctx,
                            pos_start,
                            pos_end,
                        }),
                        Tokens::Multiply => RuntimeResult::new().success(Type::Float {
                            val: v * v1,
                            ctx,
                            pos_start,
                            pos_end,
                        }),
                        Tokens::Divide => RuntimeResult::new().success(Type::Float {
                            val: v / v1,
                            ctx,
                            pos_start,
                            pos_end,
                        }),
                        Tokens::DoubleEquals => RuntimeResult::new().success(Type::Boolean {
                            val: v == v1,
                            pos_start,
                            pos_end,
                            ctx,
                        }),
                        Tokens::NotEquals => RuntimeResult::new().success(Type::Boolean {
                            val: v == v1,
                            pos_start,
                            pos_end,
                            ctx,
                        }),
                        Tokens::LessThan => RuntimeResult::new().success(Type::Boolean {
                            val: v < v1,
                            pos_start,
                            pos_end,
                            ctx,
                        }),
                        Tokens::LessThanEquals => RuntimeResult::new().success(Type::Boolean {
                            val: v <= v1,
                            pos_start,
                            pos_end,
                            ctx,
                        }),
                        Tokens::GreaterThan => RuntimeResult::new().success(Type::Boolean {
                            val: v > v1,
                            pos_start,
                            pos_end,
                            ctx,
                        }),
                        Tokens::GreaterThanEquals => RuntimeResult::new().success(Type::Boolean {
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
            Type::Boolean {
                val,
                ctx,
                pos_start,
                ..
            } => {
                if let Type::Boolean {
                    val: v1, pos_end, ..
                } = n
                {
                    if op
                        .clone()
                        .matches(Tokens::Keyword, DynType::String("and".to_string()))
                    {
                        return RuntimeResult::new().success(Type::Boolean {
                            val: val && v1,
                            pos_start,
                            pos_end,
                            ctx,
                        });
                    } else if op
                        .clone()
                        .matches(Tokens::Keyword, DynType::String("or".to_string()))
                    {
                        return RuntimeResult::new().success(Type::Boolean {
                            val: val || v1,
                            pos_start,
                            pos_end,
                            ctx,
                        });
                    }

                    match op.r#type {
                        Tokens::DoubleEquals => RuntimeResult::new().success(Type::Boolean {
                            val: val == v1,
                            pos_start,
                            pos_end,
                            ctx,
                        }),
                        Tokens::NotEquals => RuntimeResult::new().success(Type::Boolean {
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
                    return RuntimeResult::new().success(Type::Int {
                        val: val.neg(),
                        ctx,
                        pos_end,
                        pos_start,
                    });
                } else if let Tokens::Plus = u {
                    return RuntimeResult::new().success(Type::Int {
                        val: val.abs(),
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
}
