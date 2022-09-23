// #![warn(missing_docs)]
// #![warn(clippy::missing_docs_in_private_items)]
#![crate_type = "proc-macro"]

use std::{collections::HashMap, str::FromStr};

use crate::{
    models::{
        cvec::CVec,
        domain::{item::Item, FromMap},
        uuid::Uuid,
    },
    views::member_view::{CliMemberView, MemberView},
};
use models::domain::member::Member;

pub mod controllers;
pub mod models;
pub mod types;
pub mod views;

// use crate::{controllers::app::App, models::system::System, views::main_view::MainView};

fn main() {
    let allan = Member::new(
        "Allan".to_owned(),
        "allan@enigma.com".to_owned(),
        "123456".to_owned(),
    );
    println!("{}", allan);

    let data = allan.to_string();
    //

    // println!("{}", allan.get_uuid());
    // let allan_data = "(name,Allan);
    //      (email,allan@enigma.com);
    //      (phone_nr,123456);
    //      (credits,0);
    //      (day_of_creation,2022-09-23 12:21:39.024237 +02:00);
    //      (uuid,(len,6);(uuid_type,Member);(value,1R942S));
    //      (items,)";

    // let chrs = allan_data.chars().collect::<Vec<char>>();
    // let open = 0u32;
    // let mut buf = String::new();
    // for (idx, chr) in chrs.iter().enumerate() {
    //     if chr.eq(&'(') {
    //         open += 1;
    //     }
    //     if chr.eq(&')') {
    //         open -= 1;
    //     }
    // }

    // let allan = Member::from_str(allan_data).ok().expect("Wrong format");
    // println!("{allan}");
    // let data = allan_data
    //     .split(";")
    //     .collect::<Vec<&str>>()
    //     .iter()
    //     .map(|item| {
    //         let strs = item.split(",").collect::<Vec<&str>>();
    //         let key = match strs.first() {
    //             Some(k) => k.replace("(", ""),
    //             None => String::new(),
    //         };
    //         let val = match strs.last() {
    //             Some(v) => v.replace(")", ""),
    //             None => String::new(),
    //         };
    //         let out: (String, String) = (key, val);
    //         out
    //     })
    //     .collect::<HashMap<String, String>>();

    // let allan1 = Member::new2(data);

    // println!("{:#?}", data);
    // let allan: Member = Member::from_complete_map(data);
    // let mview = CliMemberView::new();
    // mview.display_member_simple(allan);

    // let mut binding = BuilderTest::new();
    // let b: BuilderTest = BuilderTest::new()
    //     .attr1("Hello".to_owned())
    //     .attr2(Item::new(
    //         "Monopoly".to_owned(),
    //         "A Family Game".to_owned(),
    //         Category::Game,
    //         allan.clone(),
    //         20f64,
    //     ))
    //     .attr3(allan.clone())
    //     .attr4(42)
    //     .attr5(Uuid::default())
    //     .build();

    // println!("{b}");
    // let bmap = HashMap::from([("attr1".to_owned(), "Bob".to_owned())]);
    // let bnew = b.copy_with(bmap);
    // println!("{bnew}");

    // let data = HashMap::from([("attr1", "Hello")]);
    // let attr1 = data.get("attr1").unwrap().parse::<String>().unwrap();
    // println! {"{attr1}"};

    // let turing1 = Member::new(
    //     "Turing".to_owned(),
    //     "allan@enigma.com".to_owned(),
    //     "123".to_owned(),
    // );
    // let turing2 = Member::new(
    //     "Turing".to_owned(),
    //     "turing@enigma.com".to_owned(),
    //     "123456".to_owned(),
    // );
    // let turing3 = Member::new(
    //     "Turing".to_owned(),
    //     "turing@enigma.com".to_owned(),
    //     "123".to_owned(),
    // );
    // let mut members = Table::new();

    // members.add_row(allan.to_row());
    // members.add_row(turing1.to_row());
    // members.add_row(turing2.to_row());
    // members.add_row(turing3.to_row());

    // let con = Console::new();
    // let item = Item::new(
    //     "Monopoly".to_owned(),
    //     "A Family Game".to_owned(),
    //     Category::Game,
    //     allan,
    //     20f64,
    // );
    // let buffers = con.to_user_editable_buffers_map(item);
    // let new_buffers = con.get_consecutive_str_input(buffers);

    // for buffer in new_buffers {
    //     println!("{:?}", buffer);
    // }

    // con.confirm("Name".to_owned(), "Jeff".to_owned());
    // println!("{}", Category::from("Tool"));

    // let table = item.to_table();
    // con.table(table);
    // let mut h_map = HashMap::new();
    // let name: String = String::new();
    // let description: String = String::new();
    // let category: String = String::new();
    // let cost_per_day: String = String::new();
    // h_map.insert("Name", name);
    // h_map.insert("Description", description);
    // h_map.insert("Category", category);
    // h_map.insert("Cost Per Day", cost_per_day);
    // con.get_consecutive_str_input(h_map);
    // con.table(members);
    // let model = System::new();
    // let view = CliMainView::new();
    // let app = MainApp::new();
    // app.run(model, view);
}
