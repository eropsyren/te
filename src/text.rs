use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

const DEFAULT_ROW_LENGTH: usize = 120;

pub struct Text {
    rows: Vec<Box<Vec<char>>>,
}

impl Text {
    pub fn empty() -> Text {
        Text { rows: vec![] }
    }

    pub fn from(file: File) -> Text {
        let mut rows: Vec<Box<Vec<char>>> = vec![];

        for line in BufReader::new(file).lines() {
            match line {
                Ok(line) => {
                    let line: Vec<char> = line.chars().collect();

                    rows.push(Box::new(line));
                }
                _ => (),
            }
        }

        Text { rows }
    }

    pub fn get_row(&self, index: usize) -> Option<&Box<Vec<char>>> {
        self.rows.get(index)
    }

    pub fn get_rows(&self, from: usize, to: usize) -> &[Box<Vec<char>>]  {
        &self.rows[from..to]
    }

    pub fn append_row(&mut self, row: usize, c: char) {
        let row_i = row;
        let row = self.rows.get_mut(row_i);

        match row {
            Some(row) => match c {
                '\n' => self
                    .rows
                    .insert(row_i, Box::new(Vec::with_capacity(DEFAULT_ROW_LENGTH))),
                _ => row.push(c),
            },
            None => eprint!("No such row: {}", row_i),
        }
    }

    pub fn insert(&mut self, row: usize, column: usize, c: char) {
        let row_i = row;
        let row = self.rows.get_mut(row_i);

        match row {
            Some(row) => match c {
                '\n' => self
                    .rows
                    .insert(row_i, Box::new(Vec::with_capacity(DEFAULT_ROW_LENGTH))),
                _ => row.insert(column, c),
            },
            None => eprint!("Error inserting character at: {}{}", row_i, column),
        }
    }

    pub fn delete(&mut self, row: usize, column: usize) {
        let row_i = row;

        match self.rows.get_mut(row_i) {
            Some(row) => {
                row.remove(column);
            }
            None => eprint!("Row {} does not exist", row_i),
        }
    }
}
