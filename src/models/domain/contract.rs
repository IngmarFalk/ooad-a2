use super::{item::Item, member::Member, FromMap};
use crate::models::cdate::CDate;
use crate::models::uuid::Uuid;
use chrono::Duration;
use derive_getters::{Dissolve, Getters};
use shared::{Builder, CData, CFromMap, CFromStr, CPartialEq, CToMap, CToStr, Model};
use std::collections::HashMap;
use std::str::FromStr;

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
    start_date: CDate,

    #[getter(rename = "get_end_date")]
    end_date: CDate,

    #[eq]
    #[getter(rename = "get_uuid")]
    #[mutable_ignore]
    uuid: Uuid,

    #[getter(rename = "get_item")]
    #[mutable_ignore]
    item: Item,

    #[getter(rename = "get_contract_len")]
    #[mutable_ignore]
    contract_len: i64,

    #[getter(rename = "get_credits")]
    credits: f64,
}

impl Contract {
    /// Creates a new Contract.
    pub fn new(
        owner: Member,
        lendee: Member,
        start_date: CDate,
        end_date: CDate,
        item: Item,
        contract_len: i64,
        credits: f64,
    ) -> Self {
        Self {
            owner,
            lendee,
            uuid: Uuid::new(),
            start_date,
            end_date: end_date,
            item,
            contract_len,
            credits,
        }
    }

    pub fn get_days_left(&self) -> Option<u32> {
        let now = CDate::now();
        if now < self.start_date || now > self.end_date {
            return None;
        }
        Some(now.days_till(&self.end_date).unwrap() as u32)
    }

    /// ! IMPORTANT
    ///
    /// ! Before calling this method to build a new contract instance, the item has to
    /// have already been set.
    pub fn from_now_with_days(&mut self, days: i64) -> Self {
        self.contract_len = days;
        self.start_date = CDate::now();
        self.end_date = CDate::new(self.start_date.as_naive_date() + Duration::days(days));
        self.credits = self.item.get_cost_per_day() * days as f64;
        self.clone()
    }

    pub fn from_date_with_days(&mut self, date: CDate, days: i64) -> Self {
        self.contract_len = days;
        self.start_date = date;
        self.end_date = CDate::new(self.start_date.as_naive_date() + Duration::days(days));
        self.credits = self.item.get_cost_per_day() * days as f64;
        self.clone()
    }
}
