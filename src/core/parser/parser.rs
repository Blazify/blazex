use crate::core::parser::nodes::Node;
use crate::core::parser::parser_result::ParseResult;
use crate::core::token::Token;
use crate::utils::constants::{DynType, Tokens};
use crate::utils::error::Error;

#[derive(Debug, Clone)]
pub struct Parser {
    pub tokens: Vec<Token>,
    pub token_index: usize,
    pub current_token: Token,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        let current_token = tokens.clone()[0].clone();
        Parser {
            tokens,
            token_index: 0,
            current_token,
        }
    }

    fn advance(&mut self) -> Token {
        self.token_index += 1;
        self.update_current_token();
        self.current_token.clone()
    }

    fn update_current_token(&mut self) {
        if self.token_index >= 0 as usize && self.token_index < self.tokens.len() {
            self.current_token = self.tokens.clone()[self.clone().token_index].clone();
        }
    }

    pub fn parse(&mut self) -> ParseResult {
        let mut res = self.statements();
        if res.error.is_none() && self.current_token.r#type != Tokens::EOF {
            println!("{:?}", self.current_token.r#type);
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected Operators, Variables, Functions, etc but found none",
            ));
        }
        res
    }

    fn reverse(&mut self, cnt: usize) -> Token {
        self.token_index -= cnt;
        self.update_current_token();

        self.clone().current_token
    }

    fn statements(&mut self) -> ParseResult {
        let mut res = ParseResult::new();
        let mut statements: Vec<Node> = vec![];

        while self.current_token.r#type == Tokens::Newline {
            res.register_advancement();
            self.advance();
        }

        let mut statement = res.register(self.statement());
        if res.error.is_some() {
            return res;
        };
        statements.push(statement.unwrap());
        let mut more_statements = true;

        loop {
            let mut newline_ct = 0;
            while self.current_token.r#type == Tokens::Newline {
                res.register_advancement();
                self.advance();
                newline_ct += 1;
            }

            if newline_ct == 0 {
                more_statements = false;
            }

            if !more_statements {
                break;
            }
            statement = res.try_register(self.statement());
            if statement.is_none() {
                self.reverse(res.to_reverse_count as usize);
                more_statements = false;
                continue;
            }
            statements.push(statement.unwrap())
        }
        res.success(Node::ArrayNode {
            element_nodes: statements,
            pos_start: self.current_token.pos_start,
            pos_end: self.current_token.pos_end,
        })
    }

    fn statement(&mut self) -> ParseResult {
        let mut res = ParseResult::new();
        let pos_start = self.current_token.clone().pos_start;

        if self
            .clone()
            .current_token
            .matches(Tokens::Keyword, DynType::String("return".to_string()))
        {
            res.register_advancement();
            self.advance();

            let expr = res.try_register(self.expr());
            if expr.is_none() {
                self.reverse(res.to_reverse_count as usize);
            }

            return res.success(Node::ReturnNode {
                value: Box::new(expr),
                pos_start,
                pos_end: self.current_token.clone().pos_end,
            });
        }

        let expr = res.register(self.expr());
        if res.error.is_some() {
            return res.failure(Error::new(
                "Invalid Syntax",
                pos_start,
                self.current_token.pos_end.clone(),
                "Expected keywords, variables, etc",
            ));
        }
        res.success(expr.unwrap())
    }

    fn expr(&mut self) -> ParseResult {
        let mut res = ParseResult::new();
        if self
            .current_token
            .clone()
            .matches(Tokens::Keyword, DynType::String(String::from("val")))
            || self
                .current_token
                .clone()
                .matches(Tokens::Keyword, DynType::String(String::from("var")))
        {
            let var_type: String;
            match self.current_token.value.clone() {
                DynType::String(value) => var_type = value,
                _ => panic!(),
            };
            res.register_advancement();
            self.advance();

            if self.current_token.r#type != Tokens::Identifier {
                return res.failure(Error::new(
                    "Invalid Syntax Error",
                    self.current_token.pos_start.clone(),
                    self.current_token.pos_end.clone(),
                    "Expected Identifier",
                ));
            }

            let var_name = self.current_token.clone();
            res.register_advancement();
            self.advance();

            if self.current_token.r#type != Tokens::Equals {
                return res.failure(Error::new(
                    "Invalid Syntax",
                    self.current_token.pos_start.clone(),
                    self.current_token.pos_end.clone(),
                    "Expected '='",
                ));
            }

            res.register_advancement();
            self.advance();

            let expr = res.register(self.expr()).unwrap();

            let reassignable = if var_type == String::from("var") {
                true
            } else {
                false
            };
            return res.success(Node::VarAssignNode {
                name: var_name.clone(),
                value: Box::new(expr),
                reassignable,
                pos_start: var_name.pos_start,
                pos_end: self.current_token.clone().pos_end,
            });
        }

        let pos_start = self.current_token.clone().pos_start;
        let mut left = res.register(self.comp_expr());
        if res.error.is_some() {
            return res;
        }

        while self
            .current_token
            .clone()
            .matches(Tokens::Keyword, DynType::String("and".to_string()))
            || self
                .current_token
                .clone()
                .matches(Tokens::Keyword, DynType::String("or".to_string()))
        {
            let op_token = self.current_token.clone();
            res.register_advancement();
            self.advance();
            let right = res.register(self.comp_expr());
            if res.error.is_some() {
                return res;
            }
            left = Option::from(Node::BinOpNode {
                left: Box::new(left.clone().unwrap()),
                right: Box::new(right.clone().unwrap()),
                op_token,
                pos_start: pos_start.clone(),
                pos_end: self.current_token.clone().pos_end,
            });
        }

        if res.error.is_some() {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected 'var', int, float, identifier, '+', '-' or '('",
            ));
        }

        res.success(left.unwrap())
    }

    fn comp_expr(&mut self) -> ParseResult {
        let mut res = ParseResult::new();

        if self
            .current_token
            .clone()
            .matches(Tokens::Keyword, DynType::String("not".to_string()))
        {
            let op_token = self.current_token.clone();
            res.register_advancement();
            self.advance();

            let node = res.register(self.comp_expr());
            if res.error.is_some() {
                return res;
            }

            return res.success(Node::UnaryNode {
                node: Box::new(node.clone().unwrap()),
                op_token: op_token.clone(),
                pos_start: op_token.pos_start,
                pos_end: self.current_token.clone().pos_start,
            });
        }

        let pos_start = self.current_token.clone().pos_start;
        let mut left = res.register(self.arith_expr());
        if res.error.is_some() {
            return res;
        }

        while [
            Tokens::DoubleEquals,
            Tokens::NotEquals,
            Tokens::LessThan,
            Tokens::LessThanEquals,
            Tokens::GreaterThan,
            Tokens::GreaterThanEquals,
        ]
        .contains(&self.current_token.r#type)
        {
            let op_token = self.current_token.clone();
            res.register_advancement();
            self.advance();

            let right = res.register(self.arith_expr());
            if res.error.is_some() {
                return res;
            }
            left = Option::from(Node::BinOpNode {
                left: Box::new(left.clone().unwrap()),
                right: Box::new(right.clone().unwrap()),
                op_token,
                pos_start: pos_start.clone(),
                pos_end: self.current_token.clone().pos_end,
            });
        }

        if res.error.is_some() {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "A Int or Float or Identifier, '+', '-', '(', 'not', '!' was Expected",
            ));
        }
        res.success(left.unwrap())
    }

    fn arith_expr(&mut self) -> ParseResult {
        let mut res = ParseResult::new();

        let pos_start = self.current_token.clone().pos_start;
        let mut left = res.register(self.term());
        if res.error.is_some() {
            return res;
        }

        while [Tokens::Plus, Tokens::Minus].contains(&self.current_token.r#type) {
            let op_token = self.current_token.clone();
            res.register_advancement();
            self.advance();

            let right = res.register(self.term());
            if res.error.is_some() {
                return res;
            }

            left = Option::from(Node::BinOpNode {
                left: Box::new(left.clone().unwrap()),
                right: Box::new(right.clone().unwrap()),
                op_token,
                pos_start,
                pos_end: self.current_token.clone().pos_end,
            });
        }

        res.success(left.unwrap())
    }

    fn term(&mut self) -> ParseResult {
        let mut res = ParseResult::new();

        let pos_start = self.current_token.clone().pos_start;
        let mut left = res.register(self.factor());
        if res.error.is_some() {
            return res;
        }

        while [Tokens::Multiply, Tokens::Divide].contains(&self.current_token.r#type) {
            let op_token = self.current_token.clone();
            res.register_advancement();
            self.advance();

            let right = res.register(self.factor());
            if res.error.is_some() {
                return res;
            }

            left = Option::from(Node::BinOpNode {
                left: Box::new(left.clone().unwrap()),
                right: Box::new(right.clone().unwrap()),
                op_token,
                pos_start,
                pos_end: self.current_token.clone().pos_end,
            });
        }

        res.success(left.unwrap())
    }

    fn factor(&mut self) -> ParseResult {
        let mut res = ParseResult::new();
        let token = self.current_token.clone();

        if [Tokens::Plus, Tokens::Minus].contains(&self.current_token.r#type) {
            res.register_advancement();
            self.advance();
            let factor = res.register(self.factor());
            if res.error.is_some() {
                return res;
            }
            return res.success(Node::UnaryNode {
                op_token: token.clone(),
                node: Box::new(factor.clone().unwrap()),
                pos_start: token.pos_start,
                pos_end: self.current_token.clone().pos_end,
            });
        }
        self.power()
    }

    fn power(&mut self) -> ParseResult {
        let mut res = ParseResult::new();
        let pos_start = self.current_token.clone().pos_start;
        let mut left = res.register(self.call());
        if res.error.is_some() {
            return res;
        }

        while self.current_token.r#type == Tokens::Power {
            let op_token = self.current_token.clone();
            res.register_advancement();
            self.advance();

            let right = res.register(self.factor());
            if res.error.is_some() {
                return res;
            }

            left = Option::from(Node::BinOpNode {
                left: Box::new(left.clone().unwrap()),
                right: Box::new(right.clone().unwrap()),
                op_token,
                pos_start: pos_start.clone(),
                pos_end: self.current_token.clone().pos_end,
            });
        }

        res.success(left.unwrap())
    }

    fn call(&mut self) -> ParseResult {
        let mut res = ParseResult::new();
        let pos_start = self.current_token.clone().pos_start;
        let atom = res.register(self.obj_prop_expr());
        if res.error.is_some() {
            return res;
        }

        if self.current_token.r#type == Tokens::LeftParenthesis {
            let mut arg_nodes: Vec<Node> = vec![];
            res.register_advancement();
            self.advance();

            if self.current_token.r#type == Tokens::RightParenthesis {
                res.register_advancement();
                self.advance();
            } else {
                let expr = res.register(self.expr());
                if res.error.is_some() {
                    return res.failure(Error::new(
                        "Invalid Syntax",
                        self.current_token.pos_start.clone(),
                        self.current_token.pos_end.clone(),
                        "Expected ')', 'var', int, float, identifier, '+', '-' or ','",
                    ));
                }
                arg_nodes.push(expr.unwrap());

                while self.current_token.r#type == Tokens::Comma {
                    res.register_advancement();
                    self.advance();

                    let expr = res.register(self.expr());
                    if res.error.is_some() {
                        return res.failure(Error::new(
                            "Invalid Syntax",
                            self.current_token.pos_start.clone(),
                            self.current_token.pos_end.clone(),
                            "Expected ')', 'var', int, float, identifier, '+', '-' or ','",
                        ));
                    }
                    arg_nodes.push(expr.unwrap());
                }

                if self.current_token.r#type != Tokens::RightParenthesis {
                    return res.failure(Error::new(
                        "Invalid Syntax",
                        self.current_token.pos_start.clone(),
                        self.current_token.pos_end.clone(),
                        "Expected ')' or ','",
                    ));
                }
                res.register_advancement();
                self.advance();
            }
            return res.success(Node::CallNode {
                node_to_call: Box::new(atom.clone().unwrap()),
                args: arg_nodes,
                pos_start: pos_start.clone(),
                pos_end: self.current_token.clone().pos_end,
            });
        } else if self.current_token.r#type == Tokens::Dot {
            self.advance();
            res.register_advancement();

            if self.current_token.r#type != Tokens::Identifier {
                return res.failure(Error::new(
                    "Invalid Syntax",
                    self.current_token.pos_start.clone(),
                    self.current_token.pos_end.clone(),
                    "Expected identifier",
                ));
            }

            let mut id = self.current_token.clone();

            res.register_advancement();
            self.advance();

            let mut l = Node::ObjectPropAccess {
                object: Box::new(atom.clone().unwrap()),
                property: id,
                pos_start,
                pos_end: self.current_token.clone().pos_end,
            };

            while self.current_token.r#type == Tokens::Dot {
                self.advance();
                res.register_advancement();

                if self.current_token.r#type != Tokens::Identifier {
                    return res.failure(Error::new(
                        "Invalid Syntax",
                        self.current_token.pos_start.clone(),
                        self.current_token.pos_end.clone(),
                        "Expected identifier",
                    ));
                }

                id = self.current_token.clone();

                res.register_advancement();
                self.advance();

                l = Node::ObjectPropAccess {
                    object: Box::new(l),
                    property: id,
                    pos_start,
                    pos_end: self.current_token.clone().pos_end,
                };
            }
            return res.success(l);
        }

        res.success(atom.unwrap())
    }

    fn obj_prop_expr(&mut self) -> ParseResult {
        let mut res = ParseResult::new();
        let pos_start = self.current_token.clone().pos_start;
        let atom = res.register(self.atom());
        if res.error.is_some() {
            return res;
        }

        if self.current_token.r#type == Tokens::Dot {
            self.advance();
            res.register_advancement();

            if self.current_token.r#type != Tokens::Identifier {
                return res.failure(Error::new(
                    "Invalid Syntax",
                    self.current_token.pos_start.clone(),
                    self.current_token.pos_end.clone(),
                    "Expected identifier",
                ));
            }

            let mut id = self.current_token.clone();

            res.register_advancement();
            self.advance();

            let mut l = Node::ObjectPropAccess {
                object: Box::new(atom.clone().unwrap()),
                property: id,
                pos_start,
                pos_end: self.current_token.clone().pos_end,
            };

            while self.current_token.r#type == Tokens::Dot {
                self.advance();
                res.register_advancement();

                if self.current_token.r#type != Tokens::Identifier {
                    return res.failure(Error::new(
                        "Invalid Syntax",
                        self.current_token.pos_start.clone(),
                        self.current_token.pos_end.clone(),
                        "Expected identifier",
                    ));
                }

                id = self.current_token.clone();

                res.register_advancement();
                self.advance();

                l = Node::ObjectPropAccess {
                    object: Box::new(l),
                    property: id,
                    pos_start,
                    pos_end: self.current_token.clone().pos_end,
                };
            }
            return res.success(l);
        } else if self.current_token.r#type == Tokens::LeftParenthesis {
            let mut arg_nodes: Vec<Node> = vec![];
            res.register_advancement();
            self.advance();

            if self.current_token.r#type == Tokens::RightParenthesis {
                res.register_advancement();
                self.advance();
            } else {
                let expr = res.register(self.expr());
                if res.error.is_some() {
                    return res.failure(Error::new(
                        "Invalid Syntax",
                        self.current_token.pos_start.clone(),
                        self.current_token.pos_end.clone(),
                        "Expected ')', 'var', int, float, identifier, '+', '-' or ','",
                    ));
                }
                arg_nodes.push(expr.unwrap());

                while self.current_token.r#type == Tokens::Comma {
                    res.register_advancement();
                    self.advance();

                    let expr = res.register(self.expr());
                    if res.error.is_some() {
                        return res.failure(Error::new(
                            "Invalid Syntax",
                            self.current_token.pos_start.clone(),
                            self.current_token.pos_end.clone(),
                            "Expected ')', 'var', int, float, identifier, '+', '-' or ','",
                        ));
                    }
                    arg_nodes.push(expr.unwrap());
                }

                if self.current_token.r#type != Tokens::RightParenthesis {
                    return res.failure(Error::new(
                        "Invalid Syntax",
                        self.current_token.pos_start.clone(),
                        self.current_token.pos_end.clone(),
                        "Expected ')' or ','",
                    ));
                }
                res.register_advancement();
                self.advance();
            }
            return res.success(Node::CallNode {
                node_to_call: Box::new(atom.clone().unwrap()),
                args: arg_nodes,
                pos_start: pos_start.clone(),
                pos_end: self.current_token.clone().pos_end,
            });
        }

        res.success(atom.unwrap())
    }

    fn atom(&mut self) -> ParseResult {
        let mut res = ParseResult::new();
        let token = self.current_token.clone();

        if [Tokens::Int, Tokens::Float].contains(&token.r#type) {
            res.register_advancement();
            self.advance();
            return res.success(Node::NumberNode {
                token: token.clone(),
                pos_start: token.clone().pos_start,
                pos_end: token.clone().pos_end,
            });
        } else if token.r#type == Tokens::Boolean {
            res.register_advancement();
            self.advance();
            return res.success(Node::BooleanNode {
                token: token.clone(),
                pos_start: token.clone().pos_start,
                pos_end: token.clone().pos_end,
            });
        } else if token.r#type == Tokens::String {
            res.register_advancement();
            self.advance();
            return res.success(Node::StringNode {
                token: token.clone(),
                pos_start: token.clone().pos_start,
                pos_end: token.clone().pos_end,
            });
        } else if token.r#type == Tokens::Char {
            res.register_advancement();
            self.advance();
            return res.success(Node::CharNode {
                token: token.clone(),
                pos_start: token.clone().pos_start,
                pos_end: token.clone().pos_end,
            });
        } else if token.r#type == Tokens::Identifier {
            res.register_advancement();
            self.advance();

            if self.current_token.r#type == Tokens::Equals {
                res.register_advancement();
                self.advance();

                let new_value = res.register(self.expr());
                if res.error.is_some() {
                    return res;
                }

                return res.success(Node::VarReassignNode {
                    name: token.clone(),
                    value: Box::new(new_value.clone().unwrap()),
                    pos_start: token.clone().pos_start,
                    pos_end: self.current_token.clone().pos_end,
                });
            }

            return res.success(Node::VarAccessNode {
                token: token.clone(),
                pos_start: token.clone().pos_start,
                pos_end: token.clone().pos_end,
            });
        } else if token.r#type == Tokens::LeftParenthesis {
            res.register_advancement();
            self.advance();
            let expr = res.register(self.expr());
            if res.error.is_some() {
                return res;
            }
            if self.current_token.clone().r#type != Tokens::RightParenthesis {
                return res.failure(Error::new(
                    "Invalid Syntax",
                    self.current_token.clone().pos_start,
                    self.current_token.clone().pos_end,
                    "Expected ')'",
                ));
            }

            res.register_advancement();
            self.advance();
            return res.success(expr.unwrap());
        } else if token.r#type == Tokens::LeftSquareBraces {
            let array_expr = res.register(self.array_expr());
            if res.error.is_some() {
                return res;
            }
            return res.success(array_expr.unwrap());
        } else if token.r#type == Tokens::LeftCurlyBraces {
            let obj_expr = res.register(self.obj_expr());
            if res.error.is_some() {
                return res;
            }
            return res.success(obj_expr.unwrap());
        } else if token
            .clone()
            .matches(Tokens::Keyword, DynType::String("if".to_string()))
        {
            let if_expr = res.register(self.if_expr());
            if res.error.is_some() {
                return res;
            }
            return res.success(if_expr.unwrap());
        } else if token
            .clone()
            .matches(Tokens::Keyword, DynType::String("while".to_string()))
        {
            let while_expr = res.register(self.while_expr());
            if res.error.is_some() {
                return res;
            }
            return res.success(while_expr.unwrap());
        } else if token
            .clone()
            .matches(Tokens::Keyword, DynType::String("for".to_string()))
        {
            let for_expr = res.register(self.for_expr());
            if res.error.is_some() {
                return res;
            }
            return res.success(for_expr.unwrap());
        } else if token
            .clone()
            .matches(Tokens::Keyword, DynType::String("fun".to_string()))
        {
            let fun_def = res.register(self.fun_def());
            if res.error.is_some() {
                return res;
            }
            return res.success(fun_def.unwrap());
        } else if token
            .clone()
            .matches(Tokens::Keyword, DynType::String("class".to_string()))
        {
            let class_def = res.register(self.class_def());
            if res.error.is_some() {
                return res;
            }
            return res.success(class_def.unwrap());
        } else if token
            .clone()
            .matches(Tokens::Keyword, DynType::String("new".to_string()))
        {
            let class_init = res.register(self.class_init());
            if res.error.is_some() {
                return res;
            }
            return res.success(class_init.unwrap());
        } else if token
            .clone()
            .matches(Tokens::Keyword, DynType::String("soul".to_string()))
        {
            self.advance();
            res.register_advancement();

            return res.success(Node::VarAccessNode {
                token: token.clone(),
                pos_start: token.clone().pos_start,
                pos_end: token.clone().pos_end,
            });
        }

        res.failure(Error::new(
            "Invalid Syntax",
            token.pos_start,
            token.pos_end,
            "A Int, Float, String, Char, Keyword, Identifier, '+', '-', '(', etc was Expected",
        ))
    }

    fn obj_expr(&mut self) -> ParseResult {
        let mut res = ParseResult::new();
        let pos_start = self.current_token.clone().pos_start;
        let mut properties: Vec<(Token, Node)> = vec![];

        if self.current_token.r#type != Tokens::LeftCurlyBraces {
            return res.failure(Error::new(
                "Invalid syntax",
                pos_start,
                self.current_token.clone().pos_end,
                "'{' was expected.",
            ));
        }

        self.advance();
        res.register_advancement();

        if self.current_token.r#type == Tokens::Newline {
            res.register_advancement();
            self.advance();
        }

        if self.current_token.r#type == Tokens::RightCurlyBraces {
            res.register_advancement();
            self.advance();
        } else {
            let mut expr = res.register(self.expr());
            if res.error.is_some() {
                return res.failure(Error::new(
                    "Invalid syntax",
                    pos_start,
                    self.current_token.pos_end,
                    "'}', 'key' was expected.",
                ));
            }

            let mut tok;
            if let Node::StringNode { token, .. } = expr.unwrap() {
                tok = token;
            } else {
                return res.failure(Error::new(
                    "Invalid syntax",
                    pos_start,
                    self.current_token.clone().pos_end,
                    "string was expected.",
                ));
            }

            if self.current_token.r#type != Tokens::Colon {
                return res.failure(Error::new(
                    "Invalid syntax",
                    pos_start,
                    self.current_token.clone().pos_end,
                    "':' was expected.",
                ));
            }

            res.register_advancement();
            self.advance();

            expr = res.register(self.expr());
            if res.error.is_some() {
                return res;
            }

            properties.push((tok, expr.unwrap()));

            while self.current_token.r#type == Tokens::Comma {
                self.advance();
                res.register_advancement();

                if self.current_token.r#type == Tokens::Newline {
                    res.register_advancement();
                    self.advance();
                }

                expr = res.register(self.expr());
                if res.error.is_some() {
                    return res.failure(Error::new(
                        "Invalid syntax",
                        pos_start,
                        self.current_token.pos_end,
                        "'}' ',', 'key' was expected.",
                    ));
                }

                if let Node::StringNode { token, .. } = expr.unwrap() {
                    tok = token;
                } else {
                    return res.failure(Error::new(
                        "Invalid syntax",
                        pos_start,
                        self.current_token.clone().pos_end,
                        "string was expected.",
                    ));
                }

                if self.current_token.r#type != Tokens::Colon {
                    return res.failure(Error::new(
                        "Invalid syntax",
                        pos_start,
                        self.current_token.clone().pos_end,
                        "':' was expected.",
                    ));
                }

                res.register_advancement();
                self.advance();

                expr = res.register(self.expr());
                if res.error.is_some() {
                    return res;
                }

                properties.push((tok, expr.unwrap()));
            }

            if self.current_token.r#type == Tokens::Newline {
                self.advance();
                res.register_advancement()
            }

            if self.current_token.r#type != Tokens::RightCurlyBraces {
                return res.failure(Error::new(
                    "Invalid syntax",
                    pos_start,
                    self.current_token.clone().pos_end,
                    "'}', ',' was expected.",
                ));
            }
        }

        res.register_advancement();
        self.advance();

        res.success(Node::ObjectDefNode {
            properties,
            pos_start,
            pos_end: self.current_token.clone().pos_end,
        })
    }

    fn array_expr(&mut self) -> ParseResult {
        let mut res = ParseResult::new();
        let mut element_nodes: Vec<Node> = vec![];
        let token = self.current_token.clone();
        let pos_start = self.current_token.pos_start.clone();

        if self.current_token.r#type != Tokens::LeftSquareBraces {
            return res.failure(Error::new(
                "Invalid syntax",
                pos_start,
                token.pos_end,
                "'[' was expected.",
            ));
        }

        res.register_advancement();
        self.advance();

        if self.current_token.r#type == Tokens::RightSquareBraces {
            res.register_advancement();
            self.advance();
        } else {
            let mut expr = res.register(self.expr());
            if res.error.is_some() {
                return res.failure(Error::new(
                    "Invalid Syntax",
                    pos_start,
                    token.pos_end,
                    "Expected ']', 'var', 'if', 'for', 'while', 'fun', int, float, identifier, '+', '-', '(', '[' or 'NOT'"
                ));
            }

            element_nodes.push(expr.unwrap());
            while self.current_token.r#type == Tokens::Comma {
                res.register_advancement();
                self.advance();

                expr = res.register(self.expr());
                if res.error.is_some() {
                    return res;
                }
                element_nodes.push(expr.unwrap());
            }

            if self.current_token.r#type != Tokens::RightSquareBraces {
                return res.failure(Error::new(
                    "Invalid Syntax",
                    pos_start,
                    token.pos_end,
                    "Expected ']' or ','.",
                ));
            }
            res.register_advancement();
            self.advance();
        }

        res.success(Node::ArrayNode {
            element_nodes,
            pos_start,
            pos_end: token.pos_end,
        })
    }

    fn class_def(&mut self) -> ParseResult {
        let mut res = ParseResult::new();
        let pos_start = self.current_token.clone().pos_start;
        let mut methods: Vec<Node> = vec![];
        let mut properties: Vec<Node> = vec![];
        let mut constructor: Option<Node> = None;

        if !self
            .current_token
            .clone()
            .matches(Tokens::Keyword, DynType::String("class".to_string()))
        {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected 'class'",
            ));
        }

        res.register_advancement();
        self.advance();

        if self.current_token.r#type != Tokens::Identifier {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected identifier",
            ));
        }

        let name = self.current_token.clone();

        res.register_advancement();
        self.advance();

        if self.current_token.r#type != Tokens::LeftCurlyBraces {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected '{'",
            ));
        }

        res.register_advancement();
        self.advance();

        while self.current_token.r#type != Tokens::EOF
            && self.current_token.r#type != Tokens::RightCurlyBraces
        {
            while self.current_token.r#type == Tokens::Newline {
                res.register_advancement();
                self.advance();
            }

            let expr = res.register(self.expr());
            if res.error.is_some() {
                return res;
            }

            let expr_ = expr.clone().unwrap().clone();

            match expr_.clone() {
                Node::FunDef { name, .. } => {
                    if name.is_some() {
                        if constructor.is_some() {
                            return res.failure(Error::new(
                                "Invalid Syntax",
                                self.current_token.pos_start.clone(),
                                self.current_token.pos_end.clone(),
                                "Constructor defined",
                            ));
                        }
                        constructor = Some(expr_);
                    } else {
                        methods.push(expr_)
                    }
                }
                Node::VarAssignNode { .. } => properties.push(expr_),
                _ => {
                    return res.failure(Error::new(
                        "Invalid Syntax",
                        self.current_token.pos_start.clone(),
                        self.current_token.pos_end.clone(),
                        "Expected properties or methods",
                    ))
                }
            }
        }

        if self.current_token.r#type != Tokens::RightCurlyBraces {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected '}'",
            ));
        }

        res.register_advancement();
        self.advance();

        res.success(Node::ClassDefNode {
            name,
            constructor: Box::new(constructor),
            properties,
            methods,
            pos_start,
            pos_end: self.current_token.clone().pos_start,
        })
    }

    fn class_init(&mut self) -> ParseResult {
        let mut res = ParseResult::new();
        let pos_start = self.current_token.clone().pos_start;
        let mut constructor_params: Vec<Node> = vec![];

        if !self
            .current_token
            .clone()
            .matches(Tokens::Keyword, DynType::String("new".to_string()))
        {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected 'new'",
            ));
        }

        res.register_advancement();
        self.advance();

        if self.current_token.r#type != Tokens::Identifier {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected identifier",
            ));
        }

        let name = self.current_token.clone();

        res.register_advancement();
        self.advance();

        if self.current_token.r#type == Tokens::LeftParenthesis {
            res.register_advancement();
            self.advance();

            if self.current_token.r#type == Tokens::RightParenthesis {
                res.register_advancement();
                self.advance();
            } else {
                let expr = res.register(self.expr());
                if res.error.is_some() {
                    return res.failure(Error::new(
                        "Invalid Syntax",
                        self.current_token.pos_start.clone(),
                        self.current_token.pos_end.clone(),
                        "Expected ')', 'var', int, float, identifier, '+', '-' or ','",
                    ));
                }
                constructor_params.push(expr.unwrap());

                while self.current_token.r#type == Tokens::Comma {
                    res.register_advancement();
                    self.advance();

                    let expr = res.register(self.expr());
                    if res.error.is_some() {
                        return res.failure(Error::new(
                            "Invalid Syntax",
                            self.current_token.pos_start.clone(),
                            self.current_token.pos_end.clone(),
                            "Expected ')', 'var', int, float, identifier, '+', '-' or ','",
                        ));
                    }
                    constructor_params.push(expr.unwrap());
                }

                if self.current_token.r#type != Tokens::RightParenthesis {
                    return res.failure(Error::new(
                        "Invalid Syntax",
                        self.current_token.pos_start.clone(),
                        self.current_token.pos_end.clone(),
                        "Expected ')' or ','",
                    ));
                }
                res.register_advancement();
                self.advance();
            }
        } else {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected '('",
            ));
        }

        res.success(Node::ClassInitNode {
            name,
            constructor_params,
            pos_start,
            pos_end: self.current_token.clone().pos_end,
        })
    }

    fn if_expr(&mut self) -> ParseResult {
        let mut res = ParseResult::new();
        if !self
            .current_token
            .clone()
            .matches(Tokens::Keyword, DynType::String("if".to_string()))
        {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected 'if'",
            ));
        }

        res.register_advancement();
        self.advance();

        let pos_start = self.current_token.clone().pos_start;
        let mut cases: Vec<(Node, Node)> = vec![];
        let mut else_case: Option<Node> = None;

        let first_condition = res.register(self.expr());
        if res.error.is_some() {
            return res;
        }

        if !self
            .current_token
            .clone()
            .matches(Tokens::LeftCurlyBraces, DynType::None)
        {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected '{'",
            ));
        }

        res.register_advancement();
        self.advance();

        let first_expr = res.register(self.statements());
        if res.error.is_some() {
            return res;
        }

        cases.push((first_condition.unwrap(), first_expr.unwrap()));

        if !self
            .current_token
            .clone()
            .matches(Tokens::RightCurlyBraces, DynType::None)
        {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected '}'",
            ));
        }
        self.advance();
        res.register_advancement();

        while self
            .current_token
            .clone()
            .matches(Tokens::Keyword, DynType::String("else".to_string()))
        {
            res.register_advancement();
            self.advance();

            if self
                .current_token
                .clone()
                .matches(Tokens::Keyword, DynType::String("if".to_string()))
            {
                res.register_advancement();
                self.advance();

                let condition = res.register(self.expr());
                if res.error.is_some() {
                    return res;
                }

                if !self
                    .current_token
                    .clone()
                    .matches(Tokens::LeftCurlyBraces, DynType::None)
                {
                    return res.failure(Error::new(
                        "Invalid Syntax",
                        self.current_token.pos_start.clone(),
                        self.current_token.pos_end.clone(),
                        "Expected '{'",
                    ));
                }

                res.register_advancement();
                self.advance();

                let else_if = res.register(self.statements());
                if res.error.is_some() {
                    return res;
                }

                cases.push((condition.unwrap(), else_if.unwrap()));

                if !self
                    .current_token
                    .clone()
                    .matches(Tokens::RightCurlyBraces, DynType::None)
                {
                    return res.failure(Error::new(
                        "Invalid Syntax",
                        self.current_token.pos_start.clone(),
                        self.current_token.pos_end.clone(),
                        "Expected '}'",
                    ));
                }
                res.register_advancement();
                self.advance();
            } else {
                if !self
                    .current_token
                    .clone()
                    .matches(Tokens::LeftCurlyBraces, DynType::None)
                {
                    return res.failure(Error::new(
                        "Invalid Syntax",
                        self.current_token.pos_start.clone(),
                        self.current_token.pos_end.clone(),
                        "Expected '}'",
                    ));
                }
                self.advance();
                res.register_advancement();

                let else_ = res.register(self.statements());
                if res.error.is_some() {
                    return res;
                }

                else_case = Some(else_.unwrap());
                if !self
                    .current_token
                    .clone()
                    .matches(Tokens::RightCurlyBraces, DynType::None)
                {
                    return res.failure(Error::new(
                        "Invalid Syntax",
                        self.current_token.pos_start.clone(),
                        self.current_token.pos_end.clone(),
                        "Expected '}'",
                    ));
                }
                res.register_advancement();
                self.advance();
                break;
            }
        }
        res.success(Node::IfNode {
            cases,
            else_case: Box::new(else_case.clone()),
            pos_start,
            pos_end: self.current_token.clone().pos_end,
        })
    }

    fn while_expr(&mut self) -> ParseResult {
        let mut res = ParseResult::new();
        let pos_start = self.current_token.clone().pos_start;
        if !self
            .current_token
            .clone()
            .matches(Tokens::Keyword, DynType::String("while".to_string()))
        {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected 'while'",
            ));
        }

        res.register_advancement();
        self.advance();

        let condition_node = res.register(self.expr());
        if res.error.is_some() {
            return res;
        }

        if !self
            .current_token
            .clone()
            .matches(Tokens::LeftCurlyBraces, DynType::None)
        {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected '{'",
            ));
        }

        res.register_advancement();
        self.advance();

        let body_node = res.register(self.statements());
        if res.error.is_some() {
            return res;
        }

        if !self
            .current_token
            .clone()
            .matches(Tokens::RightCurlyBraces, DynType::None)
        {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected '}'",
            ));
        }

        res.register_advancement();
        self.advance();

        res.success(Node::WhileNode {
            condition_node: Box::new(condition_node.clone().unwrap()),
            body_node: Box::new(body_node.clone().unwrap()),
            pos_start: pos_start.clone(),
            pos_end: self.current_token.clone().pos_end,
        })
    }

    fn for_expr(&mut self) -> ParseResult {
        let mut res = ParseResult::new();
        if !self
            .current_token
            .clone()
            .matches(Tokens::Keyword, DynType::String("for".to_string()))
        {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected 'for'",
            ));
        }

        res.register_advancement();
        self.advance();
        let start = self.current_token.clone().pos_start;

        if self.current_token.r#type != Tokens::Identifier {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected Identifier",
            ));
        }

        let var_name = self.current_token.clone();
        res.register_advancement();
        self.advance();

        if self.current_token.r#type != Tokens::Equals {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected '='",
            ));
        }

        res.register_advancement();
        self.advance();

        let init_expr = res.register(self.expr());
        if res.error.is_some() {
            return res;
        }

        if !self
            .current_token
            .clone()
            .matches(Tokens::Keyword, DynType::String("to".to_string()))
        {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected 'to'",
            ));
        }

        res.register_advancement();
        self.advance();

        let end_expr = res.register(self.expr());
        if res.error.is_some() {
            return res;
        }

        let mut step: Option<Node> = None;
        if self
            .current_token
            .clone()
            .matches(Tokens::Keyword, DynType::String("step".to_string()))
        {
            res.register_advancement();
            self.advance();
            let expr = res.register(self.expr());
            if res.error.is_some() {
                return res;
            }
            step = Some(expr.unwrap());
        }

        if !self
            .current_token
            .clone()
            .matches(Tokens::LeftCurlyBraces, DynType::None)
        {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected '{'",
            ));
        }

        res.register_advancement();
        self.advance();

        let body = res.register(self.statements());
        if res.error.is_some() {
            return res;
        }

        if !self
            .current_token
            .clone()
            .matches(Tokens::RightCurlyBraces, DynType::None)
        {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected '}'",
            ));
        }

        res.register_advancement();
        self.advance();

        res.success(Node::ForNode {
            var_name_token: var_name,
            start_value: Box::new(init_expr.clone().unwrap()),
            end_value: Box::new(end_expr.clone().unwrap()),
            body_node: Box::new(body.clone().unwrap()),
            step_value_node: Box::new(step.clone()),
            pos_start: start,
            pos_end: self.current_token.clone().pos_end,
        })
    }

    fn fun_def(&mut self) -> ParseResult {
        let mut res = ParseResult::new();
        let pos_start = self.current_token.clone().pos_start;
        if !self
            .current_token
            .clone()
            .matches(Tokens::Keyword, DynType::String("fun".to_string()))
        {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected 'fun'",
            ));
        }

        res.register_advancement();
        self.advance();

        let mut fun_name: Option<Token> = None;
        if self.current_token.r#type == Tokens::Identifier {
            fun_name = Some(self.current_token.clone());

            res.register_advancement();
            self.advance();

            if self.current_token.r#type != Tokens::LeftParenthesis {
                return res.failure(Error::new(
                    "Invalid Syntax",
                    self.current_token.pos_start.clone(),
                    self.current_token.pos_end.clone(),
                    "Expected '('",
                ));
            }
        } else if self.current_token.r#type != Tokens::LeftParenthesis {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected '(' or identifier",
            ));
        }

        res.register_advancement();
        self.advance();

        let mut args_name_tokens: Vec<Token> = vec![];
        if self.current_token.r#type == Tokens::Identifier {
            let name = self.current_token.clone();
            args_name_tokens.push(name);

            res.register_advancement();
            self.advance();

            while self.current_token.r#type == Tokens::Comma {
                res.register_advancement();
                self.advance();

                if self.current_token.r#type == Tokens::Identifier {
                    let new_arg_token = self.current_token.clone();
                    args_name_tokens.push(new_arg_token);
                    res.register_advancement();
                    self.advance();
                } else {
                    return res.failure(Error::new(
                        "Invalid Syntax",
                        self.current_token.pos_start.clone(),
                        self.current_token.pos_end.clone(),
                        "Expected Identifier",
                    ));
                }
            }

            if self.current_token.r#type != Tokens::RightParenthesis {
                return res.failure(Error::new(
                    "Invalid Syntax",
                    self.current_token.pos_start.clone(),
                    self.current_token.pos_end.clone(),
                    "Expected ')' or ','",
                ));
            }
        } else if self.current_token.r#type != Tokens::RightParenthesis {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected ')' or identifier",
            ));
        }

        res.register_advancement();
        self.advance();

        if self.current_token.r#type != Tokens::Arrow {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected '=>'",
            ));
        }

        res.register_advancement();
        self.advance();

        if !self
            .current_token
            .clone()
            .matches(Tokens::LeftCurlyBraces, DynType::None)
        {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected '{'",
            ));
        }
        self.advance();
        res.register_advancement();

        let body_node = res.register(self.statements());
        if res.error.is_some() {
            return res;
        }

        if !self
            .current_token
            .clone()
            .matches(Tokens::RightCurlyBraces, DynType::None)
        {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected '}'",
            ));
        }
        self.advance();
        res.register_advancement();

        res.success(Node::FunDef {
            name: fun_name,
            body_node: Box::new(body_node.clone().unwrap()),
            arg_tokens: args_name_tokens,
            pos_start,
            pos_end: self.current_token.clone().pos_end,
        })
    }
}
