use bzxc_shared::{Error, Node, Tokens};

use crate::parse_result::ParseResult;

use super::Parser;

impl Parser {
    pub(crate) fn call_access_expr(
        &mut self,
        expr: Option<Node>,
        mut res: ParseResult,
    ) -> ParseResult {
        if self.current_token.value == Tokens::Dot {
            self.advance();
            res.register_advancement();

            if let Tokens::Identifier(_) = self.current_token.value {
            } else {
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

            let mut l;

            if self.current_token.value == Tokens::Equals {
                res.register_advancement();
                self.advance();

                let val = res.register(self.expr());
                if res.error.is_some() {
                    return res;
                }

                return res.success(Node::ObjectPropEdit {
                    object: Box::new(expr.clone().unwrap()),
                    property: id,
                    new_val: Box::new(val.unwrap()),
                });
            } else if self.current_token.value == Tokens::LeftParenthesis {
                let mut arg_nodes: Vec<Node> = vec![];
                res.register_advancement();
                self.advance();

                if self.current_token.value == Tokens::RightParenthesis {
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

                    while self.current_token.value == Tokens::Comma {
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

                    if self.current_token.value != Tokens::RightParenthesis {
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

                l = Node::ObjectMethodCall {
                    object: Box::new(expr.clone().unwrap()),
                    property: id,
                    args: arg_nodes,
                }
            } else {
                l = Node::ObjectPropAccess {
                    object: Box::new(expr.clone().unwrap()),
                    property: id,
                };
            }

            while self.current_token.value == Tokens::Dot {
                self.advance();
                res.register_advancement();

                if let Tokens::Identifier(_) = self.current_token.value {
                } else {
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

                if self.current_token.value == Tokens::Equals {
                    res.register_advancement();
                    self.advance();

                    let expr = res.register(self.expr());
                    if res.error.is_some() {
                        return res;
                    }

                    return res.success(Node::ObjectPropEdit {
                        object: Box::new(l),
                        property: id,
                        new_val: Box::new(expr.unwrap()),
                    });
                } else if self.current_token.value == Tokens::LeftParenthesis {
                    let mut arg_nodes: Vec<Node> = vec![];
                    res.register_advancement();
                    self.advance();

                    if self.current_token.value == Tokens::RightParenthesis {
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

                        while self.current_token.value == Tokens::Comma {
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

                        if self.current_token.value != Tokens::RightParenthesis {
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

                    l = Node::ObjectMethodCall {
                        object: Box::new(l),
                        property: id,
                        args: arg_nodes,
                    }
                } else {
                    l = Node::ObjectPropAccess {
                        object: Box::new(l),
                        property: id,
                    };
                }
            }
            return res.success(l);
        } else if self.current_token.value == Tokens::LeftParenthesis {
            let mut arg_nodes: Vec<Node> = vec![];
            res.register_advancement();
            self.advance();

            if self.current_token.value == Tokens::RightParenthesis {
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

                while self.current_token.value == Tokens::Comma {
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

                if self.current_token.value != Tokens::RightParenthesis {
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
                node_to_call: Box::new(expr.clone().unwrap()),
                args: arg_nodes,
            });
        } else if self.current_token.value == Tokens::LeftSquareBraces {
            res.register_advancement();
            self.advance();

            let idx = res.register(self.expr());
            if res.error.is_some() {
                return res;
            }

            if self.current_token.value != Tokens::RightSquareBraces {
                return res.failure(Error::new(
                    "Invalid Syntax",
                    self.current_token.pos_start.clone(),
                    self.current_token.pos_end.clone(),
                    "Expected ']'",
                ));
            }

            res.register_advancement();
            self.advance();

            return res.success(Node::ArrayAcess {
                array: Box::new(expr.unwrap()),
                index: Box::new(idx.unwrap()),
            });
        }

        res.success(expr.unwrap())
    }
}
