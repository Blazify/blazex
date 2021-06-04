/*
 * Copyright 2020 to 2021 BlazifyOrg
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *    http://www.apache.org/licenses/LICENSE-2.0
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
*/

use bzxc_shared::{to_static_str, DynType, Error, Tokens, Type};

use crate::parse_result::ParseResult;

use super::Parser;

impl Parser {
    pub(crate) fn type_expr(&mut self, res: &mut ParseResult) -> Result<Type, Error> {
        let pos_start = self.current_token.pos_start.clone();

        if self.current_token.typee != Tokens::Identifier
            && self.current_token.typee != Tokens::Keyword
            && self.current_token.typee != Tokens::LeftSquareBraces
        {
            return Err(Error::new(
                "Invalid Syntax",
                pos_start,
                self.current_token.pos_end.clone(),
                "Expected Type",
            ));
        }

        match &self.current_token.value.clone() {
            DynType::String(typee) => {
                res.register_advancement();
                self.advance();

                match typee.as_str() {
                    "int" => Ok(Type::Int),
                    "float" => Ok(Type::Float),
                    "char" => Ok(Type::Char),
                    "boolean" => Ok(Type::Boolean),
                    "string" => Ok(Type::String),
                    "void" => Ok(Type::Void),
                    "fun" => {
                        if self.current_token.typee != Tokens::LeftParenthesis {
                            return Err(Error::new(
                                "Syntax Error",
                                pos_start,
                                self.current_token.pos_end.clone(),
                                "Expected '('",
                            ));
                        }

                        self.advance();
                        res.register_advancement();

                        let mut arg_types = vec![];
                        if self.current_token.typee == Tokens::RightParenthesis {
                            self.advance();
                            res.register_advancement();

                            if self.current_token.typee != Tokens::Colon {
                                return Err(Error::new(
                                    "Invalid Syntax",
                                    self.current_token.pos_start.clone(),
                                    self.current_token.pos_end.clone(),
                                    "Expected ':'",
                                ));
                            }

                            res.register_advancement();
                            self.advance();
                            let typee = self.type_expr(res);
                            match typee {
                                Ok(typ) => Ok(Type::Function(vec![], Box::new(typ))),
                                Err(e) => Err(e),
                            }
                        } else if self.current_token.typee == Tokens::Keyword {
                            let typee = self.type_expr(res);
                            match typee {
                                Ok(typ) => arg_types.push(typ),
                                Err(e) => return Err(e),
                            }

                            while self.current_token.typee == Tokens::Comma {
                                res.register_advancement();
                                self.advance();

                                if self.current_token.typee == Tokens::Keyword {
                                    let typee = self.type_expr(res);
                                    match typee {
                                        Ok(typ) => arg_types.push(typ),
                                        Err(e) => return Err(e),
                                    }
                                } else {
                                    return Err(Error::new(
                                        "Invalid Syntax",
                                        self.current_token.pos_start.clone(),
                                        self.current_token.pos_end.clone(),
                                        "Expected keyword",
                                    ));
                                }
                            }

                            if self.current_token.typee != Tokens::RightParenthesis {
                                return Err(Error::new(
                                    "Invalid Syntax",
                                    self.current_token.pos_start.clone(),
                                    self.current_token.pos_end.clone(),
                                    "Expected ')' or ','",
                                ));
                            }

                            self.advance();
                            res.register_advancement();

                            if self.current_token.typee != Tokens::Colon {
                                return Err(Error::new(
                                    "Invalid Syntax",
                                    self.current_token.pos_start.clone(),
                                    self.current_token.pos_end.clone(),
                                    "Expected ':'",
                                ));
                            }

                            res.register_advancement();
                            self.advance();
                            let typee = self.type_expr(res);
                            match typee {
                                Ok(typ) => Ok(Type::Function(arg_types, Box::new(typ))),
                                Err(e) => Err(e),
                            }
                        } else {
                            Err(Error::new(
                                "Syntax Error",
                                pos_start,
                                self.current_token.pos_end.clone(),
                                "Expected ')' or arguments",
                            ))
                        }
                    }
                    _ => Ok(Type::Custom(to_static_str(typee.clone()))),
                }
            }
            _ => match self.current_token.typee {
                Tokens::LeftSquareBraces => {
                    println!("omaiwa");
                    self.advance();
                    res.register_advancement();

                    let typee = self.type_expr(res)?;

                    if self.current_token.typee != Tokens::Comma {
                        return Err(Error::new(
                            "Syntax Error",
                            pos_start,
                            self.current_token.pos_end.clone(),
                            "Expected ','",
                        ));
                    }

                    self.advance();
                    res.register_advancement();

                    if self.current_token.typee != Tokens::Int {
                        return Err(Error::new(
                            "Syntax Error",
                            pos_start,
                            self.current_token.pos_end.clone(),
                            "Expected int",
                        ));
                    }

                    let size = self.current_token.clone();

                    res.register_advancement();
                    self.advance();

                    if self.current_token.typee != Tokens::RightSquareBraces {
                        return Err(Error::new(
                            "Syntax Error",
                            pos_start,
                            self.current_token.pos_end.clone(),
                            "Expected ']'",
                        ));
                    }

                    self.advance();
                    res.register_advancement();
                    Ok(Type::Array(Box::new(typee), size))
                }
                _ => Err(Error::new(
                    "Invalid Syntax",
                    pos_start,
                    self.current_token.pos_end.clone(),
                    "Expected Type",
                )),
            },
        }
    }
}
