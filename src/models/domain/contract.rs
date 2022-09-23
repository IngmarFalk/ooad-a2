use chrono::Local;
// use chrono::Local;
use derive_getters::{Dissolve, Getters};
use prettytable::{row, Row, Table};
use shared::{CFromMap, CFromStr, CToMap, CToStr};
use std::collections::HashMap;
use std::str::FromStr;

use crate::models::uuid::Uuid;

use super::{item::Item, member::Member, Data, FromMap};

pub trait ContractValidation {
    fn validate_credits() -> bool;
    fn validate_availability() -> bool;
}

#[derive(Debug, Clone, Default, Getters, Dissolve, CFromStr, CFromMap, CToStr, CToMap)]
#[dissolve(rename = "unpack")]
pub struct Contract {
    #[getter(rename = "get_owner")]
    owner: Member,
    #[getter(rename = "get_lendee")]
    lendee: Member,
    #[getter(rename = "get_start_day")]
    start_day: chrono::DateTime<Local>,
    #[getter(rename = "get_end_day")]
    end_day: chrono::DateTime<Local>,
    #[getter(rename = "get_uuid")]
    uuid: Uuid,
    #[getter(rename = "get_item")]
    item: Item,
    #[getter(rename = "get_contract_len")]
    contract_len: u32,
    #[getter(rename = "get_credits")]
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

impl PartialEq for Contract {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}

impl Into<String> for Contract {
    fn into(self) -> String {
        self.to_string()
    }
}

// impl crate::models::domain::FromMap for Contract {
//     fn from_partial_map(
//         data: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
//     ) -> Self {
//         todo!()
//     }
//     fn from_complete_map(
//         data: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
//     ) -> Self {
//         Self {
//             uuid: data.get("uuid").unwrap().parse::<Uuid>().unwrap(),
//             owner: data.get("owner").unwrap().parse::<Member>().unwrap(),
//             lendee: data.get("lendee").unwrap().parse::<Member>().unwrap(),
//             item: data.get("item").unwrap().parse::<Item>().unwrap(),
//             contract_len: data.get("contract_len").unwrap().parse::<u32>().unwrap(),
//             credits: data.get("credits").unwrap().parse::<f64>().unwrap(),
//         }
//     }
//     fn copy_with(
//         &mut self,
//         data: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
//     ) -> Self {
//         todo!()
//     }
// }

impl Data for Contract {
    fn to_row(&self) -> Row {
        row![
            // self.start_day.date().naive_local().to_string(),
            // self.end_day.date().naive_local().to_string(),
            self.contract_len.to_string(),
            self.credits.to_string(),
        ]
    }

    fn head(&self) -> Vec<String> {
        vec![
            "owner".to_owned(),
            "lendee".to_owned(),
            "uuid".to_owned(),
            // "start_day".to_owned(),
            // "end_day".to_owned(),
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
