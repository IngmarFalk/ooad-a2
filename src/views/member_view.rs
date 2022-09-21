use std::default;

use crate::{
    models::domain::{member::Member, Data},
    types::MembersList,
};

use super::console::Console;

pub trait MemberView {
    fn display_member_verbose(&self, member: Member);
    fn display_member_simple(&self, member: Member);
    fn ls_simple(&self, members: Vec<Member>);
    fn ls_verbose(&self, members: Vec<Member>);
    fn get_member_info(&self) -> Member;
    fn edit_member_info(&self, member: &mut Member) -> Member;
}

pub struct CliMemberView {
    console: Console,
}

impl CliMemberView {
    pub fn new() -> CliMemberView {
        CliMemberView {
            console: Console::new(),
        }
    }
}

impl MemberView for CliMemberView {
    fn display_member_verbose(&self, member: Member) {
        let mut items_str = String::new();
        if member.items().len() == 0 {
            items_str.push_str(" []")
        }
        for item in member.items().iter() {
            // let formatted = format!("\n\t{}", item);
            let formatted = format!("\n\t{},", item);
            items_str.push_str(&formatted);
        }
        let out = format!(
            "Name:\t\t{}\nEmail:\t\t{}\nPhone number:\t{}\nCredits:\t{}\nItems [{}\n]",
            member.name(),
            member.email(),
            member.phone_nr(),
            member.credits(),
            items_str
        );
        println!("{out}");
    }

    fn display_member_simple(&self, member: Member) {
        let out = format!(
            "Name:\t\t{}\nEmail:\t\t{}\nCredits:\t{}\nItems:\t\t{}\n",
            member.name(),
            member.email(),
            member.credits(),
            member.items().len(),
        );
        println!("{out}");
    }

    fn ls_simple(&self, members: MembersList) {
        let table = members.iter().map(|m| m.to_row()).collect();
        self.console.table(table);
    }

    fn ls_verbose(&self, members: Vec<Member>) {
        for m in members {
            self.display_member_verbose(m);
        }
    }

    fn get_member_info(&self) -> Member {
        Member::new(
            self.console.get_str_input("Name: "),
            self.console.get_str_input("Email: "),
            self.console.get_str_input("Phone number: "),
        )
    }

    fn edit_member_info(&self, member: &mut Member) -> Member {
        todo!()
    }
}
