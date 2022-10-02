use super::{app::App, member_controller::MemberController};
use crate::{
    models::{
        domain::item::Item,
        system::{LendingSystem, System},
    },
    types::{Model, View},
    views::{
        item_view::{ItemMenuOption, ItemView},
        member_view::{CliMemberView, MemberView},
    },
};
use shared::controller;

#[derive(Debug, Clone)]
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
    M: Model + LendingSystem + Clone,
    V: View + ItemView,
{
    fn display_item_info(&self) -> M {
        let items = self.model.get_items();
        let item: Option<&Item> = self.view.select_item(items);
        match item {
            Some(i) => {
                self.view.display_item_info(i);
                self.view.wait("");
                self.model.clone()
            }
            None => {
                self.view.wait("Nothing to select.");
                self.model.clone()
            }
        }
    }

    fn edit_item(&mut self) -> M {
        let model = self.model.clone();
        let items = model.get_items();
        let item_to_edit = self.view.select_item(items);
        let new_info = self.view.get_item_info();
        match item_to_edit {
            Some(i) => match self.model.update_item(i, &new_info) {
                Ok(_) => self.model.clone(),
                Err(_) => {
                    self.view.wait("Unable to update item information.");
                    self.model.clone()
                },
            },
            None => {
                self.view.wait("Unable to retrieve item.");
                self.model.clone()
            }
        }
    }

    fn create_item(&mut self) -> M {
        let model = self.model.clone();
        let mv: CliMemberView = CliMemberView::new();
        let members = model.get_members();
        let owner = mv.select_member(members);
        match owner {
            Some(o) => {
                let item = self.view.get_item_info().owner(o.clone()).build();
                match self.model.add_item(item) {
                    Ok(_) => self.view.wait("Item created successfully."),
                    Err(_) => self.view.wait("Unable to create Item, please try again."),
                }
            }
            None => self
                .view
                .wait("No members found. Cannot create item without an owner."),
        }
    }

    fn delete_item(&mut self) -> M {
        let model = self.model.clone();
        let items = model.get_items();
        let item_to_delete = self.view.select_item(items);
        match item_to_delete {
            Some(item) => match self.model.remove_item(item) {
                Ok(_) => self.view.wait("Successfully removed item."),
                Err(_) => self.view.wait("Unable to remove item"),
            },
            None => self.view.wait("Unable to retrieve item."),
        }
    }
}

impl<M, V> App<M> for ItemController<M, V>
where
    M: Model + LendingSystem + Clone,
    V: View + ItemView,
{
    fn run(&mut self, sys: M) -> M {
        let choice = self.view.item_menu();
        let state = match choice {
            ItemMenuOption::DisplayItemInfo => self.display_item_info(),
            ItemMenuOption::EditItemInfo => self.edit_item(),
            ItemMenuOption::CreateItem => self.create_item(),
            ItemMenuOption::DeleteItem => self.delete_item(),
            ItemMenuOption::Quit => std::process::exit(0),
            _ => sys,
        }
        self.run(state)
    }
}
