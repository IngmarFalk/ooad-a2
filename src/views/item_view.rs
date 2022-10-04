use super::console::{Console, Ui};
use super::Options;
use crate::models::domain::item::Category;
use crate::{
    models::domain::{item::Item, Data, FromMap},
    types::StringMap,
};
use prettytable::Table;
use shared::{COptions, View};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, COptions)]
pub enum ItemMenuOption {
    DisplayItemInfo,
    EditItemInfo,
    CreateItem,
    DeleteItem,
    Quit,
    Back,
    #[other]
    Other,
}

pub trait ItemView {
    fn item_menu(&self) -> ItemMenuOption;
    fn display_item_info(&self, item: &Item);
    fn edit_item_info(&self, item: &Item) -> Item;
    fn get_item_info(&self) -> Item;
    fn select_item<'a>(&'a self, items: Vec<&'a Item>) -> Option<&Item>;
    fn wait(&self, display: &str);
}

#[derive(View)]
pub struct CliItemView {
    console: Console,
}

impl ItemView for CliItemView {
    fn item_menu(&self) -> ItemMenuOption {
        self.console.title();
        let choice: ItemMenuOption = self.console.show_menu(ItemMenuOption::options());
        // let out = format!("{}", choice);
        // self.wait(out.as_str());
        match choice {
            ItemMenuOption::Other => self.item_menu(),
            _ => choice,
        }
    }

    fn display_item_info(&self, item: &Item) {
        let mut table = Table::new();
        table.add_row(item.to_row());
        self.console.display_table(table);
    }

    fn edit_item_info(&self, item: &Item) -> Item {
        let new_item_info = self
            .console
            .get_consecutive_str_input(Item::head_allowed_mutable());
        let data: StringMap = new_item_info
            .into_iter()
            .collect::<HashMap<String, String>>();

        item.copy_with(data)
    }

    fn get_item_info(&self) -> Item {
        let [name, description, category, cost_per_day] = <[String; 4]>::try_from(
            self.console
                .get_consecutive_str_input(Item::head_allowed_mutable())
                .iter()
                .map(|entry| -> String { entry.1.clone() })
                .collect::<Vec<String>>(),
        )
        .ok()
        .unwrap();
        Item::default()
            .name(name)
            .description(description)
            .category(Category::from_str(category.as_str()).unwrap())
            .cost_per_day(cost_per_day.parse::<f64>().unwrap())
            .build()
    }

    fn select_item<'a>(&'a self, items: Vec<&'a Item>) -> Option<&Item> {
        self.console.select_model(items)
    }

    fn wait(&self, display: &str) {
        self.console.wait(display)
    }
}
