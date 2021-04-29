use crate::core::interpreter::value::Value;
use crate::utils::error::Error;

#[derive(Debug, Clone)]
pub struct RuntimeResult {
    pub val: Option<Value>,
    pub error: Option<Error>,
    pub return_val: bool,
}

impl RuntimeResult {
    pub fn new() -> Self {
        Self {
            val: None,
            error: None,
            return_val: false,
        }
    }

    pub fn reset(&mut self) -> Self {
        self.error = None;
        self.val = None;
        self.return_val = false;
        self.clone()
    }

    pub fn success(&mut self, val: Value) -> Self {
        self.reset();
        self.val = Some(val);
        self.clone()
    }

    pub fn success_return(&mut self, val: Value) -> Self {
        self.reset();
        self.return_val = true;
        self.val = Some(val);
        self.clone()
    }

    pub fn failure(mut self, error: Error) -> Self {
        self.error = Some(error);
        self
    }

    pub fn should_return(&self) -> bool {
        self.error.is_some() || self.return_val
    }
}
