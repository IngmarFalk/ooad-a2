use super::app::App;
use crate::{
    models::system::LendingSystem,
    types::{Model, View},
    views::member_view::{MemberMenuOption, MemberView},
};
use shared::controller;

#[derive(Debug)]
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
    M: Model + LendingSystem,
    V: View + MemberView,
{
    fn display_member_simple(&mut self) {
        let members_vec = self.model.get_members();
        let member = self.view.select_member(members_vec);
        match member {
            Some(m) => {
                let number_of_items = self.model.count_items(&m);
                self.view.display_member_simple(m.clone(), number_of_items);
                self.view.wait("")
            }
            None => {}
        }
        self.run()
    }

    fn display_member_verbose(&mut self) {
        let members_vec = self.model.get_members();
        let member = self.view.select_member(members_vec);
        match member {
            Some(m) => {
                let items = self.model.get_items_for_member(&m);
                self.view.display_member_verbose(m.clone(), items);
                self.view.wait("")
            }
            None => self.view.wait("Something went wrong."),
        }
        self.run()
    }

    fn create_member(&mut self) {
        let new_member = self.view.get_member_info();
        match self.model.add_member(new_member) {
            Ok(_) => {
                self.view.wait("Member created successfully.");
            }
            Err(_) => {
                self.view.wait("Unable to create member, please try again.");
            }
        }
        self.run();
    }
}

impl<M, V> App for MemberController<M, V>
where
    M: Model + LendingSystem,
    V: View + MemberView,
{
    fn run(&mut self) {
        let choice = self.view.member_menu();
        match choice {
            MemberMenuOption::DisplayMemberSimple => self.display_member_simple(),
            MemberMenuOption::DisplayMemberVerbose => self.display_member_verbose(),
            MemberMenuOption::ListAllMembersSimple => todo!(),
            MemberMenuOption::ListAllMembersVerbose => todo!(),
            MemberMenuOption::CreateMember => self.create_member(),
            MemberMenuOption::DeleteMember => todo!(),
            MemberMenuOption::EditMember => todo!(),
            MemberMenuOption::Other => todo!(),
            MemberMenuOption::Quit => std::process::exit(0),
        }
    }
}
