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
