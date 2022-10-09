use std::clone;

use thiserror::Error;

/// All traits in this file can be implemented without any methods.
///
/// E.g:
/// ```rust
/// struct ExampleView {}
///
/// impl View for ExampleView {}
///
/// struct ExampleModel {}
///
/// impl Model for ExampleModel {}
///
/// struct ExampleController {}
///
/// impl Controller for ExampleController {}
/// ```

/// All Domain models implement this trait.
pub trait Model {}

/// All ui/ux related structs implement this trait.
pub trait View {}

/// All Controllers implement this trait.
pub trait Controller {}

pub type ValResult<T> = Result<T, Check>;

/// Member Validation Error.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum Check {
    Ok,
    Invalid(String),
}

impl std::fmt::Display for Check {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Check::Ok => f.write_str("Ok"),
            Check::Invalid(s) => f.write_str(s.as_str()),
        }
    }
}

pub trait Validate<T> {
    fn validate(&self) -> ValResult<()>;
    fn validate_and_build(self) -> ValResult<T>;
}
