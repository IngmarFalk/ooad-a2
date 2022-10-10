use crate::models::domain::FromMap;
use crate::models::uuid::Uuid;
use crate::types::{Check, ValResult, Validate};
use derive_getters::{Dissolve, Getters};
use shared::{Builder, CData, CFromMap, CFromStr, CPartialEq, CToMap, CToStr, Model};
use std::collections::HashMap;
use std::str::FromStr;
use thiserror::Error;

/// Defines methods needed for member validation.
pub trait MemberValidation {
    /// Validates the id.
    fn validate_id(&self) -> MemValResult<()>;
    /// Validates the phone number.
    fn validate_phone_nr(&self) -> MemValResult<()>;
    /// Validates email.
    fn validate_email(&self) -> MemValResult<()>;
}

/// Member.
#[derive(
    Clone,
    Debug,
    Getters,
    Dissolve,
    CFromStr,
    CToStr,
    CFromMap,
    CToMap,
    CData,
    CPartialEq,
    Model,
    Builder,
)]
#[dissolve(rename = "unpack")]
pub struct Member {
    #[getter(rename = "get_name")]
    name: String,

    #[getter(rename = "get_email")]
    #[eq]
    email: String,

    #[getter(rename = "get_phone_nr")]
    #[eq]
    phone_nr: String,

    #[getter(rename = "get_credits")]
    #[mutable_ignore]
    credits: f64,

    #[getter(rename = "get_day_of_creation")]
    #[mutable_ignore]
    day_of_creation: usize,

    #[getter(rename = "get_uuid")]
    #[mutable_ignore]
    uuid: Uuid,
}

impl Member {
    /// Creates new member.
    pub fn new(
        name: String,
        email: String,
        phone_nr: String,
        day_of_creation: usize,
    ) -> ValResult<Member> {
        let m = Member {
            uuid: Uuid::new(),
            credits: 0f64,
            day_of_creation,
            name,
            email,
            phone_nr,
        };
        m.validate_and_build()
    }

    /// Adds credits to member.
    pub fn add_credits(&mut self, credits: f64) -> MemValResult<()> {
        if credits < 0.0 {
            return Err(MemValError::NegativeCreditInput);
        }
        self.credits += credits;
        Ok(())
    }

    /// Deduces credits from member.
    pub fn deduce_credits(&mut self, credits: f64) -> MemValResult<()> {
        if credits < 0.0 {
            return Err(MemValError::NegativeCreditInput);
        }
        if self.credits - credits < 0.0 {
            return Err(MemValError::NegativeCreditInput);
        }
        self.credits -= credits;
        Ok(())
    }
}

impl MemberValidation for Member {
    fn validate_id(&self) -> MemValResult<()> {
        if let true = self.get_uuid().get_len() != &6 {
            return Err(MemValError::Id);
        }

        if let false = self
            .get_uuid()
            .get_value()
            .chars()
            .into_iter()
            .all(|c| c.is_alphanumeric())
        {
            return Err(MemValError::IdContainsNonAlphaNumeric);
        }

        Ok(())
    }

    fn validate_phone_nr(&self) -> MemValResult<()> {
        if let false = self
            .get_phone_nr()
            .chars()
            .map(|chr| chr.is_ascii_digit() || chr.is_whitespace())
            .all(|is_digit| is_digit)
        {
            return Err(MemValError::PhoneNumberContainsNonNumeric);
        }

        let reg = regex::Regex::new(r"([ 0-9]){8,12}$").unwrap();

        if let false = reg.is_match(self.get_phone_nr()) {
            return Err(MemValError::PhoneNumberPattern);
        }

        if self.get_phone_nr().len() > 12 || self.get_phone_nr().len() < 8 {
            return Err(MemValError::PhoneNumber);
        }

        Ok(())
    }

    fn validate_email(&self) -> MemValResult<()> {
        let email_regex = regex::Regex::new(
            r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
        )
        .unwrap();

        if let false = email_regex.is_match(self.get_email()) {
            return Err(MemValError::EmailPattern);
        }

        Ok(())
    }
}

impl Validate<Member> for Member {
    fn validate(&self) -> ValResult<()> {
        if let Err(err) = self.validate_email() {
            return Err(Check::Invalid(err.to_string()));
        }
        if let Err(err) = self.validate_id() {
            return Err(Check::Invalid(err.to_string()));
        }
        if let Err(err) = self.validate_phone_nr() {
            return Err(Check::Invalid(err.to_string()));
        }
        Ok(())
    }

    fn validate_and_build(self) -> ValResult<Member> {
        match self.validate() {
            Ok(_) => Ok(self),
            Err(err) => Err(err),
        }
    }
}

impl Default for Member {
    fn default() -> Self {
        Self {
            name: String::new(),
            email: String::new(),
            phone_nr: String::new(),
            credits: 0f64,
            day_of_creation: 0,
            uuid: Uuid::new(),
        }
    }
}

/// Member validation result.
///
/// This is the result returned by the member validation,
pub type MemValResult<T> = Result<T, MemValError>;

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
