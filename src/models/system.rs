use thiserror::Error;

use super::{item::Item, member::Member};

pub trait LendingSystem {
    fn add_member(&mut self, member: Member) -> MResult<()>;
    fn remove_member(&mut self, member: Member) -> MResult<()>;
    fn exists_member(&self, member: &Member) -> bool;
    fn create_item(&self, member: Member, item: Item) -> MResult<()>;
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

    pub fn add_member(member: Member) -> MResult<()> {
        println!("{:?}", member);
        if true {
            return Err(MError::AlreadyExists);
        }
        Ok(())
    }
}

impl LendingSystem for System {
    fn add_member(&mut self, member: Member) -> MResult<()> {
        let exists: bool = self.exists_member(&member);
        if exists {
            return Err(MError::AlreadyExists);
        }
        self.members.push(member);
        Ok(())
    }

    fn remove_member(&mut self, member: Member) -> MResult<()> {
        println!("{:?}", member);
        if true {
            return Err(MError::DoesntExist);
        }
        Ok(())
    }

    fn exists_member(&self, member: &Member) -> bool {
        println!("{:?}", &self.members);
        for m in self.members.iter() {
            if m.email == member.email || m.phone_nr == member.phone_nr {
                return true;
            }
        }
        false
        // self.members
        //     .iter()
        //     .any(|m| m.email == member.email || m.phone_nr == member.phone_nr)
    }

    fn create_item(&self, member: Member, item: Item) -> MResult<()> {
        println!("{:?}", member);
        println!("{:?}", item);

        if true {
            return Err(MError::AlreadyExists);
        }
        Ok(())
    }

    fn delete_item(&self, member: Member, item: Item) -> MResult<()> {
        println!("{:?}", member);
        println!("{:?}", item);

        if true {
            return Err(MError::DoesntExist);
        }
        Ok(())
    }
}

type MResult<T> = Result<T, MError>;

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

// #[derive(Debug, Error)]
// #[error("A member with the same email/phone number/id already exists")]
// pub struct MemberAlreadyExists;

#[cfg(test)]
mod system_tests {
    use crate::models::member::Member;

    use super::*;

    #[test]
    fn test_exists_member() {
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

        let members: Vec<&Member> = vec![&allan, &turing1, &turing2, &turing3];
        let mut system = System::new();
        let r1 = system.add_member(allan);
        assert_eq!(r1, Ok(()));

        let r2 = system.add_member(turing1);
        assert_eq!(r1, Err(MError::AlreadyExists));

        // let r3 = system.add_member(turing2);
        // assert_eq!(r1, Err(MError::AlreadyExists));

        // let r4 = system.add_member(turing3);
        // assert_eq!(r1, Ok(()));
    }
}
