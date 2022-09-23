use crate::models::cvec::CVec;
use crate::models::system::MError;
use chrono::Local;
use derive_getters::{Dissolve, Getters};
use prettytable::{row, Row, Table};
use shared::{Builder, CFromMap, CFromStr, CToMap, CToStr};
use std::str::FromStr;
use std::{collections::HashMap, fmt::Display};

use crate::models::uuid::Uuid;

use super::{contract::Contract, member::Member, Data, FromMap};

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
    type Err = MError;

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

#[derive(Debug, Clone, Default, Getters, Dissolve, Builder, CFromStr, CToStr, CFromMap, CToMap)]
#[dissolve(rename = "unpack")]
pub struct Item {
    #[getter(rename = "get_uuid")]
    uuid: Uuid,
    #[getter(rename = "get_category")]
    category: Category,
    #[getter(rename = "get_name")]
    name: String,
    #[getter(rename = "get_description")]
    description: String,
    #[getter(rename = "get_history")]
    history: CVec<Contract>,
    #[getter(rename = "get_owner")]
    owner: Member,
    #[getter(rename = "get_day_of_creation")]
    day_of_creation: chrono::DateTime<Local>,
    #[getter(rename = "get_cost_per_day")]
    cost_per_day: f64,
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
            uuid: Uuid::item(),
            history: CVec::new(),
            day_of_creation: chrono::offset::Local::now(),
        }
    }

    fn get_active_contract(&self) -> Option<Contract> {
        for contract in self.history.iter() {
            /// TODO : If current date lies within contract, return contract.
            continue;
        }
        todo!()
    }

    fn has_singular_active_contract(&self) -> bool {
        for contract in self.history.iter() {
            /// TODO : Check if there is only a single or no active contract.
            continue;
        }
        todo!()
    }
}

impl Data for Item {
    fn to_row(&self) -> Row {
        let contract_str = match &self.get_active_contract() {
            Some(c) => c.get_uuid().to_string(),
            None => "No Contract".to_owned(),
        };
        row![
            self.name.clone(),
            self.description.clone(),
            self.category.clone().to_string(),
            contract_str,
            self.cost_per_day.to_string(),
        ]
    }

    fn head(&self) -> Vec<String> {
        vec![
            "Name".to_owned(),
            "Description".to_owned(),
            "Category".to_owned(),
            "Contract".to_owned(),
            "Cost Per Day".to_owned(),
        ]
    }

    fn head_allowed_mutable(&self) -> Vec<String> {
        vec![
            "Name".to_owned(),
            "Description".to_owned(),
            "Category".to_owned(),
            "Cost Per Day".to_owned(),
        ]
    }

    fn to_table(&self) -> prettytable::Table {
        let mut table = Table::new();
        table.add_row(Row::from(self.head()));
        table.add_row(self.to_row());
        table
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }

    fn ne(&self, other: &Self) -> bool {
        self.uuid != other.uuid
    }
}

impl Eq for Item {}
