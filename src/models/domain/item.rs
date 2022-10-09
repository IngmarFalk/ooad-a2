use super::system::{SysError, SysResult};
use super::{contract::Contract, member::Member, FromMap};
use crate::models::cdate::CDate;
use crate::models::cvec::CVec;
use crate::models::uuid::Uuid;
use derive_getters::{Dissolve, Getters};
use shared::{Builder, CData, CFromMap, CFromStr, CPartialEq, CToMap, CToStr, Model};
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
    CFromStr,
    CToStr,
    CFromMap,
    CToMap,
    CData,
    CPartialEq,
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
    history: CVec<Contract>,

    #[mutable_ignore]
    #[getter(rename = "get_owner")]
    owner: Member,

    #[mutable_ignore]
    #[getter(rename = "get_day_of_creation")]
    day_of_creation: CDate,

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
    ) -> Item {
        Item {
            name,
            category,
            description,
            owner,
            cost_per_day,
            uuid: Uuid::new(),
            history: CVec::new(),
            day_of_creation: CDate::now(),
            is_available: true,
        }
    }

    /// Adds a contract to history.
    pub fn add_contract(&mut self, contract: Contract) -> SysResult<()> {
        match self.get_contract_in_period(contract.get_start_day(), contract.get_end_day()) {
            Some(_) => Err(SysError::AlreadyExists),
            None => {
                self.history.push(contract);
                if let Some(_) = self.get_active_contract() {
                    self.set_unavailable();
                }
                Ok(())
            }
        }
    }

    /// Gets the active contract. Returns Some(contract) if contract exists else None
    fn get_active_contract(&self) -> Option<Contract> {
        let current_date = CDate::now();
        for contract in self.history.iter() {
            if &current_date > contract.get_start_day() && &current_date < contract.get_end_day() {
                return Some(contract.clone());
            }
        }
        None
    }

    fn get_contract_in_period(&self, start_day: &CDate, end_day: &CDate) -> Option<Contract> {
        for contract in self.history.iter() {
            if &start_day > &contract.get_start_day() && &end_day < &contract.get_end_day() {
                return Some(contract.clone());
            }
        }
        None
    }

    fn set_unavailable(&mut self) {
        self.is_available = false;
    }

    fn set_available(&mut self) {
        self.is_available = true;
    }
}
