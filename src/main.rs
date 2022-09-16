pub mod models;

use std::iter::FromIterator;

use rand::{distributions::Alphanumeric, thread_rng, Rng};

use crate::models::{member::Member, uuid::Uuid};

fn main() {
    println!("Hello, world!");
    let uuid: Uuid = Uuid::new();
    let member: Member = Member::default();

    println!("{}", uuid);
}
