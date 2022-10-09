use super::system::SysError;
use super::{item::Item, member::Member, FromMap};
use crate::models::uuid::Uuid;
use derive_getters::{Dissolve, Getters};
use shared::{Builder, CData, CFromMap, CFromStr, CPartialEq, CToMap, CToStr, Model};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Default)]
pub enum Status {
    Active,
    Finished,
    Canceled,
    #[default]
    Future,
    Other,
}

impl FromStr for Status {
    type Err = SysError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Active" => Ok(Status::Active),
            "Finished" => Ok(Status::Finished),
            "Canceled" => Ok(Status::Canceled),
            "Future" => Ok(Status::Future),
            _ => Ok(Status::Other),
        }
    }
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Active => f.write_str("Active"),
            Status::Finished => f.write_str("Finished"),
            Status::Canceled => f.write_str("Canceled"),
            Status::Future => f.write_str("Future"),
            Status::Other => f.write_str("Other"),
        }
    }
}

/// Contract.
#[derive(
    Debug,
    Clone,
    Default,
    Getters,
    Dissolve,
    Builder,
    CFromStr,
    CFromMap,
    CToStr,
    CToMap,
    CData,
    CPartialEq,
    Model,
)]
#[dissolve(rename = "unpack")]
pub struct Contract {
    #[getter(rename = "get_owner")]
    #[mutable_ignore]
    owner: Member,

    #[getter(rename = "get_lendee")]
    #[mutable_ignore]
    lendee: Member,

    #[getter(rename = "get_start_date")]
    #[mutable_ignore]
    start_date: usize,

    #[getter(rename = "get_end_date")]
    #[mutable_ignore]
    end_date: usize,

    #[eq]
    #[getter(rename = "get_uuid")]
    #[mutable_ignore]
    uuid: Uuid,

    #[getter(rename = "get_item")]
    #[mutable_ignore]
    item: Item,

    #[getter(rename = "get_contract_len")]
    contract_len: usize,

    #[getter(rename = "get_credits")]
    #[mutable_ignore]
    credits: f64,

    #[getter(rename = "get_status")]
    #[mutable_ignore]
    status: Status,
}

impl Contract {
    /// Creates a new Contract.
    pub fn new(lendee: Member, start_date: usize, item: Item, contract_len: usize) -> Self {
        Self {
            owner: item.get_owner().clone(),
            uuid: Uuid::new(),
            credits: item.get_cost_per_day() * contract_len as f64,
            end_date: start_date + contract_len as usize,
            status: Status::Future,
            start_date,
            lendee,
            item,
            contract_len,
        }
    }

    pub fn set_status(&mut self, day: usize) {
        self.status = match day {
            date if date <= self.end_date && date >= self.start_date => Status::Active,
            date if date < self.end_date && date <= self.start_date => Status::Future,
            _ => Status::Finished,
        };
    }

    pub fn get_days_left(&self, now: usize) -> Option<usize> {
        if now < self.start_date || now > self.end_date {
            return None;
        }
        Some(&self.end_date - now)
    }

    /// ! IMPORTANT
    ///
    /// ! Before calling this method to build a new contract instance, the item has to
    /// have already been set.
    pub fn from_date(&mut self, date: usize, days: usize) -> Self {
        self.contract_len = days;
        self.end_date = self.start_date + days;
        self.credits = self.item.get_cost_per_day() * days as f64;
        self.set_status(date);
        self.clone()
    }
}
