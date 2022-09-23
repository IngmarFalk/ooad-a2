use super::item::Item;
use super::member::Member;
use super::FromMap;
use crate::types::StringMap;
use derive_getters::Getters;
use shared::{Builder, CFromMap, CFromStr, CToMap, CToStr};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone, Builder, CFromStr, CToStr, CToMap, Default, Getters)]
pub struct BuilderTest {
    #[getter(rename = "get_attr1")]
    attr1: String,
    #[getter(rename = "get_attr2")]
    attr2: Item,
    #[getter(rename = "get_attr3")]
    attr3: Member,
    #[getter(rename = "get_attr4")]
    attr4: u32,
    #[getter(rename = "get_attr5")]
    attr5: String,
}

impl BuilderTest {
    pub fn new() -> Self {
        Self {
            attr1: String::from("Test"),
            attr2: Item::default(),
            attr3: Member::default(),
            attr4: 32,
            attr5: String::from("Builder"),
        }
    }
}

impl FromMap for BuilderTest {
    fn from_partial_map(data: StringMap) -> Self {
        todo!()
    }

    fn from_complete_map(data: StringMap) -> Self {
        todo!()
    }

    fn copy_with(&mut self, data: StringMap) -> Self {
        match data.get("attr1") {
            Some(val) => self.attr1 = val.parse::<String>().ok().expect("Unable to parse to type"),
            None => {}
        }
        match data.get("attr2") {
            Some(val) => self.attr2 = val.parse::<Item>().ok().expect("Unable to parse to type"),
            None => {}
        }
        match data.get("attr3") {
            Some(val) => self.attr3 = val.parse::<Member>().ok().expect("Unable to parse to type"),
            None => {}
        }
        match data.get("attr4") {
            Some(val) => self.attr4 = val.parse::<u32>().ok().expect("Unable to parse to type"),
            None => {}
        }
        match data.get("attr5") {
            Some(val) => self.attr5 = val.parse::<String>().ok().expect("Unable to parse to type"),
            None => {}
        }
        self.build()
    }
}
