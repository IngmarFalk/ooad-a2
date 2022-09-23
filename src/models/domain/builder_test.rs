use super::item::Item;
use super::member::Member;
use super::FromMap;
use crate::models::uuid::Uuid;
use derive_getters::Getters;
use shared::{Builder, CFromMap, CFromStr, CToMap, CToStr};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone, Builder, CFromStr, CToStr, CToMap, CFromMap, Default, Getters)]
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
    attr5: Uuid,
}

impl BuilderTest {
    pub fn new() -> Self {
        Self {
            attr1: String::from("Test"),
            attr2: Item::default(),
            attr3: Member::default(),
            attr4: 32,
            attr5: Uuid::default(),
        }
    }

    // fn copy_with2(&self, data: StringMap) -> BuilderTest {
    //     let Self {
    //         attr1,
    //         attr2,
    //         attr3,
    //         attr4,
    //         attr5,
    //     } = self;
    //     Self {
    //         attr1: match data.get("attr1") {
    //             Some(val) => val.parse::<String>().unwrap(),
    //             None => attr1.to_owned(),
    //         },
    //         attr2: match data.get("attr2") {
    //             Some(val) => val.parse::<Item>().unwrap(),
    //             None => attr2.to_owned(),
    //         },
    //         attr3: match data.get("attr3") {
    //             Some(val) => val.parse::<Member>().unwrap(),
    //             None => attr3.to_owned(),
    //         },
    //         attr4: match data.get("attr4") {
    //             Some(val) => val.parse::<u32>().unwrap(),
    //             None => attr4.to_owned(),
    //         },
    //         attr5: match data.get("attr5") {
    //             Some(val) => val.parse::<Uuid>().unwrap(),
    //             None => attr5.to_owned(),
    //         },
    //     }
    // }
}
