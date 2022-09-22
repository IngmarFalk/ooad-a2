use super::item::Item;
use super::member::Member;
use super::FromMap;
use crate::types::StringMap;
use derive_getters::Getters;
use shared::{Builder, CFromStr, CToStr};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone, Builder, CFromStr, CToStr, Default, Getters)]
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

// impl ::std::fmt::Display for BuilderTest {
//     fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
//         f.write_fmt(format_args!(
//             "{};{};{};{};{}",
//             self.attr1, self.attr2, self.attr3, self.attr4, self.attr5,
//         ))
//     }
// }
// impl ToString for BuilderTest {
//     fn to_string(&self) -> String {
//         let attrs = vec![
//             self.attr1.to_string(),
//             self.attr1.to_string(),
//             self.attr1.to_string(),
//             self.attr1.to_string(),
//             self.attr1.to_string(),
//         ];
//         attrs.join(",")
//     }
// }
