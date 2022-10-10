use super::contract::Status;
use super::system::{SysError, SysResult};
use super::{contract::Contract, member::Member, FromMap};
use crate::models::uuid::Uuid;
use crate::models::vec_wrapper::VecWrapper;
use derive_getters::{Dissolve, Getters};
use shared::{
    Builder, DeriveData, DeriveFromMap, DeriveFromStr, DerivePartialEq, DeriveToMap, DeriveToStr,
    Model,
};
use std::str::FromStr;
use std::{collections::HashMap, fmt::Display};

/// The category an Item can have.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Category {
    /// A tool.
    Tool,
    /// A vehicle.
    Vehicle,
    /// A game.
    Game,
    /// A toy.
    Toy,
    /// Sport
    Sport,
    /// Anything else.
    #[default]
    Other,
}

impl FromStr for Category {
    type Err = SysError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "tool" => Ok(Category::Tool),
            "vehicle" => Ok(Category::Vehicle),
            "game" => Ok(Category::Game),
            "toy" => Ok(Category::Toy),
            "sport" => Ok(Category::Sport),
            _ => Ok(Category::Other),
        }
    }
}

impl Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Category::Tool => f.write_str("Tool"),
            Category::Vehicle => f.write_str("Vehicle"),
            Category::Game => f.write_str("Game"),
            Category::Toy => f.write_str("Toy"),
            Category::Sport => f.write_str("Sport"),
            Category::Other => f.write_str("Other"),
        }
    }
}

/// Item.
#[derive(
    Debug,
    Clone,
    Getters,
    Dissolve,
    Builder,
    DeriveFromStr,
    DeriveToStr,
    DeriveFromMap,
    DeriveToMap,
    DeriveData,
    DerivePartialEq,
    Model,
)]
#[dissolve(rename = "unpack")]
pub struct Item {
    #[getter(rename = "get_name")]
    name: String,

    #[getter(rename = "get_description")]
    description: String,

    #[getter(rename = "get_category")]
    category: Category,

    #[mutable_ignore]
    #[getter(rename = "get_history")]
    history: VecWrapper<Contract>,

    #[mutable_ignore]
    #[getter(rename = "get_owner")]
    owner: Member,

    #[mutable_ignore]
    #[getter(rename = "get_day_of_creation")]
    day_of_creation: usize,

    #[getter(rename = "get_cost_per_day")]
    cost_per_day: f64,

    #[mutable_ignore]
    #[getter(rename = "get_is_available")]
    is_available: bool,

    #[eq]
    #[mutable_ignore]
    #[getter(rename = "get_uuid")]
    uuid: Uuid,
}

impl Default for Item {
    fn default() -> Self {
        Self {
            name: Default::default(),
            description: Default::default(),
            category: Default::default(),
            history: Default::default(),
            owner: Default::default(),
            day_of_creation: Default::default(),
            cost_per_day: Default::default(),
            is_available: true,
            uuid: Default::default(),
        }
    }
}

impl Item {
    /// Creates a new item.
    pub fn new(
        name: String,
        description: String,
        category: Category,
        owner: Member,
        cost_per_day: f64,
        day_of_creation: usize,
    ) -> Item {
        Item {
            name,
            category,
            description,
            owner,
            cost_per_day,
            day_of_creation,
            uuid: Uuid::new(),
            history: VecWrapper::new(),
            is_available: true,
        }
    }

    /// Adds a contract to history.
    pub fn add_contract(&mut self, contract: Contract) -> SysResult<()> {
        match self.get_contract_in_period(contract.get_start_date(), contract.get_end_date()) {
            Some(_) => Err(SysError::AlreadyExists),
            None => {
                self.history.push(contract);
                Ok(())
            }
        }
    }

    /// Gets the active contract. Returns Some(contract) if contract exists else None
    pub fn get_active_contract(&self, now: usize) -> Option<Contract> {
        for contract in self.history.iter() {
            if &now > contract.get_start_date() && &now < contract.get_end_date() {
                return Some(contract.clone());
            }
        }
        None
    }

    pub fn get_contract_in_period(&self, start_date: &usize, end_date: &usize) -> Option<Contract> {
        for contract in self.history.iter() {
            if start_date > contract.get_start_date() && start_date < contract.get_end_date()
                || end_date > contract.get_start_date() && end_date < contract.get_end_date()
            {
                return Some(contract.clone());
            }
        }
        None
    }

    fn has_contract_on_date(&self, date: &usize) -> bool {
        for contract in self.history.iter() {
            if date >= contract.get_start_date() && date < contract.get_end_date() {
                return true;
            }
        }
        false
    }

    pub fn get_history_map(&self) -> HashMap<&str, Vec<Contract>> {
        let mut past: Vec<Contract> = Vec::new();
        let mut active: Vec<Contract> = Vec::new();
        let mut future: Vec<Contract> = Vec::new();

        for contract in self.history.iter() {
            match contract.get_status() {
                Status::Active => active.push(contract.clone()),
                Status::Finished | Status::Canceled => past.push(contract.clone()),
                Status::Future => future.push(contract.clone()),
                Status::Other => {}
            }
        }

        HashMap::from([("past", past), ("future", future), ("active", active)])
    }

    pub fn get_availability(&self, now: usize) -> Vec<(String, bool)> {
        let mut out = Vec::new();
        for i in now..(now + 30) {
            out.push((i.to_string(), self.has_contract_on_date(&i)));
        }
        out
    }
}
