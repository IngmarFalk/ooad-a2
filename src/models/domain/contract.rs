use chrono::Local;
use derive_getters::Getters;
use prettytable::{row, Row, Table};

use crate::models::uuid::Uuid;

use super::{item::Item, member::Member, Data, FromMap, ToMap};

pub trait ContractValidation {
    fn validate_credits() -> bool;
    fn validate_availability() -> bool;
}

#[derive(Debug, Clone, PartialEq, Default, Getters)]
pub struct Contract {
    pub uuid: Uuid,
    start_day: chrono::DateTime<Local>,
    end_day: chrono::DateTime<Local>,
    lendee: Member,
    contract_len: u32,
    item: Item,
    credits: f64,
}

impl Contract {
    pub fn new(
        lendee: Member,
        start_day: chrono::DateTime<Local>,
        end_day: chrono::DateTime<Local>,
        item: Item,
        contract_len: u32,
        credits: f64,
    ) -> Self {
        Self {
            uuid: Uuid::contract(),
            start_day,
            end_day,
            lendee,
            item,
            contract_len,
            credits,
        }
    }
}

impl std::fmt::Display for Contract {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Start Day:\t{}\nEnd Day:\t{}\nLength:\t{}\nCredits:\t{}",
            self.start_day, self.end_day, self.contract_len, self.credits
        ))
    }
}

impl FromMap for Contract {
    fn from_partial_map(data: crate::types::StringMap) -> Self {
        todo!()
    }

    fn from_complete_map(data: crate::types::StringMap) -> Self {
        todo!()
    }

    fn copy_with(&self, data: crate::types::StringMap) -> Self {
        todo!()
    }
}

impl ToMap for Contract {
    fn to_map(&self) -> crate::types::StringMap {
        todo!()
    }

    fn to_allowed_mutable_map(&self) -> crate::types::StringMap {
        todo!()
    }

    fn to_buffers_map(&self) -> crate::types::StringMap {
        todo!()
    }
}

impl Data for Contract {
    fn to_row(&self) -> Row {
        row![
            self.start_day.date().naive_local().to_string(),
            self.end_day.date().naive_local().to_string(),
            self.contract_len.to_string(),
            self.credits.to_string(),
        ]
    }

    fn head(&self) -> Vec<&str> {
        vec![
            "owner",
            "lendee",
            "start_day",
            "end_day",
            "length",
            "credits",
        ]
    }

    fn head_allowed_mutable(&self) -> Vec<&str> {
        vec!["end_day", "length", "credits"]
    }

    fn to_table(&self) -> prettytable::Table {
        let mut table = Table::new();
        table.add_row(Row::from(self.head()));
        table.add_row(self.to_row());
        table
    }
}
