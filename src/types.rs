use std::collections::HashMap;

use crate::models::domain::{contract::Contract, item::Item, member::Member};

pub type MembersList = Vec<Member>;
pub type ItemsList = Vec<Item>;
pub type ContractsList = Vec<Contract>;
pub type BufferVec = Vec<(String, String)>;
pub type StringMap = HashMap<String, String>;

pub trait Model {}
pub trait View {}
pub trait Controller {}
