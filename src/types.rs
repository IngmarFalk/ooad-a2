use crate::models::domain::{contract::Contract, item::Item, member::Member};

pub type MembersList = Vec<Member>;
pub type ItemsList = Vec<Item>;
pub type ContractsList = Vec<Contract>;

pub trait Model {}
pub trait View {}
pub trait Controller {}
