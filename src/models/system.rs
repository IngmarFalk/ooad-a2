use super::{contract::Contract, member::Member};

pub trait LendingSystem {
    fn add_member();
    fn remove_member();
}

pub struct System {
    pub active_contracts: Vec<Contract>,
    pub members: Vec<Member>,
}
