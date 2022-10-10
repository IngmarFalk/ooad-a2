use super::app::Page;
use crate::{
    models::domain::{item::Item, system::LendingSystem},
    types::{Model, View},
    views::{
        item_view::{ItemMenuOption, ItemView},
        member_view::{CliMemberView, MemberView},
    },
};
use shared::controller;

/// Item Controller.
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
    fn ret(&self, display: &str) -> M {
        self.view.wait(display);
        self.model.clone()
    }

    fn display_item_info(&self) -> M {
        let items = self.model.get_items();
        let item: Option<&Item> = self.view.select_item(items);
        match item {
            Some(i) => {
                self.view.display_item_info(i);
                self.ret("")
            }
            None => self.model.clone(),
        }
    }

    fn edit_item(&mut self) -> M {
        let model = self.model.clone();
        let items = model.get_items();
        let item_to_edit = self.view.select_item(items);
        match item_to_edit {
            Some(i) => {
                let new_info = self.view.edit_item_info(i);
                match new_info {
                    Some(info) => match self.model.update_item(&info) {
                        Ok(_) => self.ret("Updated item data successfully."),
                        Err(_) => self.ret("Unable to update item information."),
                    },
                    None => self.model.clone(),
                }
            }
            None => self.model.clone(),
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
                    Ok(_) => {
                        let mut updated_owner = o.clone();
                        match updated_owner.add_credits(100f64) {
                            Ok(_) => match self.model.update_member(o, &updated_owner) {
                                Ok(_) => self.ret("Item created successfully."),
                                Err(err) => self.ret(err.to_string().as_str()),
                            },
                            Err(err) => self.ret(err.to_string().as_str()),
                        }
                    }
                    Err(err) => self.ret(err.to_string().as_str()),
                }
            }
            None => self.model.clone(),
        }
    }

    fn delete_item(&mut self) -> M {
        let model = self.model.clone();
        let items = model.get_items();
        let item_to_delete = self.view.select_item(items);
        match item_to_delete {
            Some(item) => match self.model.remove_item(item) {
                Ok(_) => self.ret("Successfully removed item."),
                Err(_) => self.ret("Unable to remove item"),
            },
            None => self.model.clone(),
        }
    }
}

impl<M, V> Page<M> for ItemController<M, V>
where
    M: Model + LendingSystem + Clone,
    V: View + ItemView,
{
    fn show(&mut self, sys: M) -> M {
        let choice = self.view.item_menu();
        self.model = sys.clone();
        let state = match choice {
            ItemMenuOption::DisplayItemInfo => self.display_item_info(),
            ItemMenuOption::EditItemInfo => self.edit_item(),
            ItemMenuOption::CreateItem => self.create_item(),
            ItemMenuOption::DeleteItem => self.delete_item(),
            ItemMenuOption::Quit => std::process::exit(0),
            ItemMenuOption::Back => return sys,
            ItemMenuOption::Other => sys,
        };
        self.show(state)
    }
}
