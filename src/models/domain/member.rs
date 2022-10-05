use crate::models::cdate::CDate;
use crate::models::domain::FromMap;
use crate::models::uuid::Uuid;
use derive_getters::{Dissolve, Getters};
use shared::{CData, CFromMap, CFromStr, CPartialEq, CToMap, CToStr, Model};
use std::collections::HashMap;
use std::str::FromStr;
use thiserror::Error;

pub trait MemberValidation {
    fn validate_id(&self) -> bool;
    fn validate_phone_nr(&self) -> bool;
    fn validate_email(&self) -> bool;
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
    pub fn new(name: String, email: String, phone_nr: String) -> Member {
        Member {
            uuid: Uuid::new(),
            day_of_creation: CDate::new(),
            credits: 0f64,
            // items: CVec::new(),
            name,
            email,
            phone_nr,
        }
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
    fn validate_id(&self) -> bool {
        todo!()
    }

    fn validate_phone_nr(&self) -> bool {
        todo!()
    }

    fn validate_email(&self) -> bool {
        todo!()
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
            // items: CVec::new(),
        }
    }
}

#[derive(Debug, Error)]
#[error("Tried adding/deducing a negative amount to credits")]
pub struct NegativeCreditInput;

#[cfg(test)]
mod member_test {

    use super::Member;

    #[test]
    fn test_explicit_creation() {
        let name = "Bob".to_owned();
        let email = "bob@gmail.com".to_owned();
        let phone_nr = "40123456789".to_owned();
        let bob = Member::new(name, email, phone_nr);
        assert_eq!(bob.name, "Bob".to_owned());
        assert_eq!(bob.email, "bob@gmail.com".to_owned());
        assert_eq!(bob.phone_nr, "40123456789".to_owned());
        assert_eq!(bob.credits, 0f64);
        // assert_eq!(bob.items.to_vec(), vec![])
    }

    #[test]
    fn test_default_creation() {
        let member = Member::default();
        assert_eq!(member.name, String::new());
        assert_eq!(member.email, String::new());
        assert_eq!(member.phone_nr, String::new());
        assert_eq!(member.credits, 0.0);
    }
}
