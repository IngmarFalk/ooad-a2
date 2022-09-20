use std::collections::HashMap;

use crate::models::{
    system::{MError, MResult},
    uuid::Uuid,
};
use chrono::{self, Local};
use prettytable::{row, Row, Table};
use thiserror::Error;

use super::{item::Item, Data, FromMap, ToMap};

pub trait MemberValidation {
    fn validate_id(&self) -> bool;
    fn validate_phone_nr(&self) -> bool;
    fn validate_email(&self) -> bool;
}

#[derive(Default, Clone, Debug)]
pub struct Member {
    pub name: String,
    pub email: String,
    pub phone_nr: String,
    pub credits: f64,
    pub day_of_creation: chrono::DateTime<Local>,
    pub uuid: Uuid,
    pub items: Vec<Item>,
}

impl Member {
    pub fn new(name: String, email: String, phone_nr: String) -> Member {
        Member {
            uuid: Uuid::member(),
            day_of_creation: chrono::offset::Local::now(),
            credits: 0f64,
            items: vec![],
            name,
            email,
            phone_nr,
        }
    }

    pub fn name(&mut self, name: String) -> &mut Member {
        self.name = name;
        self
    }

    pub fn email(&mut self, email: String) -> &mut Member {
        self.email = email;
        self
    }

    pub fn phone_nr(&mut self, phone_nr: String) -> &mut Member {
        self.phone_nr = phone_nr;
        self
    }

    pub fn add_item(&mut self, item: Item) -> MResult<()> {
        let exists = self.items.iter().any(|e| e.uuid == item.uuid);
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
        self.items.retain(|i| i != &item);
        Ok(())
    }

    pub fn has_item(&self, item: &Item) -> bool {
        self.items.contains(item)
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
        // TODO : Find out if members can have negative credit
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

impl FromMap for Member {
    fn from_partial_map(data: crate::types::StringMap) -> Self {
        todo!()
    }

    fn from_complete_map(data: crate::types::StringMap) -> Self {
        todo!()
    }

    fn copy_with(&self, data: crate::types::StringMap) -> Self {
        todo!()
    }
}

impl ToMap for Member {
    fn to_map(&self) -> crate::types::StringMap {
        todo!()
    }

    fn to_allowed_mutable_map(&self) -> crate::types::StringMap {
        todo!()
    }

    fn to_buffers_map(&self) -> crate::types::StringMap {
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
            self.items.len().to_string(),
        ]
    }

    fn head(&self) -> Vec<&str> {
        vec!["Name", "Email", "Phone Number", "Uuid", "Credits", "Items"]
    }

    fn head_allowed_mutable(&self) -> Vec<&str> {
        vec!["name", "email", "phone_nr"]
    }

    fn to_table(&self) -> prettytable::Table {
        let mut table = Table::new();
        table.add_row(Row::from(self.head()));
        table.add_row(self.to_row());
        table
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
    use crate::models::domain::item::Item;

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
        assert_eq!(bob.items, vec![])
    }

    fn test_builder_creation() {
        let name = "Bob".to_owned();
        let email = "bob@gmail.com".to_owned();
        let phone_nr = "40123456789".to_owned();
        let credits = 200f64;
        let bob = Member::default().name(name).email(email).phone_nr(phone_nr);
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
