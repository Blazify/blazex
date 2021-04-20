use crate::core::interpreter::r#type::Type;
use crate::utils::error::Error;

#[derive(Debug, Clone)]
pub struct RuntimeResult {
    pub val: Option<Type>,
    pub error: Option<Error>,
}

impl RuntimeResult {
    pub fn new() -> Self {
        Self {
            val: None,
            error: None,
        }
    }

    pub fn success(mut self, val: Type) -> Self {
        self.val = Some(val);
        self
    }

    pub fn failure(mut self, error: Error) -> Self {
        self.error = Some(error);
        self
    }

    pub fn register(mut self, res: Self) -> Self {
        if res.error.is_some() {
            self.error = res.error;
        } else {
            self.val = res.val;
        }
        self
    }
}
