use crate::models::uuid::Uuid;
use chrono::{self, Local};

use super::item::Item;

pub trait MemberValidation {
    fn validate_id() -> bool;
    fn validate_phone_nr() -> bool;
    fn validate_email() -> bool;
}

#[derive(Default, Clone)]
pub struct Member {
    pub credits: f64,
    pub day_of_creation: chrono::DateTime<Local>,
    pub email: String,
    pub name: String,
    pub phone_nr: String,
    items: Vec<Item>,
    uuid: Uuid,
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

    pub fn name(&mut self, name: String) -> Member {
        self.name = name;
        self.clone()
    }

    pub fn email(&mut self, email: String) -> Member {
        self.email = email;
        self.clone()
    }

    pub fn phone_nr(&mut self, phone_nr: String) -> Member {
        self.phone_nr = phone_nr;
        self.clone()
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

#[cfg(test)]
mod member_test {
    use super::Member;
    use crate::models::category::Category;
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

    fn test_default_creation() {
        let member = Member::default();
        assert_eq!(member.name, String::default()); // String::new()
        assert_eq!(member.email, String::default()); // String::new()
        assert_eq!(member.phone_nr, String::default()); // String::new()
        assert_eq!(member.credits, f64::default()); // 0.0
    }
}
