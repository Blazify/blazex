use crate::utils::error::Error;
use inkwell::values::IntValue;

#[derive(Debug, Clone, Copy)]
pub struct CompilerResult<'a> {
    pub value: Option<IntValue<'a>>,
    pub error: Option<Error>,
}

impl<'a> CompilerResult<'a> {
    pub fn new() -> CompilerResult<'a> {
        CompilerResult {
            value: None,
            error: None,
        }
    }

    pub fn success(mut self, value: IntValue<'a>) -> CompilerResult<'a> {
        self.value = Some(value);
        self.clone()
    }

    pub fn failure(mut self, error: Error) -> CompilerResult<'a> {
        self.error = Some(error);
        self.clone()
    }

    pub fn register(mut self, res: CompilerResult<'a>) -> Option<IntValue<'a>> {
        if res.error.is_some() {
            self.error = res.error.clone();
        }
        res.value
    }
}
