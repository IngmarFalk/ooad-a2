use crate::models::cdate::CDate;
use crate::models::domain::FromMap;
use crate::models::uuid::Uuid;
use crate::types::Model;
use derive_getters::{Dissolve, Getters};
use shared::{CData, CFromMap, CFromStr, CPartialEq, CToMap, CToStr};
use std::collections::HashMap;
use std::str::FromStr;
use thiserror::Error;

pub trait MemberValidation {
    fn validate_id(&self) -> bool;
    fn validate_phone_nr(&self) -> bool;
    fn validate_email(&self) -> bool;
}

/// If you see this warning from vscode: It is a bug within rust-analyzer, the
/// linter used for rust. Important: ! This is not a Bug in this code !
#[derive(
    Clone, Debug, Getters, Dissolve, CFromStr, CToStr, CFromMap, CToMap, CData, CPartialEq,
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
    // #[getter(rename = "get_items")]
    // #[mutable_ignore]
    // items: CVec<Item>,
}

impl Model for Member {}

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

    // pub fn add_item(&mut self, item: Item) -> MResult<()> {
    //     let exists = self.items.iter().any(|e| e.get_uuid() == item.get_uuid());
    //     if exists {
    //         return Err(MError::AlreadyExists);
    //     }
    //     self.items.push(item);
    //     self.add_credits(100f64).unwrap();
    //     Ok(())
    // }

    // pub fn remove_item(&mut self, item: Item) -> MResult<()> {
    //     if !self.has_item(&item) {
    //         return Err(MError::DoesntExist);
    //     }
    //     self.items.to_vec().retain(|i| i != &item);
    //     Ok(())
    // }

    // pub fn has_item(&self, item: &Item) -> bool {
    //     self.items.to_vec().contains(item)
    // }

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
        // vec![Item::new(
        //     "Monopoly".to_owned(),
        //     "Family Game".to_owned(),
        //     crate::models::domain::item::Category::Game,
        //     20f64,
        // )],
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
        assert_eq!(member.name, String::new()); // String::new()
        assert_eq!(member.email, String::new()); // String::new()
        assert_eq!(member.phone_nr, String::new()); // String::new()
        assert_eq!(member.credits, 0.0); // 0.0
    }
}
