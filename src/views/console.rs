use chrono::format::format;

pub struct Console;

type Row = Vec<String>;
type Table = Vec<Row>;

impl Console {
    pub fn new() -> Console {
        Console {}
    }

    pub fn write(&self, out: String) {
        todo!()
    }

    pub fn writef(&self, out: String) {
        todo!()
    }

    pub fn table(&self, table: Table) {
        let length: usize = match table.first() {
            Some(row) => row.len() * 28 - 1,
            None => 0,
        };
        println!("{length}");

        let filler = ' ';
        println!("|{filler:->length$}|");
        for row in table {
            self.row(row)
        }
        println!("|{filler:->length$}|");
    }

    pub fn row(&self, row: Row) {
        let mut row_buf = String::from("|");
        for item in row {
            let item_buf = format!(" {item:<25} |");
            row_buf.push_str(&item_buf);
        }
        println!("{row_buf}")
    }
}
