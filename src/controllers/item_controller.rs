use shared::controller;

use crate::{
    models::system::LendingSystem,
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
    pub fn new(model: M, view: V) -> Self {
        Self { model, view }
    }
}

impl<M, V> App for ItemController<M, V>
where
    M: Model + LendingSystem,
    V: View + ItemView,
{
    fn run(&self) {
        let choice = self.view.item_menu();
        match choice {
            ItemMenuOption::DisplayItemInfo => todo!(),
            ItemMenuOption::EditItemInfo => todo!(),
            ItemMenuOption::GetItemInfo => todo!(),
            ItemMenuOption::Other => todo!(),
        }
    }
}
