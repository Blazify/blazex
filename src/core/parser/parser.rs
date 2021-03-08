use crate::core::nodes::binary_op_node::BinOpNode;
use crate::core::nodes::boolean_node::BooleanNode;
use crate::core::nodes::call_node::CallNode;
use crate::core::nodes::char_node::CharNode;
use crate::core::nodes::for_node::ForNode;
use crate::core::nodes::fun_def::FunDef;
use crate::core::nodes::if_node::IfNode;
use crate::core::nodes::number_node::NumberNode;
use crate::core::nodes::string_node::StringNode;
use crate::core::nodes::unary_node::UnaryNode;
use crate::core::nodes::var_access_node::VarAccessNode;
use crate::core::nodes::var_assign_node::VarAssignNode;
use crate::core::nodes::var_reassign_node::VarReassignNode;
use crate::core::nodes::while_node::WhileNode;
use crate::core::parser::parser_result::ParseResult;
use crate::core::token::Token;
use crate::utils::constants::{DynType, Nodes, Tokens};
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

    pub fn advance(&mut self) -> Token {
        self.token_index += 1;
        if self.token_index < self.tokens.len() {
            self.current_token = self.tokens.clone()[self.token_index].clone();
        };
        self.current_token.clone()
    }

    pub fn parse(&mut self) -> ParseResult {
        let mut res = self.expr();
        if res.error.is_none() && self.current_token.r#type != Tokens::EOF {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected Operators, Variables, Functions, etc but found none"
            ));
        }
        res
    }

    pub fn expr(&mut self) -> ParseResult {
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
                    "Expected Identifier"
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
                    "Expected '='"
                ));
            }

            res.register_advancement();
            self.advance();

            let expr = res.register(self.expr());
            res.register_advancement();
            self.advance();

            let assignable = if var_type == String::from("var") {
                true
            } else {
                false
            };
            return res.success(Nodes::VarAssignNode(Box::new(VarAssignNode::new(
                var_name,
                expr.unwrap(),
                assignable,
            ))));
        }

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
            left = Option::from(Nodes::BinOp(Box::new(BinOpNode::new(
                left.unwrap(),
                op_token,
                right.unwrap(),
            ))));
        }

        if res.error.is_some() {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected 'var', int, float, identifier, '+', '-' or '('"
            ));
        }

        res.success(left.unwrap())
    }

    pub fn comp_expr(&mut self) -> ParseResult {
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

            return res.success(Nodes::UnaryOp(Box::new(UnaryNode::new(
                node.unwrap(),
                op_token,
            ))));
        }

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
            left = Option::from(Nodes::BinOp(Box::new(BinOpNode::new(
                left.unwrap(),
                op_token,
                right.unwrap(),
            ))));
        }

        if res.error.is_some() {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "A Int or Float or Identifier, '+', '-', '(', 'not', '!' was Expected"
            ));
        }
        res.success(left.unwrap())
    }

    pub fn arith_expr(&mut self) -> ParseResult {
        let mut res = ParseResult::new();

        let mut left = res.register(self.term());
        if res.error.is_some() {
            return res;
        }

        if [Tokens::Plus, Tokens::Minus].contains(&self.current_token.r#type) {
            let op_token = self.current_token.clone();
            res.register_advancement();
            self.advance();

            let right = res.register(self.term());
            if res.error.is_some() {
                return res;
            }

            left = Option::from(Nodes::BinOp(Box::new(BinOpNode::new(
                left.unwrap(),
                op_token,
                right.unwrap(),
            ))));
        }

        res.success(left.unwrap())
    }

    pub fn term(&mut self) -> ParseResult {
        let mut res = ParseResult::new();

        let mut left = res.register(self.factor());
        if res.error.is_some() {
            return res;
        }

        if [Tokens::Multiply, Tokens::Divide].contains(&self.current_token.r#type) {
            let op_token = self.current_token.clone();
            res.register_advancement();
            self.advance();

            let right = res.register(self.factor());
            if res.error.is_some() {
                return res;
            }

            left = Option::from(Nodes::BinOp(Box::new(BinOpNode::new(
                left.unwrap(),
                op_token,
                right.unwrap(),
            ))));
        }

        res.success(left.unwrap())
    }

    pub fn factor(&mut self) -> ParseResult {
        let mut res = ParseResult::new();
        let token = self.current_token.clone();

        if [Tokens::Plus, Tokens::Minus].contains(&self.current_token.r#type) {
            res.register_advancement();
            self.advance();
            let factor = res.register(self.factor());
            if res.error.is_some() {
                return res;
            }
            return res.success(Nodes::UnaryOp(Box::new(UnaryNode::new(
                factor.unwrap(),
                token,
            ))));
        }
        self.power()
    }

    pub fn power(&mut self) -> ParseResult {
        let mut res = ParseResult::new();
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

            left = Option::from(Nodes::BinOp(Box::new(BinOpNode::new(
                left.unwrap(),
                op_token,
                right.unwrap(),
            ))));
        }

        res.success(left.unwrap())
    }

    pub fn call(&mut self) -> ParseResult {
        let mut res = ParseResult::new();
        let atom = res.register(self.atom());
        if res.error.is_some() {
            return res;
        }

        if self.current_token.r#type == Tokens::LeftParenthesis {
            let mut arg_nodes: Vec<Nodes> = vec![];
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
                        "Expected ')', 'var', int, float, identifier, '+', '-' or ','"
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
                            "Expected ')', 'var', int, float, identifier, '+', '-' or ','"
                        ));
                    }
                    arg_nodes.push(expr.unwrap());
                }

                if self.current_token.r#type != Tokens::RightParenthesis {
                    return res.failure(Error::new(
                        "Invalid Syntax",
                        self.current_token.pos_start.clone(),
                        self.current_token.pos_end.clone(),
                        "Expected ')' or ','"
                    ));
                }
            }
            return res.success(Nodes::CallNode(Box::new(CallNode::new(
                atom.unwrap(),
                Some(arg_nodes),
            ))));
        }

        res.success(atom.unwrap())
    }

    pub fn atom(&mut self) -> ParseResult {
        let mut res = ParseResult::new();
        let token = self.current_token.clone();

        if [Tokens::Int, Tokens::Float].contains(&token.r#type) {
            res.register_advancement();
            self.advance();
            return res.success(Nodes::Number(Box::new(NumberNode::new(token))));
        } else if token.r#type == Tokens::Boolean {
            res.register_advancement();
            self.advance();
            return res.success(Nodes::BooleanNode(Box::new(BooleanNode::new(token))));
        } else if token.r#type == Tokens::String {
            res.register_advancement();
            self.advance();
            return res.success(Nodes::StringNode(Box::new(StringNode::new(token))));
        } else if token.r#type == Tokens::Char {
            res.register_advancement();
            self.advance();
            return res.success(Nodes::CharNode(Box::new(CharNode::new(token))));
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

                res.register_advancement();
                self.advance();

                return res.success(Nodes::VarReassignNode(Box::new(VarReassignNode::new(
                    token,
                    new_value.unwrap(),
                ))));
            }

            return res.success(Nodes::VarAccessNode(Box::new(VarAccessNode::new(token))));
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
                    "Expected ')'"
                ));
            }
            return res.success(expr.unwrap());
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
        }

        res.failure(Error::new(
            "Invalid Syntax",
            token.pos_start,
            token.pos_end,
            "A Int, Float, String, Char, Keyword, Identifier, '+', '-', '(', etc was Expected"
        ))
    }

    pub fn if_expr(&mut self) -> ParseResult {
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
                "Expected 'if'"
            ));
        }

        res.register_advancement();
        self.advance();

        let mut cases: Vec<(Nodes, Nodes)> = vec![];
        let mut else_case: Option<Nodes> = None;

        let first_condition = res.register(self.expr());
        if res.error.is_some() {
            return res;
        }

        if !self
            .current_token
            .clone()
            .matches(Tokens::Keyword, DynType::String("then".to_string()))
        {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected 'then'"
            ));
        }

        res.register_advancement();
        self.advance();

        let first_expr = res.register(self.expr());
        if res.error.is_some() {
            return res;
        }
        cases.push((first_condition.unwrap(), first_expr.unwrap()));

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
                    .matches(Tokens::Keyword, DynType::String("then".to_string()))
                {
                    return res.failure(Error::new(
                        "Invalid Syntax",
                        self.current_token.pos_start.clone(),
                        self.current_token.pos_end.clone(),
                        "Expected 'then'"
                    ));
                }

                res.register_advancement();
                self.advance();

                let expr = res.register(self.expr());
                if res.error.is_some() {
                    return res;
                }

                cases.push((condition.unwrap(), expr.unwrap()));
                res.register_advancement();
                self.advance();
            } else {
                let else_expr = res.register(self.expr());
                if res.error.is_some() {
                    return res;
                }
                else_case = Some(else_expr.unwrap());
                res.register_advancement();
                self.advance();
            }
        }
        res.success(Nodes::IfNode(Box::new(IfNode::new(cases, else_case))))
    }

    pub fn while_expr(&mut self) -> ParseResult {
        let mut res = ParseResult::new();
        if !self
            .current_token
            .clone()
            .matches(Tokens::Keyword, DynType::String("while".to_string()))
        {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected 'while'"
            ));
        }

        res.register_advancement();
        self.advance();

        let condition = res.register(self.expr());
        if res.error.is_some() {
            return res;
        }

        if !self
            .current_token
            .clone()
            .matches(Tokens::Keyword, DynType::String("then".to_string()))
        {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected 'then'"
            ));
        }

        res.register_advancement();
        self.advance();

        let body = res.register(self.expr());
        if res.error.is_some() {
            return res;
        }

        res.success(Nodes::WhileNode(Box::new(WhileNode::new(
            condition.unwrap(),
            body.unwrap(),
        ))))
    }

    pub fn for_expr(&mut self) -> ParseResult {
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
                "Expected 'for'"
            ));
        }

        res.register_advancement();
        self.advance();

        if self.current_token.r#type != Tokens::Identifier {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected Identifier"
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
                "Expected '='"
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
                "Expected 'to'"
            ));
        }

        res.register_advancement();
        self.advance();

        let end_expr = res.register(self.expr());
        if res.error.is_some() {
            return res;
        }

        let mut step: Option<Nodes> = None;
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
            .matches(Tokens::Keyword, DynType::String("then".to_string()))
        {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected 'then'"
            ));
        }

        res.register_advancement();
        self.advance();

        let body = res.register(self.expr());
        if res.error.is_some() {
            return res;
        }

        res.success(Nodes::ForNode(Box::new(ForNode::new(
            var_name,
            init_expr.unwrap(),
            end_expr.unwrap(),
            body.unwrap(),
            step,
        ))))
    }

    pub fn fun_def(&mut self) -> ParseResult {
        let mut res = ParseResult::new();
        if !self
            .current_token
            .clone()
            .matches(Tokens::Keyword, DynType::String("fun".to_string()))
        {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected 'fun'"
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
                    "Expected '('"
                ));
            }
        } else if self.current_token.r#type != Tokens::LeftParenthesis {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected '(' or identifier"
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
                        "Expected Identifier"
                    ));
                }
            }

            if self.current_token.r#type != Tokens::RightParenthesis {
                return res.failure(Error::new(
                    "Invalid Syntax",
                    self.current_token.pos_start.clone(),
                    self.current_token.pos_end.clone(),
                    "Expected ')' or ','"
                ));
            }
        } else if self.current_token.r#type != Tokens::RightParenthesis {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected ')' or identifier"
            ));
        }

        res.register_advancement();
        self.advance();

        if self.current_token.r#type != Tokens::Arrow {
            return res.failure(Error::new(
                "Invalid Syntax",
                self.current_token.pos_start.clone(),
                self.current_token.pos_end.clone(),
                "Expected '=>'"
            ));
        }

        res.register_advancement();
        self.advance();

        let body_node = res.register(self.expr());
        if res.error.is_some() {
            return res;
        }

        res.success(Nodes::FunDef(Box::new(FunDef::new(
            body_node.unwrap(),
            fun_name,
            Some(args_name_tokens),
        ))))
    }
}
