use super::console::{Console, Ui};
use crate::models::domain::item::Item;
use crate::models::domain::member::Member;
use crate::models::domain::Data;
use crate::views::Options;
use prettytable::{Cell, Row, Table};
use shared::{COptions, View};
use std::str::FromStr;

#[derive(Debug, COptions)]
pub enum MemberMenuOption {
    DisplayMemberSimple,
    DisplayMemberVerbose,
    ListAllMembersSimple,
    ListAllMembersVerbose,
    CreateMember,
    DeleteMember,
    EditMember,
    Quit,
    Back,
    #[other]
    Other,
}

pub trait MemberView {
    fn member_menu(&self) -> MemberMenuOption;
    fn display_member_verbose(&self, member: &Member, items: Vec<&Item>);
    fn display_member_simple(&self, member: &Member, number_of_items: usize);
    fn display_all_simple(&self, members: Vec<(&Member, usize)>);
    fn display_all_verbose(&self, members: Vec<(&Member, Vec<&Item>)>);
    fn get_member_info(&self) -> Member;
    fn edit_member_info(&self, member: Member) -> Option<Member>;
    fn select_member<'a>(&'a self, members: Vec<&'a Member>) -> Option<&Member>;
    fn wait(&self, display: &str);
}

#[derive(View)]
pub struct CliMemberView {
    console: Console,
}

impl MemberView for CliMemberView {
    fn member_menu(&self) -> MemberMenuOption {
        self.console.title();
        let choice: MemberMenuOption = self.console.show_menu(MemberMenuOption::options());
        match choice {
            MemberMenuOption::Other => self.member_menu(),
            _ => choice,
        }
    }

    fn display_member_verbose(&self, member: &Member, items: Vec<&Item>) {
        let mut items_str = String::new();
        if items.len() == 0 {
            items_str.push_str("[]")
        }
        for item in items.iter() {
            let formatted = format!(
                "\n\t({}\n\t{}\n\t{}\n\t{}),",
                item.get_name(),
                item.get_description(),
                item.get_category(),
                item.get_cost_per_day()
            );
            items_str.push_str(&formatted);
        }
        let out = format!(
            "Name:\t\t{}\nEmail:\t\t{}\nPhone number:\t{}\nCredits:\t{}\nItems: [{}]\n",
            member.get_name(),
            member.get_email(),
            member.get_phone_nr(),
            member.get_credits(),
            items_str,
        );
        self.console.writef(out.as_str());
    }

    fn display_member_simple(&self, member: &Member, number_of_items: usize) {
        self.console.clear();
        let out = format!(
            "Name:\t\t{}\nEmail:\t\t{}\nCredits:\t{}\nItems:\t\t{}\n",
            member.get_name(),
            member.get_email(),
            member.get_credits(),
            number_of_items,
        );
        self.console.writef(out.as_str());
    }

    fn display_all_simple(&self, data: Vec<(&Member, usize)>) {
        self.console.clear();
        let mut table = Table::new();
        let mut head = Row::from(Member::head());
        let number_of_items = Cell::new("Number of Items");
        head.add_cell(number_of_items);
        table.add_row(head);
        for entry in data {
            let mut row = entry.0.to_row();
            let cell = Cell::new(&entry.1.to_string());
            row.add_cell(cell);
            table.add_row(row);
        }
        self.console.display_table(table);
        self.wait("")
    }

    fn display_all_verbose(&self, data: Vec<(&Member, Vec<&Item>)>) {
        self.console.clear();
        let mut table = Table::new();
        let mut head = Row::from(Member::head());
        let items = Cell::new("Items");
        head.add_cell(items);
        table.add_row(head);
        for entry in data {
            let mut buf = String::new();
            for item in entry.1 {
                buf.push_str(item.to_string().as_str());
            }
            let mut row = entry.0.to_row();
            let cell = Cell::new(buf.as_str());
            row.add_cell(cell);
            table.add_row(row);
        }
        self.console.display_table(table);
        self.wait("")
    }

    fn get_member_info(&self) -> Member {
        let new_member = Member::default();
        self.console.get_model_info(new_member)
    }

    fn edit_member_info(&self, member: Member) -> Option<Member> {
        self.console.edit_model_info(member)
    }

    fn select_member<'a>(&'a self, members: Vec<&'a Member>) -> Option<&Member> {
        self.console.select_model::<Member>(members)
    }

    fn wait(&self, display: &str) {
        self.console.wait(display);
    }
}
