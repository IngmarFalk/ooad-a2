use super::Options;
use crate::{
    models::domain::{Data, FromMap, ToMap},
    types::{Model, StringMap},
};
use prettytable::{Row, Table};
use std::{
    collections::HashMap,
    io::{self, stdin, Write},
};

pub trait Ui {
    fn show_menu<T>(&self, menu_options: Vec<String>) -> T
    where
        T: Options + std::fmt::Display + std::str::FromStr;
    fn confirm(&self, original: String, new: String) -> bool;
    fn display_table(&self, table: Table);
    fn display_row(&self, row: Row);
    fn get_str_input(&self, display: &str) -> String;
    fn get_int_input(&self, display: &str) -> usize;
    fn get_char_input(&self, display: &str) -> char;
    fn get_consecutive_str_input(&self, display_strings: Vec<String>) -> StringMap;
    fn select_model<'a, T>(&'a self, vec_model: Vec<&'a T>) -> Option<&T>
    where
        T: Data + FromMap + ToMap + Model;
    fn edit_model_info<T>(&self, model: T) -> Option<T>
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
        print!("\n{out}: ")
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

    pub fn get_model_info<T>(&self, obj: T) -> T
    where
        T: Data + FromMap + ToMap + Model,
    {
        self.title();
        let new_model_info = self.get_consecutive_str_input(T::head_allowed_mutable());
        let data: StringMap = HashMap::from(
            new_model_info
                .into_iter()
                .collect::<HashMap<String, String>>(),
        );
        obj.copy_with(data)
    }
}

impl Ui for Console {
    fn show_menu<T>(&self, menu_options: Vec<String>) -> T
    where
        T: Options + std::str::FromStr + std::fmt::Display,
    {
        self.title();
        let out = menu_options
            .iter()
            .enumerate()
            .map(|(cnt, opt)| "\t".to_owned() + cnt.to_string().as_str() + "\t:\t" + opt)
            .collect::<Vec<String>>()
            .join("\n");
        let choice = T::from_choice(self.get_int_input(out.as_str()));
        choice
    }

    fn confirm(&self, arg: String, val: String) -> bool {
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
                'Y' => true,
                'n' => false,
                'N' => false,
                _ => self.confirm(arg, val),
            },
        }
    }

    fn display_table(&self, table: Table) {
        table.printstd()
    }

    fn display_row(&self, row: Row) {
        let mut _table = Table::new();
        _table.add_row(row);
        self.display_table(_table)
    }

    fn get_str_input(&self, display: &str) -> String {
        self.write(display);
        match io::stdout().flush() {
            Ok(_) => {}
            Err(err) => println!("There was some error displaying to console: {err}"),
        }
        let mut buf = String::new();
        match stdin().read_line(&mut buf) {
            Ok(_) => {}
            Err(_) => {
                let out = format!("{}\nThere was a problem reading the input.", display);
                return self.get_str_input(out.as_str());
            }
        };

        buf.strip_suffix("\n").unwrap().to_owned()
    }

    fn get_int_input(&self, display: &str) -> usize {
        let raw = self.get_str_input(display);
        match raw.parse::<usize>() {
            Ok(out) => out,
            Err(_) => self.get_int_input(display),
        }
    }

    fn get_char_input(&self, display: &str) -> char {
        let buf = self.get_str_input(display);
        match buf.parse::<char>() {
            Ok(chr) => return chr,
            Err(_) => self.get_char_input(display),
        }
    }

    fn get_consecutive_str_input(&self, input_buffers: Vec<String>) -> StringMap {
        let mut out = HashMap::new();
        for buf in input_buffers {
            let inp = self.get_str_input(buf.as_str());
            out.insert(buf, inp);
        }

        out
    }

    fn select_model<'a, T>(&'a self, vec_model: Vec<&'a T>) -> Option<&T>
    where
        T: Data + FromMap + ToMap + Model,
    {
        let pages: Vec<&[&T]> = vec_model.chunks(10).collect::<Vec<_>>();
        self.title();

        for (idx, page) in pages.iter().enumerate() {
            let _page = page.clone();
            for (jdx, item) in _page.iter().enumerate() {
                let _item = item.clone();
                let out = format!("{}\t:\t{}", jdx, item);
                self.write(out.as_str());
            }
            let msg = format!("Press n (next) / p (previous) / q (quit) / number (select)\t: ");
            // self.write(msg.as_str());
            let inp = self.get_char_input(&msg);
            if let Ok(res) = inp.to_string().parse::<usize>() {
                return match res < 10 {
                    true => Some(vec_model[idx * 10 + res]),
                    false => self.select_model(vec_model.clone()),
                };
            } else {
                return match inp {
                    'n' => None,
                    'p' => None,
                    'q' => None,
                    _ => self.select_model(vec_model),
                };
            };
        }
        None
    }

    fn edit_model_info<T>(&self, obj: T) -> Option<T>
    where
        T: Data + FromMap + ToMap + Model,
    {
        self.title();
        let new_model_info = self.get_consecutive_str_input(T::head_allowed_mutable());
        let data: StringMap = HashMap::from(
            new_model_info
                .into_iter()
                .collect::<HashMap<String, String>>(),
        );
        let obj_map = obj.to_map_allowed_mutable();
        let temp: Vec<String> = T::head_allowed_mutable().clone();
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
