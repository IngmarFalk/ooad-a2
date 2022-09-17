use chrono::Local;

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

impl Default for Category {
    fn default() -> Self {
        Category::Other
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Item {
    pub uuid: Uuid,
    pub category: Category,
    pub name: String,
    pub description: String,
    pub contract: Contract,
    day_of_creation: chrono::DateTime<Local>,
    cost_per_day: f64,
}

impl Item {
    pub fn new(
        category: Category,
        name: String,
        description: String,
        contract: Contract,
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
