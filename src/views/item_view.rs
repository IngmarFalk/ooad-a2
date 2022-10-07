use super::console::{Console, Ui};
use super::Options;
use crate::models::domain::item::Category;
use crate::models::domain::{item::Item, Data, FromMap};
use prettytable::Table;
use shared::{COptions, View};
use std::collections::HashMap;
use std::str::FromStr;

/// An enum that contains all possible valid choices for the
/// item menu.
#[derive(Debug, COptions)]
pub enum ItemMenuOption {
    /// Displays a single item.
    DisplayItemInfo,
    /// Items a specific item and returns a new instance.
    EditItemInfo,
    /// Creates a new item.
    CreateItem,
    /// Delets a specific item.
    DeleteItem,
    /// Go back to the previous page.
    Back,
    /// Quits the entire application.
    Quit,
    /// Any other state chosen by the user.
    #[other]
    Other,
}

/// Defines all required methods for a concrete implementation of the item view.
pub trait ItemView {
    ///Displays all options for the item menu.
    fn item_menu(&self) -> ItemMenuOption;
    /// Displaying information of a specific item.
    fn display_item_info(&self, item: &Item);
    /// Editing a specific item.
    fn edit_item_info(&self, item: &Item) -> Item;
    /// Getting information for a new item.
    fn get_item_info(&self) -> Item;
    /// Selecting an item from a list of possible options.
    fn select_item<'a>(&'a self, items: Vec<&'a Item>) -> Option<&Item>;
    /// Displays a message to the user and waits for him to respond.
    fn wait(&self, display: &str);
}

/// A concrete implementation of the item view.
#[derive(View)]
pub struct CliItemView {
    console: Console,
}

impl ItemView for CliItemView {
    fn item_menu(&self) -> ItemMenuOption {
        self.console.title();
        let choice: ItemMenuOption = self.console.show_menu(ItemMenuOption::options());
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
        let data = new_item_info
            .into_iter()
            .collect::<HashMap<String, String>>();

        item.copy_with(data)
    }

    fn get_item_info(&self) -> Item {
        let data = self
            .console
            .get_consecutive_str_input(Item::head_allowed_mutable());

        let buf = String::new();
        let name = match data.get("name") {
            Some(val) => val,
            None => &buf,
        };
        let description = match data.get("description") {
            Some(val) => val,
            None => &buf,
        };
        let category = Category::from_str(match data.get("category") {
            Some(val) => val,
            None => &buf,
        })
        .unwrap();
        let cost_per_day = match data.get("cost_per_day") {
            Some(val) => val,
            None => &buf,
        };

        let cost_per_day = match cost_per_day.parse::<f64>() {
            Ok(val) => val,
            Err(_) => {
                self.wait("Invalid input: Cost per day has to be of type int/float.");
                self.parse_float(|| self.console.get_str_input("cost_per_day: "))
            }
        };
        Item::default()
            .name(name.clone())
            .description(description.clone())
            .category(category)
            .cost_per_day(cost_per_day)
            .build()
    }

    fn select_item<'a>(&'a self, items: Vec<&'a Item>) -> Option<&Item> {
        self.console.select_model(items)
    }

    fn wait(&self, display: &str) {
        self.console.wait(display)
    }
}

impl CliItemView {
    fn parse_float<F>(&self, cpd: F) -> f64
    where
        F: Fn() -> String,
    {
        let s = cpd();
        match s.parse::<f64>() {
            Ok(val) => val,
            Err(_) => {
                let cpd_i = match s.parse::<i32>() {
                    Ok(val) => val,
                    Err(_) => -1,
                };
                match cpd_i < 0 {
                    true => {
                        self.wait("Invalid input: Cost per day has to be of type int/float.");
                        self.parse_float(cpd)
                    }
                    false => cpd_i as f64,
                }
            }
        }
    }
}
