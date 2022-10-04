use super::Options;
use crate::{
    models::domain::{Data, FromMap, ToMap},
    types::{Model, StringMap},
};
use prettytable::{Cell, Row, Table};
use std::{
    collections::HashMap,
    io::{self, stdin, Write},
};

#[derive(Debug, Clone, Copy)]
pub enum Either<A, B> {
    Left(A),
    Right(B),
    None,
}

impl<'a, A: 'a, B: 'a> Either<A, B> {
    pub fn unwrap_left<F>(self, fun: F) -> Option<A>
    where
        F: Fn(B) -> Either<A, B>,
    {
        match self {
            Either::Left(left) => Option::Some(left),
            Either::Right(right) => fun(right).unwrap_left(fun),
            Either::None => Option::None,
        }
    }
}

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
    fn display_page<'a, M>(
        &'a self,
        vec_model: Vec<&'a M>,
        chunks: Vec<&[&M]>,
        current_page: usize,
    ) -> Either<&M, usize>
    where
        M: Data + FromMap + ToMap + Model;
    fn select_model<'a, M>(&'a self, vec_model: Vec<&'a M>) -> Option<&M>
    where
        M: Data + FromMap + ToMap + Model;
    fn edit_model_info<T>(&self, model: &T) -> Option<T>
    where
        T: Data + FromMap + ToMap + Model;
    fn wait(&self, display: &str);
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
        print!("\n{out}");
        io::stdout().flush().unwrap();
    }

    pub fn writef(&self, out: &str) {
        print!("\n{out}: ");
        io::stdout().flush().unwrap();
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
        let inp = self.get_int_input((out + "\n").as_str());
        let choice = T::from_choice(inp);
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
        table.printstd();
    }

    fn display_row(&self, row: Row) {
        let mut _table = Table::new();
        _table.add_row(row);
        self.display_table(_table)
    }

    fn get_str_input(&self, display: &str) -> String {
        self.writef(display);
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

    fn get_consecutive_str_input(&self, inputs: Vec<String>) -> StringMap {
        let mut out = HashMap::new();
        for buf in inputs {
            let inp = self.get_str_input(buf.as_str());
            if inp.as_str() != "" {
                out.insert(buf, inp);
            }
        }

        out
    }

    fn display_page<'a, M>(
        &'a self,
        vec_model: Vec<&'a M>,
        chunks: Vec<&[&M]>,
        curr_page: usize,
    ) -> Either<&M, usize>
    where
        M: Data + FromMap + ToMap + Model + Data,
    {
        if curr_page > chunks.len() {
            return Either::None;
        }
        let page = chunks[curr_page];
        let head = M::head();
        let _page = page.clone();
        let mut table = Table::new();
        let mut table_head = Row::new(vec![]);
        table_head.add_cell(Cell::new("Selection"));
        for key in head.iter() {
            table_head.add_cell(Cell::new(key.as_str()));
        }
        table.add_row(table_head);
        for (jdx, item) in _page.iter().enumerate() {
            let mut row = Row::new(vec![]);
            row.add_cell(Cell::new(jdx.to_string().as_str()));
            for key in head.iter() {
                let data = item.to_map();
                let cell_data = data.get(key).unwrap();
                match key.to_lowercase().as_str() {
                    "uuid" => {
                        let uuid_value = cell_data
                            .split(";")
                            .collect::<Vec<&str>>()
                            .last()
                            .unwrap()
                            .split(",")
                            .last()
                            .unwrap();
                        row.add_cell(Cell::new(uuid_value));
                    }
                    _ => {
                        row.add_cell(Cell::new(cell_data.as_str()));
                    }
                }
            }
            table.add_row(row);
        }
        self.display_table(table);

        let page_count = chunks.len();
        let page_display = format!("Page: {} / {}", curr_page, page_count);
        self.write(page_display.as_str());
        let msg = format!("Press \n\tn\t(next)\n\tp\t(previous)\n\tq\t(quit)\n\te\t(go back to menu)\n\t0..9\t(select)\n\t");
        let inp = self.get_char_input(&msg);

        if let Ok(res) = inp.to_string().parse::<usize>() {
            return match res < 10 {
                true => Either::Left(vec_model[curr_page * 10 + res]),
                false => self.display_page(vec_model.clone(), chunks.clone(), curr_page),
            };
        } else {
            return match inp {
                'n' => {
                    if curr_page < page_count {
                        Either::Right(curr_page + 1)
                    } else {
                        self.display_page(vec_model.clone(), chunks.clone(), curr_page)
                    }
                }
                'p' => {
                    if curr_page > 0 {
                        Either::Right(curr_page - 1)
                    } else {
                        self.display_page(vec_model.clone(), chunks.clone(), curr_page)
                    }
                }
                'q' => std::process::exit(0),
                'e' => Either::None,
                _ => self.display_page(vec_model, chunks, curr_page),
            };
        };
    }

    fn select_model<'a, M>(&'a self, vec_model: Vec<&'a M>) -> Option<&M>
    where
        M: Data + FromMap + ToMap + Model + Data,
    {
        if vec_model.is_empty() {
            return None;
        }
        let pages: Vec<&[&M]> = vec_model.chunks(10).collect::<Vec<_>>();
        self.clear();
        self.title();

        let fun = |page: usize| -> Either<&M, usize> {
            self.display_page(vec_model.clone(), pages.clone(), page.to_owned())
        };

        self.display_page(vec_model.clone(), pages.clone(), 0)
            .unwrap_left::<_>(fun)
    }

    fn edit_model_info<T>(&self, obj: &T) -> Option<T>
    where
        T: Data + FromMap + ToMap + Model,
    {
        self.title();
        let new_model_info = self.get_consecutive_str_input(T::head_allowed_mutable());
        let obj_map = obj.to_map_allowed_mutable();
        let values_tuples = new_model_info
            .iter()
            .map(|s| (obj_map.get(s.0).unwrap(), new_model_info.get(s.0).unwrap()));
        let (keys, vals) =
            values_tuples
                .into_iter()
                .fold((String::new(), String::new()), |mut tpl, entry| {
                    tpl.0.push_str(format!("[ {} ]", entry.0).as_str());
                    tpl.1.push_str(format!("[ {} ]", entry.1).as_str());
                    tpl
                });
        if self.confirm(keys, vals) {
            return Some(obj.copy_with(new_model_info));
        }
        None
    }

    fn wait(&self, display: &str) {
        let out = format!("{}\n\nPress Enter to continue", display);
        match self.get_str_input(out.as_str()).as_str() {
            "" => {}
            _ => self.wait(display),
        }
    }
}
