use crate::{
    models::domain::{Data, FromMap, ToMap},
    types::{BufferVec, Model, StringMap},
};
use prettytable::{Row, Table};
use std::{
    collections::HashMap,
    io::{self, stdin, Write},
};
use thiserror::Error;

#[derive(Debug, Error)]
#[error("The Table to be displayed contained rows with different item counts.")]
pub struct InconsistentRowLength;

#[derive(Debug)]
pub struct Console;

impl Console {
    pub fn new() -> Console {
        Console {}
    }

    pub fn clear(&self) {
        if cfg!(windows) {
            std::process::Command::new("cls").status().unwrap();
        } else {
            std::process::Command::new("clear").status().unwrap();
        }
    }

    pub fn write(&self, out: &str) {
        print!("\n\t{out}: ")
    }

    pub fn writef(&self, out: &str) {
        self.title();
        self.write(out);
    }

    pub fn title(&self) {
        self.clear();

        println!("+  S T U F F   L E N D I N G   S Y S T E M   +");
        io::stdout().flush().unwrap();
    }

    pub fn confirm(&self, arg: String, val: String) -> bool {
        self.title();
        let str_raw = self.get_str_input(
            format!(
                "Are you sure you want to change ({}) to: {}. (y/n)",
                arg, val
            )
            .as_str(),
        );

        let chr = match str_raw.len() {
            1 => str_raw.chars().next().unwrap(),
            _ => 'q',
        };

        self.clear();

        match chr {
            c => match c {
                'y' => true,
                'n' => false,
                _ => self.confirm(arg, val),
            },
        }
    }

    pub fn table(&self, table: Table) {
        table.printstd()
    }

    pub fn row(&self, row: Row) {
        let mut _table = Table::new();
        _table.add_row(row);
        self.table(_table)
    }

    pub fn get_str_input(&self, display: &str) -> String {
        self.write(display);
        match io::stdout().flush() {
            Ok(_) => {}
            Err(err) => println!("There was some error displaying to console: {err}"),
        }
        let mut buf = String::new();
        match stdin().read_line(&mut buf) {
            Ok(_) => {}
            Err(_) => println!("There was a problem reading the input"),
        };
        buf.strip_suffix("\n").unwrap().to_owned()
    }

    pub fn get_consecutive_str_input(&self, input_buffers: Vec<String>) -> StringMap {
        self.title();
        let mut out = HashMap::new();
        for buf in input_buffers {
            let inp = self.get_str_input(buf.as_str());
            out.insert(buf, inp);
        }

        out
    }

    pub fn edit_model_info<T>(&self, obj: T) -> T
    where
        T: Data + FromMap + ToMap + Model,
    {
        let new_model_info = self.get_consecutive_str_input(obj.head_allowed_mutable());
        let data: StringMap = HashMap::from(
            new_model_info
                .into_iter()
                .collect::<HashMap<String, String>>(),
        );
        obj.copy_with(data)
    }
}
