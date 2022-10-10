use crate::errors::{Check, MemValError};
use crate::models::uuid::Uuid;
use crate::types::FromMap;
use crate::types::{MemValResult, ValResult, Validate};
use derive_getters::{Dissolve, Getters};
use shared::{
    Builder, DeriveData, DeriveFromMap, DeriveFromStr, DerivePartialEq, DeriveToMap, DeriveToStr,
    Model,
};
use std::collections::HashMap;
use std::str::FromStr;

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
    DeriveFromStr,
    DeriveToStr,
    DeriveFromMap,
    DeriveToMap,
    DeriveData,
    DerivePartialEq,
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
