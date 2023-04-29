use crate::prelude::*;

impl From<String> for Operand {
    fn from(s: String) -> Self {
        Self {
            value: Some(operand::Value::StringValue(s)),
        }
    }
}

impl From<f32> for Operand {
    fn from(f: f32) -> Self {
        Self {
            value: Some(operand::Value::FloatValue(f)),
        }
    }
}

impl From<bool> for Operand {
    fn from(b: bool) -> Self {
        Self {
            value: Some(operand::Value::BoolValue(b)),
        }
    }
}
