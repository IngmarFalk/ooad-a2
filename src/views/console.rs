use std::io::{self, stdin, Write};

use prettytable::Table;
use thiserror::Error;

type Row = Vec<String>;

#[derive(Debug, Error)]
#[error("The Table to be displayed contained rows with different item counts.")]
pub struct InconsistentRowLength;

#[derive(Debug)]
pub struct Console;

impl Console {
    pub fn new() -> Console {
        Console {}
    }

    pub fn write(&self, out: String) {
        todo!()
    }

    pub fn writef(&self, out: String) {
        // TODO figure out a format for regular messages
        // TODO Maybe also clear the screen before every message
        todo!()
    }

    pub fn confirm(&self, arg: String, val: String) -> bool {
        /// TODO Something like this: Are you sure you want to change `arg` to `val` ? (y/n):
        todo!()
    }

    pub fn table(&self, table: Table) {
        table.printstd()
    }

    pub fn row(&self, row: Row) {
        let mut row_buf = String::from("|");
        for item in row {
            let item_buf = format!(" {item:<25} |");
            row_buf.push_str(&item_buf);
        }
        println!("{row_buf}")
    }

    pub fn get_str_input(&self, display: &str) -> String {
        print!("{display}");
        match io::stdout().flush() {
            Ok(_) => {}
            Err(err) => println!("There was some error displaying to console: {err}"),
        }
        let mut buf = String::new();
        match stdin().read_line(&mut buf) {
            Ok(_) => {}
            Err(_) => println!("There was a problem reading the input"),
        };
        buf
    }
}
