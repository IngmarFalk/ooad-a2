use super::console::{Console, Ui};
use crate::models::domain::item::Item;
use crate::models::domain::member::Member;
use crate::models::domain::Data;
use crate::views::Options;
use prettytable::{Cell, Row, Table};
use shared::{DeriveOptions, View};
use std::str::FromStr;

/// All the options for the member menu.
#[derive(Debug, DeriveOptions)]
pub enum MemberMenuOption {
    /// Displaying a single member in a simple format.
    DisplayMemberSimple,
    /// Displaying a single meber in a verbose format.
    DisplayMemberVerbose,
    /// Displays all members in a simple format.
    ListAllMembersSimple,
    /// Displays all members in a verbose format.
    ListAllMembersVerbose,
    /// Creates a new member in the system.
    CreateMember,
    /// Deletes a member from the system.
    DeleteMember,
    /// Edits member information for a specific member.
    EditMember,
    /// Goes back to previous page.
    Back,
    /// Quits the entire application.
    Quit,
    #[other]
    /// Any other choice made by the user.
    Other,
}

/// Defines all methods required by a concrete implementaion of the member view.
pub trait MemberView {
    /// Displays all options for the member menu.
    fn member_menu(&self) -> MemberMenuOption;
    /// Displaying a sinlge member in a verbose format.
    fn display_member_verbose(&self, member: &Member, items: Vec<&Item>);
    /// Displaying a member in a simple format.
    fn display_member_simple(&self, member: &Member, number_of_items: usize);
    /// Displays all members in a simple format.
    fn display_all_simple(&self, members: Vec<(&Member, usize)>);
    /// Displays all members in a verbose format.
    fn display_all_verbose(&self, members: Vec<(&Member, Vec<&Item>)>);
    /// Getting information for a new member.
    fn get_member_info(&self) -> Member;
    /// Edits a single member.
    fn edit_member_info(&self, member: &Member) -> Option<Member>;
    /// Selecting a member from a list of options.
    fn select_member<'a>(&'a self, members: Vec<&'a Member>) -> Option<&Member>;
    /// Displays a message to the user and waits for him to respond.
    fn wait(&self, display: &str);
}

/// A concrete implementation of the member view.
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
        if items.is_empty() {
            items_str.push_str("[]")
        }
        for item in items.iter() {
            let formatted = format!(
                "\n\t(\n\t\t{}\n\t\t{}\n\t\t{}\n\t\t{}\n\t),",
                item.get_name(),
                item.get_description(),
                item.get_category(),
                item.get_cost_per_day()
            );
            items_str.push_str(&formatted);
        }
        let out = format!(
            "Name:\t\t{}\nEmail:\t\t{}\nPhone number:\t{}\nCredits:\t{}\nItems: [\n{}\n]\n",
            member.get_name(),
            member.get_email(),
            member.get_phone_nr(),
            member.get_credits(),
            items_str,
        );
        self.console.clear();
        self.console.title();
        self.console.write(out.as_str());
    }

    fn display_member_simple(&self, member: &Member, number_of_items: usize) {
        let out = format!(
            "Name:\t\t{}\nEmail:\t\t{}\nCredits:\t{}\nItems:\t\t{}\n",
            member.get_name(),
            member.get_email(),
            member.get_credits(),
            number_of_items,
        );
        self.console.clear();
        self.console.title();
        self.console.write(out.as_str());
    }

    fn display_all_simple(&self, data: Vec<(&Member, usize)>) {
        self.console.clear();
        if data.is_empty() {
            self.wait("No members to show.");
            return;
        }
        let mut table = Table::new();
        let mut head = Row::from(Member::head());
        let number_of_items = Cell::new("Number of Items");
        head.add_cell(number_of_items);
        table.set_titles(head);
        for entry in data {
            let mut row = entry.0.to_row();
            row.remove_cell(5);
            row.add_cell(Cell::new(entry.0.get_uuid().get_value()));
            let cell = Cell::new(&entry.1.to_string());
            row.add_cell(cell);
            table.add_row(row);
        }
        self.console.display_table(table);
        self.wait("")
    }

    fn display_all_verbose(&self, data: Vec<(&Member, Vec<&Item>)>) {
        self.console.clear();
        if data.is_empty() {
            self.wait("No members to show.");
            return;
        }
        let mut table = Table::new();
        let mut head = Row::from(Member::head());
        let items = Cell::new("Items");
        head.add_cell(items);
        table.set_titles(head);
        for entry in data {
            let mut buf = String::new();
            for item in entry.1 {
                let out = format!(
                    "Name: {}, Description: {}, Category: {}, Contracts: {}\n",
                    &item.get_name(),
                    &item.get_description(),
                    &item.get_category(),
                    &item.get_history().len,
                );
                buf.push_str(out.as_str())
            }

            let mut row = entry.0.to_row();
            row.remove_cell(5);
            row.add_cell(Cell::new(entry.0.get_uuid().get_value()));
            let cell = Cell::new(&buf);
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

    fn edit_member_info(&self, member: &Member) -> Option<Member> {
        self.console.edit_model_info(member)
    }

    fn select_member<'a>(&'a self, members: Vec<&'a Member>) -> Option<&Member> {
        self.console.select_model::<Member>(members)
    }

    fn wait(&self, display: &str) {
        self.console.wait(display);
    }
}
