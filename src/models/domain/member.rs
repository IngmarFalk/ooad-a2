use super::{item::Item, Data};
use crate::models::domain::FromMap;
use crate::models::{
    cvec::CVec,
    system::{MError, MResult},
    uuid::Uuid,
};
use chrono::{self, Local};
use derive_getters::{Dissolve, Getters};
use prettytable::{row, Row, Table};
use shared::{CFromMap, CFromStr, CToMap, CToStr};
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
#[derive(Clone, Debug, Getters, Dissolve, CFromStr, CToStr, CFromMap, CToMap)]
#[dissolve(rename = "unpack")]
pub struct Member {
    #[getter(rename = "get_name")]
    name: String,
    #[getter(rename = "get_email")]
    email: String,
    #[getter(rename = "get_phone_nr")]
    phone_nr: String,
    #[getter(rename = "get_credits")]
    credits: f64,
    #[getter(rename = "get_day_of_creation")]
    day_of_creation: chrono::DateTime<Local>,
    #[getter(rename = "get_uuid")]
    uuid: Uuid,
    #[getter(rename = "get_items")]
    items: CVec<Item>,
}

impl Member {
    pub fn new(name: String, email: String, phone_nr: String) -> Member {
        Member {
            uuid: Uuid::member(),
            day_of_creation: chrono::offset::Local::now(),
            credits: 0f64,
            items: CVec::new(),
            name,
            email,
            phone_nr,
        }
    }

    pub fn add_item(&mut self, item: Item) -> MResult<()> {
        let exists = self.items.iter().any(|e| e.get_uuid() == item.get_uuid());
        if exists {
            return Err(MError::AlreadyExists);
        }
        self.items.push(item);
        self.add_credits(100f64).unwrap();
        Ok(())
    }

    pub fn remove_item(&mut self, item: Item) -> MResult<()> {
        if !self.has_item(&item) {
            return Err(MError::DoesntExist);
        }
        self.items.to_vec().retain(|i| i != &item);
        Ok(())
    }

    pub fn has_item(&self, item: &Item) -> bool {
        self.items.to_vec().contains(item)
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

    pub fn new2(
        data: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    ) -> Self {
        Self {
            name: data.get("name").unwrap().parse::<String>().unwrap(),
            email: data.get("email").unwrap().parse::<String>().unwrap(),
            phone_nr: data.get("phone_nr").unwrap().parse::<String>().unwrap(),
            credits: data.get("credits").unwrap().parse::<f64>().unwrap(),
            day_of_creation: data
                .get("day_of_creation")
                .unwrap()
                .parse::<chrono::DateTime<Local>>()
                .unwrap(),
            uuid: data.get("uuid").unwrap().parse::<Uuid>().unwrap(),
            items: data.get("items").unwrap().parse::<CVec<Item>>().unwrap(),
        }
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

impl Data for Member {
    fn to_row(&self) -> Row {
        row![
            self.name.clone(),
            self.email.clone(),
            self.phone_nr.clone(),
            self.uuid.to_string(),
            self.credits.clone().to_string(),
            self.items.to_vec().len().to_string(),
        ]
    }

    fn head(&self) -> Vec<String> {
        vec![
            "Name".to_owned(),
            "Email".to_owned(),
            "Phone Number".to_owned(),
            "Uuid".to_owned(),
            "Credits".to_owned(),
            "Items".to_owned(),
        ]
    }

    fn head_allowed_mutable(&self) -> Vec<String> {
        vec!["name".to_owned(), "email".to_owned(), "phone_nr".to_owned()]
    }

    fn to_table(&self) -> prettytable::Table {
        let mut table = Table::new();
        table.add_row(Row::from(self.head()));
        table.add_row(self.to_row());
        table
    }
}

impl Default for Member {
    fn default() -> Self {
        Self {
            name: String::new(),
            email: String::new(),
            phone_nr: String::new(),
            credits: 0f64,
            day_of_creation: chrono::offset::Local::now(),
            uuid: Uuid::member(),
            items: CVec::new(),
        }
    }
}

impl PartialEq for Member {
    fn eq(&self, other: &Self) -> bool {
        self.email == other.email || self.phone_nr == other.phone_nr || self.uuid == other.uuid
    }

    fn ne(&self, other: &Self) -> bool {
        self.email != other.email && self.phone_nr != other.phone_nr && self.uuid == other.uuid
    }
}

impl Eq for Member {}

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
        assert_eq!(bob.items.to_vec(), vec![])
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
