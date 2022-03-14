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
use std::collections::HashMap;

use bzxc_shared::Type;

#[derive(Debug, Clone)]
pub struct TypeEnv(Vec<HashMap<String, Type>>);

impl TypeEnv {
    pub fn new() -> Self {
        Self(vec![HashMap::new()])
    }

    pub fn set(&mut self, k: String, v: Type) {
        self.0.last_mut().unwrap().insert(k, v);
    }

    pub fn get(&self, k: String) -> Option<Type> {
        for map in self.0.iter().rev() {
            if let Some(v) = map.get(&k) {
                return Some(v.clone());
            }
        }

        None
    }

    pub fn push_scope(&mut self) {
        self.0.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        self.0.pop();
    }
}
