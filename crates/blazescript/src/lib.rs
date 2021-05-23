#![feature(test)]
#![allow(dead_code)]
use blaze_vm::{Konstants, VM};
use bzsc_bytecode::ByteCodeGen;
use bzsc_lexer::Lexer;
use bzsc_parser::parser::Parser;
use std::collections::HashMap;
use std::process::exit;

pub fn run_program(name: String, program: String) {
    let lexed = Lexer::new(
        Box::leak(name.to_owned().into_boxed_str()),
        Box::leak(program.to_owned().into_boxed_str()),
    )
    .lex();

    let mut tokens = vec![];
    match lexed {
        Ok(tok) => tokens.extend(tok),
        Err(e) => {
            e.prettify();
            exit(1);
        }
    }

    let parsed = Parser::new(tokens).parse();
    if parsed.error.is_some() {
        parsed.error.unwrap().prettify();
        exit(1);
    }

    let mut bytecode = ByteCodeGen::new();
    bytecode.compile_node(parsed.node.unwrap());

    let mut vm = VM::new(bytecode.bytecode, None);
    vm.run();

    let mut var = HashMap::new();
    for (k, v) in &bytecode.variables {
        var.insert(v.clone(), k.clone());
    }
}

/*
* Print Prettified Version of Result
*/
pub fn format_print(k: &Konstants, props: HashMap<u16, String>) -> String {
    match k {
        Konstants::None => {
            panic!("Unexpected `None`")
        }
        Konstants::Null => {
            format!("Null")
        }
        Konstants::Int(i) => {
            format!("{}", i)
        }
        Konstants::Float(i) => {
            format!("{}", i)
        }
        Konstants::String(i) => {
            format!("{}", i)
        }
        Konstants::Char(i) => {
            format!("{}", i)
        }
        Konstants::Boolean(i) => {
            format!("{}", i)
        }
        Konstants::Array(x_arr) => {
            let mut res = vec![];
            for x in &x_arr[..] {
                res.push(format_print(x, props.clone()));
            }
            let a = res.join(", ");
            format!("[{}]", a)
        }
        Konstants::Object(x) => {
            let mut str = String::from("{\n    ");
            for (a, b) in x {
                str.push_str(
                    format!(
                        "{}: {},\n",
                        props.get(&(*a as u16)).unwrap(),
                        format_print(b, props.clone())
                    )
                    .as_str(),
                );
                str.push_str("    ");
            }
            str.push_str("\r}");
            str
        }
        Konstants::Function(x, _) | Konstants::Constructor(x, _) => {
            let mut str = String::from("Function<(");
            let mut arr = vec![];
            for a in x {
                arr.push(props.get(a).unwrap().clone());
            }
            str.push_str(arr.join(", ").as_str());
            str.push(')');
            str.push('>');
            str
        }
    }
}
