use std::fmt::{write, Display};

use chrono::{Date, Local, Utc};

use super::{contract::Contract, uuid::Uuid};

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

#[derive(Debug, Clone, PartialEq)]
pub struct Item {
    pub uuid: Uuid,
    pub category: Category,
    pub name: String,
    pub description: String,
    pub contract: Option<Contract>,
    day_of_creation: chrono::DateTime<Local>,
    cost_per_day: f64,
}

impl Item {
    pub fn new(
        category: Category,
        name: String,
        description: String,
        contract: Option<Contract>,
        day_of_creation: chrono::DateTime<Local>,
        cost_per_day: f64,
    ) -> Item {
        Item {
            uuid: Uuid::new(),
            category,
            name,
            description,
            contract,
            day_of_creation,
            cost_per_day,
        }
    }

    pub fn name(mut self, name: String) -> Item {
        self.name = name;
        self
    }

    pub fn category(mut self, category: Category) -> Item {
        self.category = category;
        self
    }
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // f.write_fmt(format_args!("Name:\t{}\nCategory:\t{}\nDescription:\t{}Contract:\t{:?}\nDay of creation:\t{}\nCost per Day:\t{}", self.name, self.category, self.description, self.contract, self.day_of_creation, self.cost_per_day))
        f.write_fmt(format_args!(
            "Item [\n\t  Name:\t\t{}\n\t  Description:\t{}\n\t  Category:\t{},\n\t  Contract:\t{:?}\n\t  Date:\t\t{}\n\t  Cost per Day:\t${}\n\t]",
            self.name,
            self.description,
            self.category,
            self.contract,
            self.day_of_creation.date().naive_local(),
            self.cost_per_day,
        ))
    }
}
