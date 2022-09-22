use anyhow::Result;
use std::collections::HashMap;
use thiserror::Error;

use crate::types::Model;

use super::{
    domain::{item::Item, member::Member},
    uuid::Uuid,
};

pub trait LendingSystem {
    fn get_member(&self, member: &Member) -> MResult<Member>;
    fn get_member_mut(&mut self, member: &Member) -> MResult<&mut Member>;
    fn add_member(&mut self, member: Member) -> MResult<()>;
    fn remove_member(&mut self, member: Member) -> MResult<()>;
    fn exists_member(&self, member: &Member) -> bool;
    fn add_item(&mut self, item: Item) -> MResult<()>;
    fn remove_item(&mut self, item: Item) -> MResult<()>;
}

/// TODO : Create a cache HashMap in system where whenever we add an item
/// TODO : or contract we store a chain of keys as a track to the exact
/// TODO : item or contract. That way we can look up a contract id
/// TODO : in this map via a single operation.

#[derive(Debug)]
pub struct System {
    members: HashMap<Uuid, Member>,
    items: HashMap<Uuid, Item>,
}

impl Model for System {}

impl System {
    pub fn new() -> System {
        System {
            members: HashMap::new(),
            items: HashMap::new(),
        }
    }

    pub fn members(&mut self, members: HashMap<Uuid, Member>) -> &mut Self {
        self.members = members;
        self
    }

    pub fn items(&mut self, items: HashMap<Uuid, Item>) -> &mut Self {
        self.items = items;
        self
    }
}

impl LendingSystem for System {
    fn get_member(&self, member: &Member) -> MResult<Member> {
        match self.members.get(&member.uuid()) {
            Some(m) => Ok(m.clone()),
            None => Err(MError::DoesntExist),
        }
    }

    fn get_member_mut(&mut self, member: &Member) -> MResult<&mut Member> {
        match self.members.get_mut(&member.uuid()) {
            Some(m) => Ok(m),
            None => Err(MError::DoesntExist),
        }
    }

    fn add_member(&mut self, member: Member) -> MResult<()> {
        if self.exists_member(&member) {
            return Err(MError::AlreadyExists);
        }
        self.members.insert(member.uuid().clone(), member);
        Ok(())
    }

    fn remove_member(&mut self, member: Member) -> MResult<()> {
        if !self.exists_member(&member) {
            return Err(MError::DoesntExist);
        }
        self.members.remove(&member.uuid());
        Ok(())
    }

    fn exists_member(&self, member: &Member) -> bool {
        self.members.iter().any(|entry| {
            let m = entry.1.clone();
            m == member.clone()
        })
    }

    fn add_item(&mut self, item: Item) -> MResult<()> {
        let res = match self.get_member_mut(&item.get_owner()) {
            Ok(m) => match m.add_item(item.clone()) {
                Ok(_) => Ok(()),
                Err(err) => Err(err),
            },
            Err(err) => return Err(err),
        };
        match res {
            Ok(_) => match self.items.insert(item.get_uuid().clone(), item) {
                Some(_) => todo!(),
                None => todo!(),
            },
            Err(err) => Err(err),
        }
    }

    fn remove_item(&mut self, item: Item) -> MResult<()> {
        let res = match self.get_member_mut(&item.get_owner()) {
            Ok(m) => m.remove_item(item.clone()),
            Err(err) => return Err(err),
        };
        match res {
            Ok(_) => match self.items.remove(&item.get_uuid().clone()) {
                Some(_) => todo!(),
                None => todo!(),
            },
            Err(_) => todo!(),
        }
    }
}

pub type MResult<T> = Result<T, MError>;

#[derive(Debug, Error, PartialEq)]
pub enum MError {
    AlreadyExists,
    DoesntExist,
    ReadingFile,
    WritingFile,
}

impl std::fmt::Display for MError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            MError::AlreadyExists => f.write_str("There already exists an element with this info"),
            MError::DoesntExist => f.write_str("There doesnt exists an element with this info"),
            MError::ReadingFile => f.write_str("There was a problem reading from a file"),
            MError::WritingFile => f.write_str("There was a problem writing to a file"),
        }
    }
}

#[cfg(test)]
mod system_tests {
    use crate::models::domain::{
        item::{Category, Item},
        member::Member,
    };

    use super::*;

    #[test]
    fn test_add_member() {
        let allan = Member::new(
            "Allan".to_owned(),
            "allan@enigma.com".to_owned(),
            "123456".to_owned(),
        );
        let turing1 = Member::new(
            "Turing".to_owned(),
            "allan@enigma.com".to_owned(),
            "123".to_owned(),
        );
        let turing2 = Member::new(
            "Turing".to_owned(),
            "turing@enigma.com".to_owned(),
            "123456".to_owned(),
        );
        let turing3 = Member::new(
            "Turing".to_owned(),
            "turing@enigma.com".to_owned(),
            "123".to_owned(),
        );

        let mut system = System::new();
        let r1 = system.add_member(allan);
        assert_eq!(r1, Ok(()));
        println!("1");

        let r2 = system.add_member(turing1);
        assert_eq!(r2, Err(MError::AlreadyExists));
        println!("2");

        let r3 = system.add_member(turing2);
        assert_eq!(r3, Err(MError::AlreadyExists));
        println!("3");

        let r4 = system.add_member(turing3);
        assert_eq!(r4, Ok(()));
        println!("4");
    }

    #[test]
    fn test_exists_member() {
        let turing = Member::new(
            "Turing".to_owned(),
            "turing@enigma.com".to_owned(),
            "123".to_owned(),
        );
        let allan = Member::new(
            "Allan".to_owned(),
            "allan@enigma.com".to_owned(),
            "123567".to_owned(),
        );

        let mut system = System::new();
        system
            .add_member(turing.clone())
            .expect("failed to add member");

        let r1 = system.exists_member(&turing);
        assert_eq!(r1, true);

        let r2 = system.exists_member(&allan);
        assert_eq!(r2, false);
    }

    #[test]
    fn test_remove_member() {
        let turing = Member::new(
            "Turing".to_owned(),
            "turing@enigma.com".to_owned(),
            "123".to_owned(),
        );

        let mut system = System::new();
        system
            .add_member(turing.clone())
            .expect("failed to add member");

        let r1 = system.exists_member(&turing);
        assert_eq!(r1, true);

        match system.remove_member(turing.clone()) {
            Ok(_) => {}
            Err(_) => assert!(false),
        }

        let r2 = system.exists_member(&turing);
        assert_eq!(r2, false);
    }

    #[test]
    fn test_create_item() {
        let turing = Member::new(
            "Turing".to_owned(),
            "turing@enigma.com".to_owned(),
            "123".to_owned(),
        );

        let item = Item::default()
            .with_name("Monopoly".to_owned())
            .with_description("A beautiful Family Game.".to_owned())
            .with_cost_per_day(20f64)
            .with_category(Category::Game);

        let mut system = System::new();
        system
            .add_member(turing.clone())
            .expect("failed to add member");

        let r1 = system.add_item(item);
        assert_eq!(r1, Ok(()))
    }

    #[test]
    fn test_exists_item() {
        let turing = Member::new(
            "Turing".to_owned(),
            "turing@enigma.com".to_owned(),
            "123".to_owned(),
        );

        let item = Item::default()
            .with_name("Monopoly".to_owned())
            .with_description("A beautiful Family Game.".to_owned())
            .with_cost_per_day(20f64)
            .with_category(Category::Game);

        let mut system = System::new();
        system
            .add_member(turing.clone())
            .expect("failed to add member");

        let r0 = turing.has_item(&item);
        assert_eq!(r0, false);

        let r1 = system.add_item(item.clone());
        assert_eq!(r1, Ok(()));

        let r2 = match system.get_member(&turing) {
            Ok(member) => {
                println!("{:?}", member.items());
                member.has_item(&item)
            }
            Err(_) => false,
        };
        assert_eq!(r2, true);
    }

    #[test]
    fn test_delete_item() {
        let turing = Member::new(
            "Turing".to_owned(),
            "turing@enigma.com".to_owned(),
            "123".to_owned(),
        );

        let item = Item::default()
            .with_name("Monopoly".to_owned())
            .with_description("A beautiful Family Game.".to_owned())
            .with_cost_per_day(20f64)
            .with_category(Category::Game);

        let mut system = System::new();
        system
            .add_member(turing.clone())
            .expect("failed to add member");

        let r1 = system.add_item(item.clone());
        assert_eq!(r1, Ok(()));

        let r2 = system.remove_item(item.clone());
        assert_eq!(r2, Ok(()));

        let r3 = match system.get_member(&turing) {
            Ok(member) => {
                println!("{:?}", member.items());
                member.has_item(&item)
            }
            Err(_) => false,
        };
        assert_eq!(r3, false);
    }
}
