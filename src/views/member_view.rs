use prettytable::{Cell, Row, Table};

use super::console::Console;
use crate::models::domain::item::Item;
use crate::models::domain::member::Member;
use crate::models::domain::Data;

pub trait MemberView {
    fn display_member_verbose(&self, member: Member, items: Vec<Item>);
    fn display_member_simple(&self, member: Member, number_of_items: usize);
    fn ls_simple(&self, members: Vec<(Member, usize)>);
    fn ls_verbose(&self, members: Vec<(Member, Vec<Item>)>);
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
    fn display_member_verbose(&self, member: Member, items: Vec<Item>) {
        let mut items_str = String::new();
        if items.len() == 0 {
            items_str.push_str(" []")
        }
        for item in items.iter() {
            let formatted = format!("\n\t{},", item);
            items_str.push_str(&formatted);
        }
        let out = format!(
            "Name:\t\t{}\nEmail:\t\t{}\nPhone number:\t{}\nCredits:\t{}\nItems [{}\n]",
            member.get_name(),
            member.get_email(),
            member.get_phone_nr(),
            member.get_credits(),
            items_str
        );
        println!("{out}");
    }

    fn display_member_simple(&self, member: Member, number_of_items: usize) {
        let out = format!(
            "Name:\t\t{}\nEmail:\t\t{}\nCredits:\t{}\nItems:\t\t{}\n",
            member.get_name(),
            member.get_email(),
            member.get_credits(),
            number_of_items,
        );
        println!("{out}");
    }

    fn ls_simple(&self, data: Vec<(Member, usize)>) {
        let mut table = Table::new();
        let mut head = Row::from(Member::default().head());
        let number_of_items = Cell::new("Number of Items");
        head.add_cell(number_of_items);
        table.add_row(head);
        for entry in data {
            let mut row = entry.0.to_row();
            let cell = Cell::new(&entry.1.to_string());
            row.add_cell(cell);
            table.add_row(row);
        }
        self.console.table(table);
    }

    fn ls_verbose(&self, data: Vec<(Member, Vec<Item>)>) {
        for entry in data {
            self.display_member_verbose(entry.0, entry.1);
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
