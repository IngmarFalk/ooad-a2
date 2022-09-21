use std::{fmt::Display, str::FromStr};

use chrono::Local;
use prettytable::{row, Row, Table};

use crate::{
    models::uuid::Uuid,
    types::{ContractsList, StringMap},
};

use super::{contract::Contract, member::Member, Data, FromMap, ToMap};

#[derive(Debug, Clone, PartialEq)]
pub enum Category {
    Tool,
    Vehicle,
    Game,
    Toy,
    Sport,
    Other,
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

#[derive(Debug, Clone, Default)]
pub struct Item {
    pub uuid: Uuid,
    pub owner: Member,
    pub category: Category,
    pub name: String,
    pub description: String,
    pub history: ContractsList,
    day_of_creation: chrono::DateTime<Local>,
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
            history: vec![],
            day_of_creation: chrono::offset::Local::now(),
        }
    }

    pub fn name(mut self, name: String) -> Item {
        self.name = name;
        self
    }

    pub fn description(mut self, description: String) -> Item {
        self.description = description;
        self
    }

    pub fn cost_per_day(mut self, cost_per_day: f64) -> Item {
        self.cost_per_day = cost_per_day;
        self
    }

    pub fn category(mut self, category: Category) -> Item {
        self.category = category;
        self
    }

    pub fn add_contract(&mut self, contract: Contract) {
        self.history.push(contract);
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

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // f.write_fmt(format_args!("Name:\t{}\nCategory:\t{}\nDescription:\t{}Contract:\t{:?}\nDay of creation:\t{}\nCost per Day:\t{}", self.name, self.category, self.description, self.contract, self.day_of_creation, self.cost_per_day))
        f.write_fmt(format_args!(
            "Item [\n\t  Name:\t\t{}\n\t  Description:\t{}\n\t  Category:\t{},\n\t Date:\t\t{}\n\t  Cost per Day:\t${}\n\t]",
            self.name,
            self.description,
            self.category,
            // self.active_contract,
            self.day_of_creation.date().naive_local(),
            self.cost_per_day,
        ))
    }
}

impl FromMap for Item {
    fn from_partial_map(data: StringMap) -> Self {
        todo!()
    }

    fn from_complete_map(data: StringMap) -> Self {
        todo!()
    }

    fn copy_with(&self, data: StringMap) -> Self {
        todo!()
    }
}

impl ToMap for Item {
    fn to_map(&self) -> StringMap {
        todo!()
    }

    fn to_allowed_mutable_map(&self) -> StringMap {
        todo!()
    }

    fn to_buffers_map(&self) -> StringMap {
        todo!()
    }
}

impl Data for Item {
    fn to_row(&self) -> Row {
        let contract_str = match &self.get_active_contract() {
            Some(c) => c.uuid.to_string(),
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

    fn head(&self) -> Vec<&str> {
        vec![
            "Name",
            "Description",
            "Category",
            "Contract",
            "Cost Per Day",
        ]
    }

    fn head_allowed_mutable(&self) -> Vec<&str> {
        vec!["Name", "Description", "Category", "Cost Per Day"]
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
