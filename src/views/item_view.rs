use std::collections::HashMap;

use prettytable::Table;

use crate::{
    models::domain::{item::Item, Data, FromMap},
    types::StringMap,
};

use super::console::{Console, Ui};

pub trait ItemView {
    fn display_item_info(&self, item: Item);
    fn edit_item_info(&self, item: Item) -> Item;
    fn get_item_info(&self) -> Item;
}

pub struct CliItemView {
    console: Console,
}

impl CliItemView {
    pub fn new() -> CliItemView {
        CliItemView {
            console: Console::new(),
        }
    }
}

impl ItemView for CliItemView {
    fn display_item_info(&self, item: Item) {
        let mut table = Table::new();
        table.add_row(item.to_row());
        self.console.display_table(table);
    }

    fn edit_item_info(&self, item: Item) -> Item {
        let new_item_info = self
            .console
            .get_consecutive_str_input(item.head_allowed_mutable());
        let data: StringMap = HashMap::from(
            new_item_info
                .into_iter()
                .collect::<HashMap<String, String>>(),
        );
        item.copy_with(data)
    }

    fn get_item_info(&self) -> Item {
        todo!()
    }
}
