use std::collections::HashMap;

use chrono::Local;
use derive_getters::{Dissolve, Getters};
use prettytable::{row, Row, Table};

use crate::{models::uuid::Uuid, types::StringMap};

use super::{item::Item, member::Member, Data, FromMap, ToMap};

pub trait ContractValidation {
    fn validate_credits() -> bool;
    fn validate_availability() -> bool;
}

#[derive(Debug, Clone, PartialEq, Default, Getters, Dissolve)]
#[dissolve(rename = "unpack")]
pub struct Contract {
    owner: Member,
    lendee: Member,
    start_day: chrono::DateTime<Local>,
    end_day: chrono::DateTime<Local>,
    uuid: Uuid,
    item: Item,
    contract_len: u32,
    credits: f64,
}

impl Contract {
    pub fn new(
        owner: Member,
        lendee: Member,
        start_day: chrono::DateTime<Local>,
        end_day: chrono::DateTime<Local>,
        item: Item,
        contract_len: u32,
        credits: f64,
    ) -> Self {
        Self {
            owner,
            lendee,
            uuid: Uuid::contract(),
            start_day,
            end_day,
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

impl ToMap for Contract {
    fn to_map(&self) -> StringMap {
        let attrs = vec![
            self.owner.to_string(),
            self.lendee.to_string(),
            self.uuid.to_string(),
            self.start_day.to_string(),
            self.end_day.to_string(),
            self.item.to_string(),
            self.contract_len.to_string(),
            self.credits.to_string(),
        ];
        self.head()
            .into_iter()
            .zip(attrs.into_iter())
            .collect::<HashMap<String, String>>()
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

    fn head(&self) -> Vec<String> {
        vec![
            "owner".to_owned(),
            "lendee".to_owned(),
            "uuid".to_owned(),
            "start_day".to_owned(),
            "end_day".to_owned(),
            "item".to_owned(),
            "contract_len".to_owned(),
            "credits".to_owned(),
        ]
    }

    fn head_allowed_mutable(&self) -> Vec<String> {
        vec![
            "end_day".to_owned(),
            "length".to_owned(),
            "credits".to_owned(),
        ]
    }

    fn to_table(&self) -> prettytable::Table {
        let mut table = Table::new();
        table.add_row(Row::from(self.head()));
        table.add_row(self.to_row());
        table
    }
}
