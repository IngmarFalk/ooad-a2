use crate::models::cdate::CDate;
use crate::models::domain::FromMap;
use crate::models::uuid::Uuid;
use derive_getters::{Dissolve, Getters};
use shared::{CData, CFromMap, CFromStr, CPartialEq, CToMap, CToStr, Model};
use std::collections::HashMap;
use std::str::FromStr;
use thiserror::Error;

pub trait MemberValidation {
    fn validate(&self) -> MValResult<Member>;
    fn validate_id(&self) -> MValResult<()>;
    fn validate_phone_nr(&self) -> MValResult<()>;
    fn validate_email(&self) -> MValResult<()>;
}

#[derive(
    Clone, Debug, Getters, Dissolve, CFromStr, CToStr, CFromMap, CToMap, CData, CPartialEq, Model,
)]
#[dissolve(rename = "unpack")]
pub struct Member {
    #[getter(rename = "get_name")]
    #[eq]
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
    day_of_creation: CDate,

    #[getter(rename = "get_uuid")]
    #[mutable_ignore]
    uuid: Uuid,
}

impl Member {
    pub fn new(name: String, email: String, phone_nr: String) -> MValResult<Member> {
        let m = Member {
            uuid: Uuid::new(),
            day_of_creation: CDate::new(),
            credits: 0f64,
            name,
            email,
            phone_nr,
        };
        m.validate()
    }

    pub fn add_credits(&mut self, credits: f64) -> Result<(), NegativeCreditInput> {
        if credits < 0.0 {
            return Err(NegativeCreditInput);
        }
        self.credits += credits;
        Ok(())
    }

    pub fn deduce_credits(&mut self, credits: f64) -> Result<(), NegativeCreditInput> {
        if credits < 0.0 {
            return Err(NegativeCreditInput);
        }
        self.credits -= credits;
        Ok(())
    }
}

impl MemberValidation for Member {
    fn validate_id(&self) -> MValResult<()> {
        if let true = self.get_uuid().get_len() != &6 {
            return Err(MemValError::Id);
        }

        Ok(())
    }

    fn validate_phone_nr(&self) -> MValResult<()> {
        if let false = self
            .get_phone_nr()
            .chars()
            .map(|chr| chr.to_string().parse::<usize>())
            .all(|item| item.is_ok())
        {
            return Err(MemValError::ContainsNonIntegers);
        }

        let reg =
            regex::Regex::new(r"^(\+\d{1,2}\s?)?1?\-?\.?\s?\(?\d{3}\)?[\s.-]?\d{3}[\s.-]?\d{4}$")
                .unwrap();

        if let false = reg.is_match(self.get_phone_nr()) {
            return Err(MemValError::PhoneNumberPattern);
        }

        Ok(())
    }

    fn validate_email(&self) -> MValResult<()> {
        let reg =
            regex::Regex::new(r"/^([a-zA-Z0-9_\.\-])+\@(([a-zA-Z0-9\-])+\.)+([a-zA-Z0-9]{2,4})+$/")
                .unwrap();

        if let false = reg.is_match(&self.get_email()) {
            return Err(MemValError::PhoneNumberPattern);
        }

        Ok(())
    }

    fn validate(&self) -> MValResult<Member> {
        if let Err(err) = self.validate_email() {
            return Err(err);
        }
        if let Err(err) = self.validate_id() {
            return Err(err);
        }
        if let Err(err) = self.validate_phone_nr() {
            return Err(err);
        }
        Ok(self.clone())
    }
}

impl Default for Member {
    fn default() -> Self {
        Self {
            name: String::new(),
            email: String::new(),
            phone_nr: String::new(),
            credits: 0f64,
            day_of_creation: CDate::new(),
            uuid: Uuid::new(),
        }
    }
}

pub type MValResult<T> = Result<T, MemValError>;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum MemValError {
    Id,
    Email,
    EmailPattern,
    PhoneNumber,
    ContainsNonIntegers,
    PhoneNumberPattern,
}

impl std::fmt::Display for MemValError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            MemValError::Id => f.write_str("Invalid Id"),
            MemValError::Email => f.write_str("Invalid Email"),
            MemValError::EmailPattern => f.write_str("Email doesnt match any valid patterns."),
            MemValError::PhoneNumber => f.write_str("Invalid Phone number"),
            MemValError::ContainsNonIntegers => {
                f.write_str("Phone number contains non-integer values.")
            }
            MemValError::PhoneNumberPattern => {
                f.write_str("Phone number doesnt match any valid patterns.")
            }
        }
    }
}

#[derive(Debug, Error)]
#[error("Tried adding/deducing a negative amount to credits")]
pub struct NegativeCreditInput;

#[cfg(test)]
mod member_test {

    use super::Member;

    // #[test]
    // fn test_explicit_creation() {
    //     let name = "Bob".to_owned();
    //     let email = "bob@gmail.com".to_owned();
    //     let phone_nr = "40123456789".to_owned();
    //     let bob = Member::new(name, email, phone_nr);
    //     assert_eq!(bob.name, "Bob".to_owned());
    //     assert_eq!(bob.email, "bob@gmail.com".to_owned());
    //     assert_eq!(bob.phone_nr, "40123456789".to_owned());
    //     assert_eq!(bob.credits, 0f64);
    //     // assert_eq!(bob.items.to_vec(), vec![])
    // }

    #[test]
    fn test_default_creation() {
        let member = Member::default();
        assert_eq!(member.name, String::new());
        assert_eq!(member.email, String::new());
        assert_eq!(member.phone_nr, String::new());
        assert_eq!(member.credits, 0.0);
    }
}
