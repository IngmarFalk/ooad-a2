use chrono::{DateTime, Local};

use super::item::Item;

pub trait ContractValidation {
    fn validate_credits() -> bool;
    fn validate_availability() -> bool;
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Contract {
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
            s_day,
            e_day,
            contract_len,
            // item,
            credits,
        }
    }
}
