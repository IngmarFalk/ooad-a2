use thiserror::Error;

/// Member Validation Error.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum MemValError {
    /// Negative Credits
    NegativeCreditInput,
    /// Would go into negative
    DeduceAmountToHigh,
    /// Id error.
    Id,
    /// If the id contains anything else then integers or alphabetic characters.
    IdContainsNonAlphaNumeric,
    /// Email error.
    Email,
    /// If the email doesnt match the validation pattern.
    EmailPattern,
    /// Phone number error.
    PhoneNumber,
    /// if the phone number contains anything but integers.
    PhoneNumberContainsNonNumeric,
    /// If the phone number doesnt match the validation pattern.
    PhoneNumberPattern,
}

impl std::fmt::Display for MemValError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            MemValError::Id => f.write_str("Invalid Id"),
            MemValError::IdContainsNonAlphaNumeric => {
                f.write_str("Id contains non alpha-numeric characters.")
            }
            MemValError::Email => f.write_str("Invalid Email"),
            MemValError::EmailPattern => f.write_str("Email doesnt match any valid patterns."),
            MemValError::PhoneNumber => f.write_str("Invalid Phone number"),
            MemValError::PhoneNumberContainsNonNumeric => {
                f.write_str("Phone number contains non-numeric values.")
            }
            MemValError::PhoneNumberPattern => {
                f.write_str("Phone number doesnt match any valid patterns.")
            }
            MemValError::NegativeCreditInput => {
                f.write_str("Tried adding/substracting negative amount of credits.")
            }
            MemValError::DeduceAmountToHigh => f.write_str("The amount of credits to deduce is higher than the amount that the member currently owns.")
        }
    }
}

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

/// System error result.

/// System Error.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum SysError {
    /// If an object already exists.
    AlreadyExists,
    /// If an object doesnt exists.
    DoesntExist,
    /// Cannot insert an object.
    CannotInsert,
    /// Cannot delete an object.
    CannotDelete,
    /// Cannot update an object.
    CannotUpdate,
}

impl std::fmt::Display for SysError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            SysError::AlreadyExists => f.write_str("This object already exists."),
            SysError::DoesntExist => f.write_str("This object doesnt exists."),
            SysError::CannotInsert => f.write_str("There was an problem inserting this object."),
            SysError::CannotDelete => f.write_str("There was a problem deleting this object."),
            SysError::CannotUpdate => f.write_str("There was a problem updating this object."),
        }
    }
}
