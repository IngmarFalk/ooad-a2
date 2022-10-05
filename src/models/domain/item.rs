use super::system::{MResult, SysError};
use super::{contract::Contract, member::Member, FromMap};
use crate::models::cdate::CDate;
use crate::models::cvec::CVec;
use crate::models::uuid::Uuid;
use derive_getters::{Dissolve, Getters};
use shared::{Builder, CData, CFromMap, CFromStr, CPartialEq, CToMap, CToStr, Model};
use std::str::FromStr;
use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Clone, PartialEq)]
pub enum Category {
    Tool,
    Vehicle,
    Game,
    Toy,
    Sport,
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
        // f.write_fmt(format_args!("{}", self.to_string().as_str()))
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

impl Default for Category {
    fn default() -> Self {
        Category::Other
    }
}

impl From<&str> for Category {
    fn from(inp: &str) -> Self {
        match inp.to_lowercase().as_str() {
            "tool" => Category::Tool,
            "vehicle" => Category::Vehicle,
            "game" => Category::Game,
            "toy" => Category::Toy,
            "sport" => Category::Sport,
            _ => Category::Other,
        }
    }
}

#[derive(
    Debug,
    Clone,
    Default,
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
    #[eq]
    #[mutable_ignore]
    #[getter(rename = "get_uuid")]
    uuid: Uuid,

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
}

impl Item {
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
            day_of_creation: CDate::new(),
            is_available: false,
        }
    }

    pub fn add_contract(&mut self, contract: Contract) -> MResult<()> {
        match self.get_active_contract() {
            Some(_) => Err(SysError::AlreadyExists),
            None => {
                self.history.push(contract);
                Ok(())
            }
        }
    }

    fn get_active_contract(&self) -> Option<Contract> {
        let current_date = CDate::new();
        for contract in self.history.iter() {
            if &current_date > contract.get_start_day() && &current_date < contract.get_end_day() {
                return Some(contract.clone());
            }
        }
        None
    }
}
