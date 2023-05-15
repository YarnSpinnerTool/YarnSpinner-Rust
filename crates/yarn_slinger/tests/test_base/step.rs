//! Adapted from <https://github.com/YarnSpinnerTool/YarnSpinner/blob/da39c7195107d8211f21c263e4084f773b84eaff/YarnSpinner.Tests/TestPlan.cs>

use reader::*;
use std::fmt::Debug;
use std::str::FromStr;

mod reader;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Step {
    pub expected_step_type: ExpectedStepType,
    pub value: Option<StepValue>,
    pub expect_option_enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StepValue {
    String(String),
    Number(usize),
}

impl Step {
    pub(crate) fn read(string: &str) -> Self {
        let mut reader = Reader::new(string);
        let expected_step_type = reader.read_next::<ExpectedStepType>();
        let delimiter: String = reader.read_next();
        assert_eq!(":", delimiter, "Expected ':' after step type");

        match expected_step_type {
            ExpectedStepType::Line | ExpectedStepType::Option | ExpectedStepType::Command => {
                let buf = reader.read_to_end();
                let value = buf.trim().to_owned();

                // Options whose text ends with " [disabled]"
                // are expected to be present, but have their
                // 'allowed' flag set to false
                if value == "*" {
                    Self::with_expected_step_type(expected_step_type)
                } else if expected_step_type == ExpectedStepType::Option
                    && value.ends_with(" [disabled]")
                {
                    Self {
                        expected_step_type,
                        value: Some(value.replace(" [disabled]", "").into()),
                        expect_option_enabled: false,
                    }
                } else {
                    Self::with_value_and_type(value, expected_step_type)
                }
            }
            ExpectedStepType::Select => {
                let value = reader.read_next::<usize>();
                Self::with_value_and_type(value, expected_step_type)
            }
            ExpectedStepType::Stop => Self::with_expected_step_type(expected_step_type),
        }
    }

    pub(crate) fn from_line(line: impl Into<String>) -> Self {
        Self::with_value_and_type(line.into(), ExpectedStepType::Line)
    }

    pub(crate) fn from_option(line: impl Into<String>) -> Self {
        Self::with_value_and_type(line.into(), ExpectedStepType::Option)
    }

    pub(crate) fn from_command(line: impl Into<String>) -> Self {
        Self::with_value_and_type(line.into(), ExpectedStepType::Command)
    }

    pub(crate) fn from_select(selection: impl Into<usize>) -> Self {
        Self::with_value_and_type(selection.into(), ExpectedStepType::Select)
    }

    pub(crate) fn from_stop() -> Self {
        Self::with_expected_step_type(ExpectedStepType::Stop)
    }

    fn with_value_and_type(
        value: impl Into<StepValue>,
        expected_step_type: ExpectedStepType,
    ) -> Self {
        Self {
            expected_step_type,
            value: Some(value.into()),
            expect_option_enabled: true,
        }
    }
    fn with_expected_step_type(expected_step_type: ExpectedStepType) -> Self {
        Self {
            expected_step_type,
            value: None,
            expect_option_enabled: true,
        }
    }
}

impl From<String> for StepValue {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}

impl From<&str> for StepValue {
    fn from(value: &str) -> Self {
        Self::String(to_rust_serialization(value))
    }
}

impl From<usize> for StepValue {
    fn from(value: usize) -> Self {
        Self::Number(value)
    }
}

impl TryInto<String> for StepValue {
    type Error = ();

    fn try_into(self) -> Result<String, Self::Error> {
        match self {
            Self::String(value) => Ok(value),
            _ => Err(()),
        }
    }
}

impl TryInto<usize> for StepValue {
    type Error = ();

    fn try_into(self) -> Result<usize, Self::Error> {
        match self {
            Self::Number(value) => Ok(value),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
pub enum ExpectedStepType {
    // expecting to see this specific line
    #[default]
    Line,

    // expecting to see this specific option (if '*' is given,
    // means 'see an option, don't care about text')
    Option,

    // expecting options to have been presented; value = the
    // index to select
    Select,

    // expecting to see this specific command
    Command,

    // expecting to stop the test here (this is optional - a
    // 'stop' at the end of a test plan is assumed)
    Stop,
}

impl FromStr for ExpectedStepType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "line" => Ok(Self::Line),
            "option" => Ok(Self::Option),
            "select" => Ok(Self::Select),
            "command" => Ok(Self::Command),
            "stop" => Ok(Self::Stop),
            _ => Err(()),
        }
    }
}

fn to_rust_serialization(line: &str) -> String {
    // Need to do this because in Rust, booleans are not capitalized when converted to strings.
    // But in C# and hence our test plans, they are: https://stackoverflow.com/questions/491334/why-does-boolean-tostring-output-true-and-not-true
    line.replace("True", "true").replace("False", "false")
}
