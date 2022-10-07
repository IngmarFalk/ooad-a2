use super::{item::Item, member::Member};
use crate::models::uuid::Uuid;
use anyhow::Result;
use shared::{Builder, Model};
use std::collections::HashMap;
use thiserror::Error;

/// All methods for the lending system.
pub trait LendingSystem {
    /// Gets all the members in the system.
    fn get_members(&self) -> Vec<&Member>;
    /// Gets a specific member.
    fn get_member(&self, member: &Member) -> SysResult<Member>;
    /// Returns a mutable version of a member.
    fn get_member_mut(&mut self, member: &Member) -> SysResult<&mut Member>;
    /// Adds a member to the system.
    fn add_member(&mut self, member: Member) -> SysResult<()>;
    /// Removes a member from the system.
    fn remove_member(&mut self, member: &Member) -> SysResult<()>;
    /// updates a member with the new information.
    fn update_member(&mut self, old_info: &Member, new_info: &Member) -> SysResult<()>;
    /// checks if the member passed into function acutally exists in this system.
    fn exists_member(&self, member: &Member) -> bool;
    /// Gets all the items in the system.
    fn get_items(&self) -> Vec<&Item>;
    /// Gets all the items for a specific member.
    fn get_items_for_member(&self, member: &Member) -> Vec<&Item>;
    /// Adds item to the system.
    fn add_item(&mut self, item: Item) -> SysResult<()>;
    /// Removes item from the system.
    fn remove_item(&mut self, item: &Item) -> SysResult<()>;
    /// Updates item with the new information.
    fn update_item(&mut self, old_info: &Item, new_info: &Item) -> SysResult<()>;
    /// Counts the number of items for a certain member.
    fn count_items_for_member(&self, member: &Member) -> usize;
    /// Increments system day counter and calls all required methods to update contracts
    /// items and members information.
    fn incr_time(&mut self);
}

/// system struct.
#[derive(Debug, Clone, Model, Builder)]
pub struct System {
    members: HashMap<Uuid, Member>,
    items: HashMap<Uuid, Item>,
    day: usize,
}

impl System {
    /// Creates a new system instance.
    pub fn new() -> System {
        System {
            members: HashMap::new(),
            items: HashMap::new(),
            day: 0,
        }
    }
}

impl LendingSystem for System {
    fn get_members(&self) -> Vec<&Member> {
        self.members
            .iter()
            .map(|entry| entry.1)
            .collect::<Vec<&Member>>()
    }

    fn get_member(&self, member: &Member) -> SysResult<Member> {
        match self.members.get(member.get_uuid()) {
            Some(m) => Ok(m.clone()),
            None => Err(SysError::DoesntExist),
        }
    }

    fn get_member_mut(&mut self, member: &Member) -> SysResult<&mut Member> {
        match self.members.get_mut(member.get_uuid()) {
            Some(m) => Ok(m),
            None => Err(SysError::DoesntExist),
        }
    }

    fn add_member(&mut self, member: Member) -> SysResult<()> {
        if self.exists_member(&member) {
            return Err(SysError::AlreadyExists);
        }
        self.members.insert(member.get_uuid().clone(), member);
        Ok(())
    }

    fn remove_member(&mut self, member: &Member) -> SysResult<()> {
        if !self.exists_member(member) {
            return Err(SysError::DoesntExist);
        }
        self.members.remove(member.get_uuid());
        Ok(())
    }

    fn update_member(&mut self, old_info: &Member, new_info: &Member) -> SysResult<()> {
        if !self.exists_member(old_info) {
            return Err(SysError::DoesntExist);
        }
        *self.members.get_mut(old_info.get_uuid()).unwrap() = new_info.clone();
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
            .filter(|item| item.get_owner() == member)
            .collect::<Vec<&Item>>()
    }

    fn add_item(&mut self, item: Item) -> SysResult<()> {
        match self.items.insert(item.get_uuid().clone(), item) {
            Some(_) => Err(SysError::AlreadyExists),
            None => Ok(()),
        }
    }

    fn remove_item(&mut self, item: &Item) -> SysResult<()> {
        match self.items.remove(&item.get_uuid().clone()) {
            Some(_) => Ok(()),
            None => Err(SysError::CannotDelete),
        }
    }

    fn update_item(&mut self, old_info: &Item, new_info: &Item) -> SysResult<()> {
        match self.items.get_mut(old_info.get_uuid()) {
            Some(_) => {
                *self.items.get_mut(old_info.get_uuid()).unwrap() = new_info.clone();
                Ok(())
            }
            None => Err(SysError::CannotUpdate),
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

    fn incr_time(&mut self) {
        self.day += 1;
    }
}

/// System error result.
pub type SysResult<T> = Result<T, SysError>;

/// System Error.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum SysError {
    /// If an object already exists.
    AlreadyExists,
    /// If an object doesnt exists.
    DoesntExist,
    /// Cannot insert an object.
    CannotInsert,
    /// Cannot delete an object.
    CannotDelete,
    /// Cannot update an object.
    CannotUpdate,
}

impl std::fmt::Display for SysError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            SysError::AlreadyExists => f.write_str("This object already exists."),
            SysError::DoesntExist => f.write_str("This object doesnt exists."),
            SysError::CannotInsert => f.write_str("There was an problem inserting this object."),
            SysError::CannotDelete => f.write_str("There was a problem deleting this object."),
            SysError::CannotUpdate => f.write_str("There was a problem updating this object."),
        }
    }
}

// #[cfg(test)]
// mod system_tests {
//     use crate::models::domain::{
//         item::{Category, Item},
//         member::Member,
//     };

//     use super::System;
//     use super::*;

//     #[test]
//     fn test_add_member() {
//         let allan = Member::new(
//             "Allan".to_owned(),
//             "allan@enigma.com".to_owned(),
//             "123456".to_owned(),
//         );
//         let turing1 = Member::new(
//             "Turing".to_owned(),
//             "allan@enigma.com".to_owned(),
//             "123".to_owned(),
//         );
//         let turing2 = Member::new(
//             "Turing".to_owned(),
//             "turing@enigma.com".to_owned(),
//             "123456".to_owned(),
//         );
//         let turing3 = Member::new(
//             "Turing".to_owned(),
//             "turing@enigma.com".to_owned(),
//             "123".to_owned(),
//         );

//         let mut system = System::new();
//         let r1 = system.add_member(allan);
//         assert_eq!(r1, Ok(()));
//         println!("1");

//         let r2 = system.add_member(turing1);
//         assert_eq!(r2, Err(SysError::AlreadyExists));
//         println!("2");

//         let r3 = system.add_member(turing2);
//         assert_eq!(r3, Err(SysError::AlreadyExists));
//         println!("3");

//         let r4 = system.add_member(turing3);
//         assert_eq!(r4, Ok(()));
//         println!("4");
//     }

//     #[test]
//     fn test_exists_member() {
//         let turing: Member = Member::new(
//             "Turing".to_owned(),
//             "turing@enigma.com".to_owned(),
//             "123".to_owned(),
//         );
//         let allan = Member::new(
//             "Allan".to_owned(),
//             "allan@enigma.com".to_owned(),
//             "123567".to_owned(),
//         );

//         let mut system = System::new();
//         system
//             .add_member(turing.clone())
//             .expect("failed to add member");

//         let r1 = system.exists_member(&turing);
//         assert_eq!(r1, true);

//         let r2 = system.exists_member(&allan);
//         assert_eq!(r2, false);
//     }

//     #[test]
//     fn test_remove_member() {
//         let turing = Member::new(
//             "Turing".to_owned(),
//             "turing@enigma.com".to_owned(),
//             "123".to_owned(),
//         );

//         let mut system = System::new();
//         system
//             .add_member(turing.clone())
//             .expect("failed to add member");

//         let r1 = system.exists_member(&turing);
//         assert_eq!(r1, true);

//         match system.remove_member(&turing) {
//             Ok(_) => {}
//             Err(_) => assert!(false),
//         }

//         let r2 = system.exists_member(&turing);
//         assert_eq!(r2, false);
//     }

//     #[test]
//     fn test_create_item() {
//         let turing = Member::new(
//             "Turing".to_owned(),
//             "turing@enigma.com".to_owned(),
//             "123".to_owned(),
//         );

//         let item = Item::default()
//             .name("Monopoly".to_owned())
//             .description("A beautiful Family Game.".to_owned())
//             .cost_per_day(20f64)
//             .category(Category::Game);

//         let mut system = System::new();
//         system
//             .add_member(turing.clone())
//             .expect("failed to add member");

//         let r1 = system.add_item(item);
//         assert_eq!(r1, Ok(()))
//     }

//     #[test]
//     fn test_exists_item() {
//         let turing = Member::new(
//             "Turing".to_owned(),
//             "turing@enigma.com".to_owned(),
//             "123".to_owned(),
//         );

//         let item = Item::default()
//             .name("Monopoly".to_owned())
//             .description("A beautiful Family Game.".to_owned())
//             .cost_per_day(20f64)
//             .category(Category::Game);

//         let mut system = System::new();
//         system
//             .add_member(turing.clone())
//             .expect("failed to add member");

//         // let r0 = system.ge(&item);
//         // assert_eq!(r0, false);

//         let r1 = system.add_item(item.clone());
//         assert_eq!(r1, Ok(()));

//         // let r2 = match system.get_member(&turing) {
//         //     Ok(member) => {
//         //         println!("{:?}", member.get_items());
//         //         member.has_item(&item)
//         //     }
//         //     Err(_) => false,
//         // };
//         // assert_eq!(r2, true);
//     }

//     #[test]
//     fn test_delete_item() {
//         let turing = Member::new(
//             "Turing".to_owned(),
//             "turing@enigma.com".to_owned(),
//             "123".to_owned(),
//         );

//         let item = Item::default()
//             .name("Monopoly".to_owned())
//             .description("A beautiful Family Game.".to_owned())
//             .cost_per_day(20f64)
//             .category(Category::Game);

//         let mut system = System::new();
//         system
//             .add_member(turing.clone())
//             .expect("failed to add member");

//         let r1 = system.add_item(item.clone());
//         assert_eq!(r1, Ok(()));

//         let r2 = system.remove_item(&item);
//         assert_eq!(r2, Ok(()));

//         // let r3 = match system.get_member(&turing) {
//         //     Ok(member) => {
//         //         println!("{:?}", member.get_items());
//         //         member.has_item(&item)
//         //     }
//         //     Err(_) => false,
//         // };
//         // assert_eq!(r3, false);
//     }
// }
