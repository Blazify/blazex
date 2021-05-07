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
use crate::utils::{constants::DynType, constants::Tokens, position::Position};
use std::fmt::{Display, Error, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub r#type: Tokens,
    pub value: DynType,
    pub pos_start: Position,
    pub pos_end: Position,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        return if self.value == DynType::None {
            write!(f, "[{:?}]", self.r#type)
        } else {
            write!(f, "[{:?}]", self.value)
        };
    }
}

impl Token {
    pub fn new(r#type: Tokens, pos_start: Position, pos_end: Position, value: DynType) -> Token {
        Token {
            r#type,
            value,
            pos_start,
            pos_end,
        }
    }

    pub fn matches(&self, r#type: Tokens, value: DynType) -> bool {
        return self.r#type == r#type && self.value == value;
    }
}
