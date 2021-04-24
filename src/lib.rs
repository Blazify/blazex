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
    // Interpreter
    pub mod interpreter {
        pub mod interpreter;
        pub mod runtime_result;
        pub mod r#type;
    }
}

// Utils
pub mod utils {
    pub mod constants;
    pub mod context;
    pub mod error;
    pub mod position;
    pub mod symbol;
    pub mod symbol_table;
}

use crate::core::lexer::lexer::Lexer;
use crate::core::parser::nodes::Node;
use crate::core::parser::parser::Parser;
use crate::utils::context::Context;

pub trait Interpret {
    fn from_ast(node: &Node, ctx: &mut Context) -> Result<String, String>;

    fn from_source(
        name: &'static str,
        file_content: &'static str,
        ctx: &mut Context,
    ) -> Result<String, String> {
        let lexed = Lexer::new(name, file_content).tokenize();
        if lexed.error.is_some() {
            return Err(lexed.error.unwrap().prettify());
        }

        let parsed = Parser::new(lexed.tokens.clone()).parse();
        if parsed.error.is_some() || parsed.node.is_none() {
            return Err(parsed.error.unwrap().prettify());
        }

        Self::from_ast(&parsed.node.unwrap(), ctx)
    }
}
