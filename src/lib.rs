#![allow(unused_assignments)]
#![allow(mutable_borrow_reservation_conflict)]
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
    // Parser
    pub mod parser {
        pub mod nodes;
        pub mod parser;
        pub mod parser_result;
    }
    // Interpreter
    pub mod interpreter {
        pub mod interpreter;
        pub mod runtime_result;
        pub mod value;
    }
    // Bytecode Generation
    pub mod bytecode {
        pub mod bytecode;
        pub mod opcode;
    }
    // VM
    pub mod vm {
        pub mod vm;
    }
}

// Utils
pub mod utils {
    pub mod constants;
    pub mod context;
    pub mod error;
    pub mod position;
    pub mod symbol;
}

pub mod std {
    pub mod lib;
}

use crate::core::lexer::lexer::Lexer;
use crate::core::parser::nodes::Node;
use crate::core::parser::parser::Parser;
use ::std::process::exit;

pub trait LanguageServer {
    type Result;
    fn from_ast(name: &'static str, node: Node) -> Self::Result;

    fn from_source(name: &'static str, file_content: &'static str) -> Self::Result {
        let lexed = Lexer::new(name, file_content).tokenize();
        if lexed.error.is_some() {
            println!("{}", lexed.error.unwrap().prettify());
            exit(1);
        }

        let parsed = Parser::new(lexed.tokens).parse();
        if parsed.error.is_some() || parsed.node.is_none() {
            println!("{}", parsed.error.unwrap().prettify());
            exit(1);
        }

        Self::from_ast(name, parsed.node.unwrap())
    }
}
