use crate::{
    models::system::LendingSystem,
    types::{Model, View},
    views::member_view::{MemberMenuOption, MemberView},
};

use super::app::App;

#[derive(Debug)]
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
    pub fn new(model: M, view: V) -> MemberController<M, V> {
        MemberController { model, view }
    }

    fn display_member_simple(&self) {
        let members_vec = self.model.get_members();
        let member = self.view.select_member(members_vec);
        match member {
            Some(m) => {
                let number_of_items = self.model.count_items(m.clone());
                self.view.display_member_simple(m.clone(), number_of_items)
            }
            None => {}
        }
        self.run()
    }
}

impl<M, V> App for MemberController<M, V>
where
    M: Model + LendingSystem,
    V: View + MemberView,
{
    fn run(&self) {
        let choice = self.view.member_menu();
        match choice {
            MemberMenuOption::DisplayMemberSimple => self.display_member_simple(),
            MemberMenuOption::DisplayMemberVerbose => todo!(),
            MemberMenuOption::ListAllMembersSimple => todo!(),
            MemberMenuOption::ListAllMembersVerbose => todo!(),
            MemberMenuOption::CreateMember => todo!(),
            MemberMenuOption::DeleteMember => todo!(),
            MemberMenuOption::EditMember => todo!(),
            MemberMenuOption::Other => todo!(),
        }
    }
}
