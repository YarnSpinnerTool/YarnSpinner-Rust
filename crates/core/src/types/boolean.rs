//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/Types/BooleanType.cs>

use crate::library::{YarnFn, YarnFnWrapper};
use crate::prelude::types::TypeProperties;
use crate::prelude::Operator;
use std::collections::HashMap;
use std::convert::Into;

/// A type that bridges to [`bool`]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct BooleanType;

impl TypeProperties for BooleanType {
    const NAME: &'static str = "Bool";
    const METHODS: HashMap<String, Box<dyn YarnFn>> = HashMap::from(
        [(Operator::EqualTo.to_string(), bool::eq)]
            .map(|(k, v)| (k, Box::new(YarnFnWrapper::from(v)) as Box<dyn YarnFn>)),
    );
}
