use crate::core::interpreter::value::Value;
use crate::utils::context::Context;
use crate::utils::position::Position;
use crate::utils::symbol::Symbol;
use std::convert::TryInto;
use std::process::exit;

pub fn init_std(ctx: &mut Context) {
    let pos = Position::new(-1, -1, -1, "<std>", "");

    std_add_func(
        "println",
        |v: Vec<Value>| -> Value {
            println!("{}", v.get(0).unwrap());
            Value::Null
        },
        1,
        pos,
        ctx,
    );

    std_add_func(
        "print",
        |v: Vec<Value>| -> Value {
            print!("{}", v.get(0).unwrap());
            Value::Null
        },
        1,
        pos,
        ctx,
    );

    std_add_func(
        "error",
        |v: Vec<Value>| -> Value {
            eprintln!("{}", v.get(0).unwrap());
            exit(1);
        },
        1,
        pos,
        ctx,
    );

    std_add_func(
        "exit",
        |v: Vec<Value>| -> Value {
            exit(v.get(0).unwrap().clone().get_int().try_into().unwrap());
        },
        1,
        pos,
        ctx,
    );
}

fn std_add_func(
    name: &'static str,
    fun: fn(Vec<Value>) -> Value,
    args_len: usize,
    pos: Position,
    ctx: &mut Context,
) {
    ctx.symbols.insert(
        name.to_string(),
        Symbol::new(
            &Value::InBuiltFunction {
                name: name.to_string(),
                pos_start: pos,
                pos_end: pos,
                args_len,
                fun,
            },
            false,
        ),
    );
}
