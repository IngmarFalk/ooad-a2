use super::Options;
use crate::{
    models::domain::{item::Item, Data, FromMap, ToMap},
    types::Model,
};
use prettytable::{Cell, Row, Table};
use std::{
    collections::HashMap,
    fmt::Debug,
    io::{self, stdin, Write},
    str::FromStr,
};

/// Either<A, B> has three valid states:
///
/// `Left(A)`,
/// `Right(B)`,
/// `None`
#[derive(Debug, Clone, Copy)]
pub enum Either<A, B> {
    /// Left is always of type `A`
    Left(A),
    /// Right is always of type `B`
    Right(B),
    /// None represents an absent value.
    None,
}

impl<'a, A: 'a, B: 'a> Either<A, B> {
    /// This method will return an Option<A>. It takes as an input
    /// a function that returns another Either<A, B>.
    ///
    /// `None`   -> Neither::None.
    ///
    /// `Left`   -> Some(left).
    ///
    /// `Right`  -> calls function (fun(right)) and passes in right. Then it calls unwrap_left on the result of that
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

///
pub trait Ui {
    /// Shows a menu for an enum that implements the `Options` trait.
    ///
    /// It takes in a list of options (Vec<String>), matches it against the enum and returns the corresponding state.
    fn show_menu<T>(&self, menu_options: Vec<String>) -> T
    where
        T: Options + std::fmt::Display + std::str::FromStr;

    /// Confirms that the user wants to change a certain original element to a new state.
    fn confirm(&self, original: String, new: String) -> bool;

    /// Displays a table to the user.
    fn display_table(&self, table: Table);

    /// Displays a row to the user.
    fn display_row(&self, row: Row);

    /// Gets string input from user.
    fn get_str_input(&self, display: &str) -> String;

    /// Gets integer input from user.
    fn get_int_input(&self, display: &str) -> usize;

    /// Gets a sinlge character from user.
    fn get_char_input(&self, display: &str) -> char;

    /// Takes in a list of keys to ask the user for input and
    /// uses them to store the answers in a hashmap, which will then be returned.
    fn get_consecutive_str_input(&self, display_strings: Vec<String>) -> HashMap<String, String>;

    /// When displaying models, there are only 10 objects at a time that are actually being
    /// shown. This is called a page. This method shows one page at a time and allows the user to
    /// navigate back and forwards.
    fn display_page<'a, M>(
        &'a self,
        vec_model: Vec<&'a M>,
        chunks: Vec<&[&M]>,
        current_page: usize,
    ) -> Either<&M, usize>
    where
        M: Data + FromMap + ToMap + Model;

    /// Shows a list of models in pages of 10 and then lets the user
    /// select one of those.
    fn select_model<'a, M>(&'a self, vec_model: Vec<&'a M>) -> Option<&M>
    where
        M: Data + FromMap + ToMap + Model;

    /// Lets the user edit the information for a model.
    fn edit_model_info<T>(&self, model: &T) -> Option<T>
    where
        T: Data + FromMap + ToMap + Model;

    /// Displays a message to the user and waits for him to acknowledge the message
    ///  before continuing with the flow.
    fn wait(&self, display: &str);
}

/// This struct does ALL the displaying to the terminal.
/// Any tui display/interaction goes through the `Console` struct.
/// Views will have an attribute `console` to for user interaction
/// which they will use to display and get information to/from the user.
#[derive(Debug)]
pub struct Console;

impl Console {
    /// Creates a new Console.
    pub fn new() -> Console {
        Console {}
    }

    /// Cleas the screen.
    pub fn clear(&self) {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        io::stdout().flush().unwrap();
        // if cfg!(windows) {
        //     std::process::Command::new("cls").status().unwrap();
        // } else {
        //     std::process::Command::new("clear").status().unwrap();
        // }
    }

    /// Writes string to output with `\n` in front of it and flushes stdout.
    pub fn write(&self, out: &str) {
        print!("\n{out}");
        io::stdout().flush().unwrap();
    }

    /// Writes string to output with `\n` in front and a `:` behind it and flushes stdout.
    pub fn writef(&self, out: &str) {
        print!("\n{out}: ");
        io::stdout().flush().unwrap();
    }

    /// Displays `+ S T U F F   L E N D I N G   S Y S T E M  + to the terminal.
    pub fn title(&self) {
        self.clear();

        println!("+  S T U F F   L E N D I N G   S Y S T E M   +");
        io::stdout().flush().unwrap();
    }

    /// Collects information for any model that implements the following traits.
    ///
    /// - `Data`, `FromMap`, `ToMap`, `Model`
    pub fn get_model_info<T>(&self, obj: T) -> T
    where
        T: Data + FromMap + ToMap + Model,
    {
        self.title();
        let new_model_info = self.get_consecutive_str_input(T::head_allowed_mutable());
        let data = new_model_info
            .into_iter()
            .collect::<HashMap<String, String>>();
        obj.copy_with_map(data)
    }
}

impl Ui for Console {
    fn show_menu<T>(&self, menu_options: Vec<String>) -> T
    where
        T: Options + std::str::FromStr + std::fmt::Display,
    {
        let out = menu_options
            .iter()
            .enumerate()
            .map(|(cnt, opt)| "\t".to_owned() + cnt.to_string().as_str() + "\t:\t" + opt)
            .collect::<Vec<String>>()
            .join("\n");
        let inp = self.get_int_input((out + "\n").as_str());
        T::from_choice(inp)
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
            'y' => true,
            'Y' => true,
            'n' => false,
            'N' => false,
            _ => self.confirm(arg, val),
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

        match buf.strip_suffix('\n') {
            Some(val) => val.to_owned(),
            None => buf,
        }
    }

    fn get_int_input(&self, display: &str) -> usize {
        self.clear();
        self.title();
        let raw = self.get_str_input(display);
        match raw.parse::<usize>() {
            Ok(out) => out,
            Err(_) => self.get_int_input(display),
        }
    }

    fn get_char_input(&self, display: &str) -> char {
        let buf = self.get_str_input(display);
        match buf.parse::<char>() {
            Ok(chr) => chr,
            Err(_) => self.get_char_input(display),
        }
    }

    fn get_consecutive_str_input(&self, inputs: Vec<String>) -> HashMap<String, String> {
        let mut out = HashMap::new();
        for buf in inputs {
            let inp = self.get_str_input(buf.as_str());
            if inp.as_str() != "" {
                out.insert(buf, inp.replace('\n', ""));
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
        self.clear();

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
        table.set_titles(table_head);
        for (jdx, item) in _page.iter().enumerate() {
            let mut row = Row::new(vec![]);
            row.add_cell(Cell::new(jdx.to_string().as_str()));
            for key in head.iter() {
                let data = item.to_map();
                let cell_data = data.get(key).unwrap();
                println!("{key}: {cell_data}");
                self.wait("Waiting... 293482u4");
                match key.to_lowercase().as_str() {
                    "uuid" => {
                        let uuid_value = cell_data
                            .split(';')
                            .collect::<Vec<&str>>()
                            .last()
                            .unwrap()
                            .split(',')
                            .last()
                            .unwrap()
                            .replace(']', "");
                        row.add_cell(Cell::new(uuid_value.as_str()));
                    }
                    "owner" => {
                        let owner_name = cell_data
                            .split(';')
                            .collect::<Vec<&str>>()
                            .first()
                            .unwrap()
                            .split(',')
                            .last()
                            .unwrap();
                        row.add_cell(Cell::new(owner_name))
                    }
                    "lendee" => {
                        let owner_name = cell_data
                            .split(';')
                            .collect::<Vec<&str>>()
                            .first()
                            .unwrap()
                            .split(',')
                            .last()
                            .unwrap();
                        row.add_cell(Cell::new(owner_name))
                    }
                    "item" => match Item::from_str(cell_data) {
                        Ok(item) => row.add_cell(Cell::new(item.get_name())),
                        Err(_) => row.add_cell(Cell::new("Item")),
                    },
                    "history" => {
                        row.add_cell(Cell::new(if cell_data.starts_with("[owner,") {
                            "..."
                        } else {
                            "None"
                        }));
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
        let page_display = format!("Page: {} / {}", curr_page + 1, page_count);
        self.write(page_display.as_str());
        let msg = "Press \n\tn\t(next)\n\tp\t(previous)\n\tq\t(quit)\n\te\t(go back to menu)\n\t0..9\t(select)\n\t";
        let inp = self.get_char_input(msg);

        if let Ok(res) = inp.to_string().parse::<usize>() {
            return match res < chunks[curr_page].len() {
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
                ' ' => self.display_page(vec_model, chunks, curr_page),
                _ => self.display_page(vec_model, chunks, curr_page),
            };
        };
    }

    fn select_model<'a, M>(&'a self, vec_model: Vec<&'a M>) -> Option<&M>
    where
        M: Data + FromMap + ToMap + Model + Data,
    {
        if vec_model.is_empty() {
            self.wait("Nothing to select.");
            return None;
        }

        let pages: Vec<&[&M]> = vec_model.chunks(10).collect::<Vec<_>>();
        self.clear();
        self.title();

        // for c in pages.iter() {
        //     for item in c.iter() {
        //         println!("{:#?}", item.to_string());
        //         self.wait("Waiting..");
        //     }
        // }

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
        self.write("I you do not want to edit a certain parameter, simply hit enter without typing anything.");
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
            return Some(obj.copy_with_map(new_model_info));
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
