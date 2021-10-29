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
