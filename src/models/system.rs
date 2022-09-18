use anyhow::Result;
use thiserror::Error;

use super::{item::Item, member::Member};

pub trait LendingSystem {
    fn add_member(&mut self, member: Member) -> MResult<()>;
    fn remove_member(&mut self, member: Member) -> MResult<()>;
    fn exists_member(&self, member: &Member) -> bool;
    fn create_item(&mut self, member: Member, item: Item) -> MResult<()>;
    fn delete_item(&self, member: Member, item: Item) -> MResult<()>;
}

#[derive(Debug)]
pub struct System {
    pub members: Vec<Member>,
}

impl System {
    pub fn new() -> System {
        System { members: vec![] }
    }

    pub fn members(&mut self, members: Vec<Member>) -> &mut Self {
        self.members = members;
        self
    }

    pub fn get_member(&self, member: Member) -> MResult<Member> {
        let idx = self.members.iter().position(|e| e.uuid == member.uuid);
        match idx {
            Some(i) => Ok(self.members[i].clone()),
            None => Err(MError::DoesntExist),
        }
    }

    fn get_member_idx(&self, member: Member) -> MResult<usize> {
        let idx = self.members.iter().position(|e| e.uuid == member.uuid);
        match idx {
            Some(i) => Ok(i),
            None => Err(MError::DoesntExist),
        }
    }
}

impl LendingSystem for System {
    fn add_member(&mut self, member: Member) -> MResult<()> {
        if self.exists_member(&member) {
            return Err(MError::AlreadyExists);
        }
        self.members.push(member);
        Ok(())
    }

    fn remove_member(&mut self, member: Member) -> MResult<()> {
        if !self.exists_member(&member) {
            return Err(MError::DoesntExist);
        }
        Ok(())
    }

    fn exists_member(&self, member: &Member) -> bool {
        self.members
            .iter()
            .any(|m| m.email == member.email || m.phone_nr == member.phone_nr)
    }

    fn create_item(&mut self, member: Member, item: Item) -> MResult<()> {
        let member = self.get_member_idx(member);
        match member {
            Ok(i) => self.members[i].add_item(item),
            Err(err) => return Err(err),
        }
    }

    fn delete_item(&self, member: Member, item: Item) -> MResult<()> {
        todo!()
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
    use crate::models::member::Member;

    use super::*;

    #[test]
    fn test_add_member() {
        let allan = Member::new(
            "Allan".to_owned(),
            "allan@enigma.com".to_owned(),
            "123456".to_owned(),
            500f64,
            vec![],
        );
        let turing1 = Member::new(
            "Turing".to_owned(),
            "allan@enigma.com".to_owned(),
            "123".to_owned(),
            500f64,
            vec![],
        );
        let turing2 = Member::new(
            "Turing".to_owned(),
            "turing@enigma.com".to_owned(),
            "123456".to_owned(),
            500f64,
            vec![],
        );
        let turing3 = Member::new(
            "Turing".to_owned(),
            "turing@enigma.com".to_owned(),
            "123".to_owned(),
            500f64,
            vec![],
        );

        let mut system = System::new();
        let r1 = system.add_member(allan);
        assert_eq!(r1, Ok(()));

        let r2 = system.add_member(turing1);
        assert_eq!(r2, Err(MError::AlreadyExists));

        let r3 = system.add_member(turing2);
        assert_eq!(r3, Err(MError::AlreadyExists));

        let r4 = system.add_member(turing3);
        assert_eq!(r4, Ok(()));
    }

    #[test]
    fn test_exists_member() {
        let turing = Member::new(
            "Turing".to_owned(),
            "turing@enigma.com".to_owned(),
            "123".to_owned(),
            500f64,
            vec![],
        );
        let allan = Member::new(
            "Allan".to_owned(),
            "allan@enigma.com".to_owned(),
            "123567".to_owned(),
            500f64,
            vec![],
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
}