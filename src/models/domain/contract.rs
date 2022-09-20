use chrono::Local;
use prettytable::{row, Row, Table};

use crate::models::uuid::Uuid;

use super::Data;

pub trait ContractValidation {
    fn validate_credits() -> bool;
    fn validate_availability() -> bool;
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Contract {
    pub uuid: Uuid,
    s_day: chrono::DateTime<Local>,
    e_day: chrono::DateTime<Local>,
    contract_len: u32,
    // item: Item,
    credits: f64,
}

impl Contract {
    pub fn new(
        s_day: chrono::DateTime<Local>,
        e_day: chrono::DateTime<Local>,
        contract_len: u32,
        // item: Item,
        credits: f64,
    ) -> Self {
        Self {
            uuid: Uuid::new(),
            s_day,
            e_day,
            contract_len,
            // item,
            credits,
        }
    }
}

impl std::fmt::Display for Contract {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Start Day:\t{}\nEnd Day:\t{}\nLength:\t{}\nCredits:\t{}",
            self.s_day, self.e_day, self.contract_len, self.credits
        ))
    }
}

impl Data for Contract {
    fn to_row(&self) -> Row {
        row![
            self.s_day.date().naive_local().to_string(),
            self.e_day.date().naive_local().to_string(),
            self.contract_len.to_string(),
            self.credits.to_string(),
        ]
    }

    fn head(&self) -> Row {
        row!["start_day", "end_day", "length", "credits"]
    }

    fn head_allowed_mutable(&self) -> Row {
        row!["end_day", "length", "credits"]
    }

    fn to_table(&self) -> prettytable::Table {
        let mut table = Table::new();
        table.add_row(self.head());
        table.add_row(self.to_row());
        table
    }
}
