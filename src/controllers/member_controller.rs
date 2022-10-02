use super::app::App;
use crate::{
    models::{
        domain::{item::Item, member::Member},
        system::LendingSystem,
    },
    types::{Model, View},
    views::member_view::{MemberMenuOption, MemberView},
};
use shared::controller;

#[derive(Debug, Clone)]
#[controller(MemberView)]
pub struct MemberController<M, V>
where
    M: Model + LendingSystem,
    V: View + MemberView,
{
    model: M,
    view: V,
}

impl<M, V> MemberController<M, V>
where
    M: Model + LendingSystem + Clone,
    V: View + MemberView,
{
    fn display_member_simple(&mut self) -> M {
        let members_vec = self.model.get_members();
        let member = self.view.select_member(members_vec);
        match member {
            Some(m) => {
                let number_of_items = self.model.count_items_for_member(&m);
                self.view.display_member_simple(&m, number_of_items);
                self.view.wait("");
                self.model.clone()
            }
            None => self.model.clone(),
        }
    }

    fn display_member_verbose(&mut self) -> M {
        let members_vec = self.model.get_members();
        let member = self.view.select_member(members_vec);
        match member {
            Some(m) => {
                let items = self.model.get_items_for_member(&m);
                self.view.display_member_verbose(&m, items);
                self.view.wait("");
                self.model.clone()
            }
            None => {
                self.view.wait("Something went wrong.");
                self.model.clone()
            }
        }
    }

    fn create_member(&mut self) -> M {
        let new_member = self.view.get_member_info();
        match self.model.add_member(new_member) {
            Ok(_) => {
                self.view.wait("Member created successfully.");
                self.model.clone()
            }
            Err(_) => {
                self.view.wait("Unable to create member, please try again.");
                self.model.clone()
            }
        }
    }

    fn delete_member(&mut self) -> M {
        let model = self.model.clone();
        let member_to_delete: Option<&Member> = self.view.select_member(model.get_members());

        match member_to_delete {
            Some(m) => match self.model.remove_member(m) {
                Ok(_) => self.model.clone(),
                Err(_) => {
                    self.view.wait("There was a problem deleting the member.");
                    self.model.clone()
                }
            },
            None => {
                self.view.wait("Couldnt retrieve member.");
                self.model.clone()
            }
        }
    }

    fn edit_member(&mut self) -> M {
        let model = self.model.clone();
        let member_to_edit: Option<&Member> = self.view.select_member(model.get_members());
        let new_info = self.view.get_member_info();
        match member_to_edit {
            Some(mem) => match self.model.update_member(mem, &new_info) {
                Ok(_) => self.model.clone(),
                Err(_) => {
                    self.view
                        .wait("There was a problem updating the member information.");
                    self.model.clone()
                }
            },
            None => {
                self.view.wait("Couldnt retrieve member.");
                self.model.clone()
            }
        }
    }

    fn display_all_members_simple(&self) -> M {
        let members = self.model.get_members();
        let mut item_counts: Vec<usize> = Vec::new();
        for member in members.iter() {
            let cnt = self.model.count_items_for_member(member);
            item_counts.push(cnt);
        }
        let tples = members
            .into_iter()
            .zip(item_counts)
            .collect::<Vec<(&Member, usize)>>();
        self.view.display_all_simple(tples);
        self.model.clone()
    }

    fn display_all_members_verbose(&self) -> M {
        let members = self.model.get_members();
        let mut items: Vec<Vec<&Item>> = Vec::new();
        for member in members.iter() {
            let member_items = self.model.get_items_for_member(member);
            items.push(member_items);
        }
        let tples = members
            .into_iter()
            .zip(items)
            .collect::<Vec<(&Member, Vec<&Item>)>>();
        self.view.display_all_verbose(tples);
        self.model.clone()
    }
}

impl<M, V> App<M> for MemberController<M, V>
where
    M: Model + LendingSystem + Clone,
    V: View + MemberView,
{
    fn run(&mut self, sys: M) -> M {
        let choice = self.view.member_menu();
        let state = match choice {
            MemberMenuOption::DisplayMemberSimple => self.display_member_simple(),
            MemberMenuOption::DisplayMemberVerbose => self.display_member_verbose(),
            MemberMenuOption::ListAllMembersSimple => self.display_all_members_simple(),
            MemberMenuOption::ListAllMembersVerbose => self.display_all_members_verbose(),
            MemberMenuOption::CreateMember => self.create_member(),
            MemberMenuOption::DeleteMember => self.delete_member(),
            MemberMenuOption::EditMember => self.edit_member(),
            MemberMenuOption::Quit => std::process::exit(0),
            _ => sys,
        };
        self.run(state)
    }
}
