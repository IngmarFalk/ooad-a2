use chrono::Local;

use super::category::Category;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Item {
    pub category: Category,
    pub name: String,
    pub description: String,
    day_of_creation: chrono::DateTime<Local>,
    cost_per_day: f64,
}

impl Item {
    pub fn new(
        category: Category,
        name: String,
        description: String,
        day_of_creation: chrono::DateTime<Local>,
        cost_per_day: f64,
    ) -> Item {
        Item {
            category,
            name,
            description,
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
