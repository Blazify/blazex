/*
   Copyright 2021 BlazifyOrg

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/
#![allow(unused_assignments)]
#![allow(mutable_borrow_reservation_conflict)]

// Core
pub mod compiler {
    // Token
    pub mod token;
    // Lexer
    pub mod lexer {
        pub mod lexer;
    }
    // Parser
    pub mod parser {
        pub mod nodes;
        pub mod parser;
        pub mod parser_result;
    }
    // Bytecode Generation
    pub mod bytecode {
        pub mod bytecode;
        pub mod opcode;
    }
}

// Virtual Machine
pub mod blazevm {
    pub mod vm;
}

// Utils
pub mod utils {
    pub mod constants;
    pub mod error;
    pub mod position;
}

use crate::compiler::lexer::lexer::Lexer;
use crate::compiler::parser::nodes::Node;
use crate::compiler::parser::parser::Parser;
use ::std::process::exit;

pub trait LanguageServer {
    type Result;
    fn from_ast(node: Node) -> Self::Result;

    fn from_source(name: &'static str, file_content: &'static str) -> Self::Result {
        let lexed = Lexer::new(name, file_content).tokenize();
        let mut tokens = vec![];
        match lexed {
            Ok(tokens_) => {
                tokens = tokens_;
            }
            Err(error) => {
                println!("{}", error.prettify());
                exit(1);
            }
        }

        let parsed = Parser::new(tokens).parse();
        if parsed.error.is_some() || parsed.node.is_none() {
            println!("{}", parsed.error.unwrap().prettify());
            exit(1);
        }

        Self::from_ast(parsed.node.unwrap())
    }
}
