use shared::controller;

use crate::{
    models::{domain::item::Item, system::LendingSystem},
    types::{Model, View},
    views::item_view::{ItemMenuOption, ItemView},
};

use super::app::App;

#[controller(ItemView)]
pub struct ItemController<M, V>
where
    M: Model + LendingSystem,
    V: View + ItemView,
{
    model: M,
    view: V,
}

impl<M, V> ItemController<M, V>
where
    M: Model + LendingSystem,
    V: View + ItemView,
{
    fn display_item_info(&self) {
        let items = self.model.get_items();
        let item: Option<&Item> = self.view.select_item(items);
        match item {
            Some(i) => self.view.display_item_info(i),
            None => todo!(),
        }
    }

    fn edit_item(&mut self) {
        todo!()
    }

    fn create_item(&mut self) {
        todo!()
    }
}

impl<M, V> App for ItemController<M, V>
where
    M: Model + LendingSystem,
    V: View + ItemView,
{
    fn run(&mut self) {
        let choice = self.view.item_menu();
        match choice {
            ItemMenuOption::DisplayItemInfo => self.display_item_info(),
            ItemMenuOption::EditItemInfo => self.edit_item(),
            ItemMenuOption::CreateItem => self.create_item(),
            ItemMenuOption::Other => self.run(),
            ItemMenuOption::Quit => std::process::exit(0),
        }
    }
}
