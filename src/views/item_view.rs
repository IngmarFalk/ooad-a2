use prettytable::Table;

use crate::models::domain::{
    item::{Category, Item},
    ToRow,
};

use super::console::Console;

pub trait ItemView {
    fn display_item_info(&self, item: Item);
    fn edit_item_info(&self, item: Item);
    fn get_item_info(&self) -> Item;
}

pub struct CliItemView {
    console: Console,
}

impl ItemView for CliItemView {
    fn display_item_info(&self, item: Item) {
        let mut table = Table::new();
        table.add_row(item.to_row());
        self.console.table(table);
    }

    fn edit_item_info(&self, item: Item) {
        let name: String = self.console.get_str_input("Name: ");
        let category = Category::from(self.console.get_str_input("Category: "));
        todo!()
    }

    fn get_item_info(&self) -> Item {
        todo!()
    }
}
