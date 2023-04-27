//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/Types/FunctionType.cs>

use crate::prelude::types::TypeProperties;
use crate::prelude::YarnFn;
use crate::types::Type;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FunctionType {
    pub parameters: Vec<Type>,
    pub return_type: Option<Type>,
}

impl FunctionType {
    pub fn add_parameter(&mut self, parameter: Type) {
        self.parameters.push(parameter);
    }
}

impl TypeProperties for FunctionType {
    type RustType = Box<dyn YarnFn>;
    const NAME: &'static str = "Function";
}
