use crate::models::{
    system::{MError, MResult},
    uuid::Uuid,
};
use chrono::{self, Local};
use thiserror::Error;

use super::{item::Item, ToRow};

pub trait MemberValidation {
    fn validate_id() -> bool;
    fn validate_phone_nr() -> bool;
    fn validate_email() -> bool;
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
        Ok(())
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

/// ! This probably does not belong here since it specifies 'ui/ux' elements.
impl std::fmt::Display for Member {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut items_str = String::from("Items:");
        for item in self.items.iter() {
            let formatted = format!("\n\t{}", item);
            items_str.push_str(&formatted);
        }
        if self.items.len() == 0 {
            items_str.push_str(" []")
        }
        f.write_fmt(format_args!(
            "{}\n{}\n{}\n{}\n{}",
            self.name, self.email, self.phone_nr, self.credits, items_str
        ))
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

impl ToRow for Member {
    fn to_row(&self) -> Vec<String> {
        vec![
            self.uuid.to_string(),
            self.name.clone(),
            self.email.clone(),
            self.phone_nr.clone(),
            self.credits.clone().to_string(),
            self.items.len().to_string(),
        ]
    }
}

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
        let credits = 200f64;
        let bob = Member::new(
            name,
            email,
            phone_nr,
            credits,
            vec![Item::new(
                crate::models::domain::item::Category::Game,
                "Monopoly".to_owned(),
                "Family Game".to_owned(),
                None,
                chrono::offset::Local::now(),
                20f64,
            )],
        );
        assert_eq!(bob.name, "Bob".to_owned());
        assert_eq!(bob.email, "bob@gmail.com".to_owned());
        assert_eq!(bob.phone_nr, "40123456789".to_owned());
        assert_eq!(bob.credits, 200f64);
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
