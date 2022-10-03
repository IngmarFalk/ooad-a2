use super::{item::Item, member::Member, FromMap};
use crate::models::cdate::CDate;
use crate::models::uuid::Uuid;
use derive_getters::{Dissolve, Getters};
use shared::{Builder, CData, CFromMap, CFromStr, CPartialEq, CToMap, CToStr, Model};
use std::collections::HashMap;
use std::str::FromStr;

pub trait ContractValidation {
    fn validate_credits() -> bool;
    fn validate_availability() -> bool;
}

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
    owner: Member,
    #[getter(rename = "get_lendee")]
    lendee: Member,
    #[getter(rename = "get_start_day")]
    start_day: CDate,
    #[getter(rename = "get_end_day")]
    end_day: CDate,
    #[eq]
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
        start_day: CDate,
        end_day: CDate,
        item: Item,
        contract_len: u32,
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
        }
    }
}
