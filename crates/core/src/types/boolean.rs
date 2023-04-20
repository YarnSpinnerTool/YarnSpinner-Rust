//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner/Types/BooleanType.cs>

use crate::prelude::types::TypeProperties;
use crate::prelude::*;
use crate::yarn_fn_hash_map;

/// A type that bridges to [`bool`]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct BooleanType;

impl TypeProperties for BooleanType {
    const NAME: &'static str = "Bool";
    const METHODS: YarnFnHashMap = yarn_fn_hash_map! {
        Operator::EqualTo.to_string() => bool::eq,
    };
}
