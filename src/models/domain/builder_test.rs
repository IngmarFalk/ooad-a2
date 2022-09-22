use crate::types::StringMap;
use shared::Builder;

use super::{item::Item, member::Member};

#[derive(Builder)]
pub struct BuilderTest {
    attr1: String,
    attr2: Item,
    attr3: Member,
    attr4: u32,
    attr5: String,
}

impl BuilderTest {
    pub fn new() -> BuilderTest {
        BuilderTest {
            attr1: String::from("Test"),
            attr2: Item::default(),
            attr3: Member::default(),
            attr4: 32,
            attr5: String::from("Builder"),
        }
    }
}

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
