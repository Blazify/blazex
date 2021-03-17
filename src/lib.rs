#![allow(unused_assignments)]
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
    // LLVM Code Generator
    pub mod llvm {
        pub mod compiler;
        pub mod compiler_result;
    }
}

// Utils
pub mod utils {
    pub mod constants;
    pub mod error;
    pub mod position;
}

use crate::core::lexer::lexer::Lexer;
use crate::core::parser::nodes::Node;
use crate::core::parser::parser::Parser;

pub trait Compile {
    fn from_ast(node: &Node) -> Result<i64, String>;

    fn from_source(name: &'static str, file_content: &'static str) -> Result<i64, String> {
        let is_dev = cfg!(feature = "development");
        let line_decor = "----------";
        let lexed = Lexer::new(name, file_content).tokenize();
        if is_dev {
            println!("{}Lexing{}\n", line_decor, line_decor);
        }

        if lexed.error.is_some() {
            return Err(lexed.error.unwrap().prettify());
        }
        if is_dev {
            for token in lexed.tokens.clone() {
                println!("{}", token);
            }
            println!("\n{}Lexing End{}\n", line_decor, line_decor);
        }

        let parsed = Parser::new(lexed.tokens.clone()).parse();
        if is_dev {
            println!("\n{}Parsing{}\n", line_decor, line_decor);
        }

        if parsed.error.is_some() || parsed.node.is_none() {
            return Err(parsed.error.unwrap().prettify());
        }

        if is_dev {
            println!("{}", parsed.clone().node.unwrap());
            println!("\n{}Parsing End{}\n", line_decor, line_decor);
        }
        Self::from_ast(&parsed.node.unwrap())
    }
}
