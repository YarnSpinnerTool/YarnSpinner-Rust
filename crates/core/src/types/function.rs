//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/Types/FunctionType.cs>

use crate::prelude::types::TypeProperties;
use crate::types::{Type, TypeFormat};
use std::fmt::Display;

pub(crate) fn function_type_properties(function_type: &FunctionType) -> TypeProperties {
    TypeProperties::from_name("Function").with_description(function_type.to_string())
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct FunctionType {
    pub parameters: Vec<Option<Type>>,
    // Needs to be on the heap because of type recursion
    pub return_type: Box<Option<Type>>,
}

impl From<FunctionType> for Type {
    fn from(function_type: FunctionType) -> Self {
        Type::Function(function_type)
    }
}

impl FunctionType {
    pub fn set_return_type(&mut self, return_type: impl Into<Option<Type>>) {
        self.return_type = Box::new(return_type.into());
    }

    pub fn add_parameter(&mut self, parameter: impl Into<Option<Type>>) {
        self.parameters.push(parameter.into());
    }

    pub fn properties(&self) -> TypeProperties {
        function_type_properties(self)
    }
}

impl Display for FunctionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let parameters = self
            .parameters
            .iter()
            .map(TypeFormat::format)
            .collect::<Vec<_>>()
            .join(", ");
        let return_type = self.return_type.as_ref().format();
        write!(f, "Fn({}) -> {}", parameters, return_type)
    }
}
