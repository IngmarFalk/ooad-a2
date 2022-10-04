use super::{item::Item, member::Member};
use crate::{models::uuid::Uuid, types::Model};
use anyhow::Result;
use std::collections::HashMap;
use thiserror::Error;

pub trait LendingSystem {
    fn get_members(&self) -> Vec<&Member>;
    fn get_member(&self, member: &Member) -> MResult<Member>;
    fn get_member_mut(&mut self, member: &Member) -> MResult<&mut Member>;
    fn add_member(&mut self, member: Member) -> MResult<()>;
    fn remove_member(&mut self, member: &Member) -> MResult<()>;
    fn update_member(&mut self, old_info: &Member, new_info: &Member) -> MResult<()>;
    fn exists_member(&self, member: &Member) -> bool;
    fn get_items(&self) -> Vec<&Item>;
    fn get_items_for_member(&self, member: &Member) -> Vec<&Item>;
    fn add_item(&mut self, item: Item) -> MResult<()>;
    fn remove_item(&mut self, item: &Item) -> MResult<()>;
    fn update_item(&mut self, old_info: &Item, new_info: &Item) -> MResult<()>;
    fn count_items_for_member(&self, member: &Member) -> usize;
}

pub trait LendingSystemState {
    fn add_member(&mut self, member: Member) -> MResult<()>;
    fn remove_member(&mut self, member: &Member) -> MResult<()>;
    fn add_item(&mut self, item: Item) -> MResult<()>;
    fn remove_item(&mut self, item: Item) -> MResult<()>;
}

#[derive(Debug, Clone)]
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

    pub fn build(self) -> Self {
        let Self { members, items } = self;
        Self { members, items }
    }
}

impl LendingSystem for System {
    fn get_members(&self) -> Vec<&Member> {
        self.members
            .iter()
            .map(|entry| entry.1)
            .collect::<Vec<&Member>>()
    }

    fn get_member(&self, member: &Member) -> MResult<Member> {
        match self.members.get(&member.get_uuid()) {
            Some(m) => Ok(m.clone()),
            None => Err(MError::DoesntExist),
        }
    }

    fn get_member_mut(&mut self, member: &Member) -> MResult<&mut Member> {
        match self.members.get_mut(&member.get_uuid()) {
            Some(m) => Ok(m),
            None => Err(MError::DoesntExist),
        }
    }

    fn add_member(&mut self, member: Member) -> MResult<()> {
        if self.exists_member(&member) {
            return Err(MError::AlreadyExists);
        }
        self.members.insert(member.get_uuid().clone(), member);
        Ok(())
    }

    fn remove_member(&mut self, member: &Member) -> MResult<()> {
        if !self.exists_member(member) {
            return Err(MError::DoesntExist);
        }
        self.members.remove(member.get_uuid());
        Ok(())
    }

    fn update_member(&mut self, old_info: &Member, new_info: &Member) -> MResult<()> {
        if !self.exists_member(old_info) {
            return Err(MError::DoesntExist);
        }
        *self.members.get_mut(&old_info.get_uuid()).unwrap() = new_info.clone();
        Ok(())
    }

    fn exists_member(&self, member: &Member) -> bool {
        self.members.iter().any(|entry| {
            let m = entry.1.clone();
            m == member.clone()
        })
    }

    fn get_items(&self) -> Vec<&Item> {
        self.items
            .iter()
            .map(|entry| entry.1)
            .collect::<Vec<&Item>>()
    }

    fn get_items_for_member(&self, member: &Member) -> Vec<&Item> {
        self.get_items()
            .into_iter()
            .filter(|item| {
                if item.get_owner() == member {
                    true
                } else {
                    false
                }
            })
            .collect::<Vec<&Item>>()
    }

    fn add_item(&mut self, item: Item) -> MResult<()> {
        match self.items.insert(item.get_uuid().clone(), item) {
            Some(_) => todo!(),
            None => todo!(),
        }
    }

    fn remove_item(&mut self, item: &Item) -> MResult<()> {
        match self.items.remove(&item.get_uuid().clone()) {
            Some(_) => todo!(),
            None => todo!(),
        }
    }

    fn update_item(&mut self, old_info: &Item, new_info: &Item) -> MResult<()> {
        match self.items.get_mut(old_info.get_uuid()) {
            Some(_) => Ok(*self.items.get_mut(old_info.get_uuid()).unwrap() = new_info.clone()),
            None => Err(MError::DoesntExist),
        }
    }

    fn count_items_for_member(&self, member: &Member) -> usize {
        self.get_items().iter().fold(0, |cnt, item| {
            if item.get_owner() == member {
                cnt + 1
            } else {
                cnt
            }
        })
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

    use super::System;
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
        let turing: Member = Member::new(
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

        match system.remove_member(&turing) {
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
            .name("Monopoly".to_owned())
            .description("A beautiful Family Game.".to_owned())
            .cost_per_day(20f64)
            .category(Category::Game);

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
            .name("Monopoly".to_owned())
            .description("A beautiful Family Game.".to_owned())
            .cost_per_day(20f64)
            .category(Category::Game);

        let mut system = System::new();
        system
            .add_member(turing.clone())
            .expect("failed to add member");

        // let r0 = system.ge(&item);
        // assert_eq!(r0, false);

        let r1 = system.add_item(item.clone());
        assert_eq!(r1, Ok(()));

        // let r2 = match system.get_member(&turing) {
        //     Ok(member) => {
        //         println!("{:?}", member.get_items());
        //         member.has_item(&item)
        //     }
        //     Err(_) => false,
        // };
        // assert_eq!(r2, true);
    }

    #[test]
    fn test_delete_item() {
        let turing = Member::new(
            "Turing".to_owned(),
            "turing@enigma.com".to_owned(),
            "123".to_owned(),
        );

        let item = Item::default()
            .name("Monopoly".to_owned())
            .description("A beautiful Family Game.".to_owned())
            .cost_per_day(20f64)
            .category(Category::Game);

        let mut system = System::new();
        system
            .add_member(turing.clone())
            .expect("failed to add member");

        let r1 = system.add_item(item.clone());
        assert_eq!(r1, Ok(()));

        let r2 = system.remove_item(&item);
        assert_eq!(r2, Ok(()));

        // let r3 = match system.get_member(&turing) {
        //     Ok(member) => {
        //         println!("{:?}", member.get_items());
        //         member.has_item(&item)
        //     }
        //     Err(_) => false,
        // };
        // assert_eq!(r3, false);
    }
}
