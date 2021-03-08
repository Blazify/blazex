#![allow(unused_assignments)]

// Utils
pub mod utils {
    pub mod constants;
    pub mod error;
    pub mod position;
}

// Core
pub mod core {
    // Token
    pub mod token;
    // Lexer
    pub mod lexer {
        pub mod lexer;
        pub mod lexer_method_result;
        pub mod lexer_result;
    }
    // Nodes
    pub mod nodes {
        pub mod binary_op_node;
        pub mod boolean_node;
        pub mod call_node;
        pub mod char_node;
        pub mod for_node;
        pub mod fun_def;
        pub mod if_node;
        pub mod number_node;
        pub mod string_node;
        pub mod unary_node;
        pub mod var_access_node;
        pub mod var_assign_node;
        pub mod var_reassign_node;
        pub mod while_node;
    }
    // Parser
    pub mod parser {
        pub mod parser;
        pub mod parser_result;
    }
}

use crate::core::lexer::lexer::Lexer;
use crate::core::parser::parser::Parser;

use crate::utils::constants::{DynType, Tokens};

fn main() {
    let lexed = Lexer::new("eval.bzs", "69420").tokenize();
    if lexed.error.is_some() {
        println!("{}", lexed.error.unwrap().prettify());
        return;
    }

    let parsed = Parser::new(lexed.tokens.clone()).parse();
    if parsed.error.is_some() || parsed.node.is_none() {
        println!("{}", parsed.error.unwrap().prettify());
        return;
    }
    for token in lexed.tokens {
        print!("[{:?}]", token.r#type);
        match token.value {
            DynType::Int(i) => {
                if token.r#type == Tokens::Int {
                    println!(": {}", i);
                } else {
                    println!();
                }
            }
            DynType::Float(f) => println!(": {}", f),
            DynType::String(s) => println!(": {}", s),
            DynType::Boolean(b) => println!(": {}", b),
            DynType::Char(c) => println!(": {}", c),
        };
    }
    println!("\nParsed:\n{:?}", parsed.node);
}
