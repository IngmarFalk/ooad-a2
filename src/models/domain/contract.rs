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

    #[getter(rename = "get_start_day")]
    #[mutable_ignore]
    start_day: CDate,

    #[getter(rename = "get_end_day")]
    end_day: CDate,

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

    #[getter(rename = "get_cost_per_day")]
    #[mutable_ignore]
    cost_per_day: f64,
}

impl Contract {
    /// Creates a new Contract.
    pub fn new(
        owner: Member,
        lendee: Member,
        start_day: CDate,
        end_day: CDate,
        item: Item,
        contract_len: i64,
        credits: f64,
    ) -> Self {
        Self {
            owner,
            lendee,
            uuid: Uuid::new(),
            start_day,
            end_day,
            item,
            contract_len,
            credits,
            cost_per_day: credits / contract_len as f64,
        }
    }

    pub fn get_days_left(&self) -> Option<u32> {
        let now = CDate::now();
        if now < self.start_day || now > self.end_day {
            return None;
        }
        Some(now.days_till(&self.end_day).unwrap() as u32)
    }

    ///! Required:
    ///
    ///! Before calling this method the `self.credits` have to already ben set.
    pub fn from_now_with_days(&mut self, days: i64) -> Self {
        self.contract_len = days;
        self.start_day = CDate::now();
        self.end_day = CDate::new(self.start_day.as_naive_date() + Duration::days(days));
        self.cost_per_day = self.credits / days as f64;
        self.clone()
    }
}
