use crate::models::uuid::Uuid;
use chrono::{self, Local};
use thiserror::Error;

use super::item::Item;

pub trait MemberValidation {
    fn validate_id() -> bool;
    fn validate_phone_nr() -> bool;
    fn validate_email() -> bool;
}

#[derive(Default, Clone, Debug)]
pub struct Member {
    pub credits: f64,
    pub day_of_creation: chrono::DateTime<Local>,
    pub email: String,
    pub name: String,
    pub phone_nr: String,
    pub uuid: Uuid,
    items: Vec<Item>,
}

impl Member {
    pub fn new(
        name: String,
        email: String,
        phone_nr: String,
        credits: f64,
        items: Vec<Item>,
    ) -> Member {
        Member {
            uuid: Uuid::new(),
            day_of_creation: chrono::offset::Local::now(),
            name,
            email,
            phone_nr,
            credits,
            items,
        }
    }

    pub fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }

    pub fn remove_item(&mut self, item: Item) -> anyhow::Result<()> {
        let idx = self.items.iter().position(|e| e == &item);
        match idx {
            Some(i) => {
                self.items.remove(i);
                Ok(())
            }
            None => todo!(),
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
        // TODO : Find out if members can have negative credit
        self.credits -= credits;
        Ok(())
    }
}

impl MemberValidation for Member {
    fn validate_id() -> bool {
        todo!()
    }

    fn validate_phone_nr() -> bool {
        todo!()
    }

    fn validate_email() -> bool {
        todo!()
    }
}

#[derive(Debug, Error)]
#[error("Tried adding/deducing a negative amount to credits")]
pub struct NegativeCreditInput;

#[cfg(test)]
mod member_test {
    use super::Member;
    use crate::models::item::Item;

    #[test]
    fn test_explicit_creation() {
        let name = "Bob".to_owned();
        let email = "bob@gmail.com".to_owned();
        let phone_nr = "40123456789".to_owned();
        let credits = 200f64;
        let bob = Member::new(name, email, phone_nr, credits, vec![Item::default()]);
        assert_eq!(bob.name, "Bob".to_owned());
        assert_eq!(bob.email, "bob@gmail.com".to_owned());
        assert_eq!(bob.phone_nr, "40123456789".to_owned());
        assert_eq!(bob.credits, 200f64);
    }

    #[test]
    fn test_default_creation() {
        let member = Member::default();
        assert_eq!(member.name, String::default()); // String::new()
        assert_eq!(member.email, String::default()); // String::new()
        assert_eq!(member.phone_nr, String::default()); // String::new()
        assert_eq!(member.credits, f64::default()); // 0.0
    }
}
