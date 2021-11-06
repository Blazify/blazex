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
pub struct TypeEnv(HashMap<String, Type>);

impl TypeEnv {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn set(&mut self, k: String, v: Type) {
        self.0.insert(k, v);
    }

    pub fn get(&self, k: String) -> Option<Type> {
        if let Some(ty) = self.0.get(&k) {
            Some(ty.clone())
        } else {
            None
        }
    }
}
