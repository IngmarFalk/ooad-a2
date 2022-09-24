use crate::{
    models::domain::{Data, FromMap, ToMap},
    types::{Model, StringMap},
};
use prettytable::{Row, Table};
use std::{
    collections::HashMap,
    io::{self, stdin, Write},
    iter::Map,
};

trait Ui {
    fn confirm(&self, original: String, new: String) -> bool;
    fn display_table(&self, table: Table);
    fn display_row(&self, row: Row);
    fn get_str_input(&self, display: &str) -> String;
    fn get_consecutive_str_input(&self, display_strings: Vec<String>) -> Map<String, String>;
    fn edit_model_info<T>(&self, model: T) -> T
    where
        T: Data + FromMap + ToMap + Model;
}

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
                "Are you sure you want to change \n\n\t{} to:\n\n\t{}. \n\n\t(y/n)",
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
        let mut out = HashMap::new();
        for buf in input_buffers {
            let inp = self.get_str_input(buf.as_str());
            out.insert(buf, inp);
        }

        out
    }

    pub fn edit_model_info<T>(&self, obj: T) -> Option<T>
    where
        T: Data + FromMap + ToMap + Model,
    {
        self.title();
        self.table(obj.to_table());
        let new_model_info = self.get_consecutive_str_input(obj.head_allowed_mutable());
        let data: StringMap = HashMap::from(
            new_model_info
                .into_iter()
                .collect::<HashMap<String, String>>(),
        );
        let obj_map = obj.to_map_allowed_mutable();
        let temp: Vec<String> = obj.head_allowed_mutable().clone();
        let values_tuples = temp
            .iter()
            .map(|s| (obj_map.get(s).unwrap(), data.get(s).unwrap()));
        let (keys, vals) =
            values_tuples
                .into_iter()
                .fold((String::new(), String::new()), |mut tpl, entry| {
                    tpl.0.push_str(format!("[ {} ]", entry.0).as_str());
                    tpl.1.push_str(format!("[ {} ]", entry.1).as_str());
                    tpl
                });
        if self.confirm(keys, vals) {
            return Some(obj.copy_with(data));
        }
        None
    }
}
