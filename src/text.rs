use std::convert::From;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use crate::line::Line;

pub struct Text {
    lines: Vec<Line>,
}

impl Text {
    pub fn empty() -> Self {
        Text {
            lines: vec![Line::empty()],
        }
    }

    pub fn get_line(&self, index: usize) -> Option<&str> {
        match self.lines.get(index) {
            Some(line) => Some(line.get_content()),
            None => None,
        }
    }

    pub fn append(&mut self, c: char) {
        match c {
            '\n' => self.lines.push(Line::empty()),
            _ => match self.lines.last_mut() {
                Some(line) => line.append(c),
                None => self.lines.push(Line::from(c)),
            },
        }
    }

    pub fn append_at_line(&mut self, idx: usize, c: char) {
        match self.lines.get_mut(idx) {
            Some(line) => match c {
                '\n' => self.lines.insert(idx, Line::empty()),
                _ => line.append(c),
            },
            None => eprint!("No line with index {}", idx),
        }
    }

    pub fn insert(&mut self, idx: usize, at: usize, c: char) {
        match self.lines.get_mut(idx) {
            Some(line) => match c {
                '\n' => {
                    let new_line = line.split(at);

                    self.lines.insert(idx + 1, new_line)
                }
                _ => line.insert(idx, c),
            },
            None => eprint!("No line with index {}", idx),
        }
    }

    pub fn delete(&mut self, row: usize, column: usize) {
        let row_i = row;

        match self.lines.get_mut(row_i) {
            Some(row) => {
                row.remove(column);
            }
            None => eprint!("Row {} does not exist", row_i),
        }
    }
}

impl From<File> for Text {
    fn from(file: File) -> Self {
        let mut lines: Vec<Line> = vec![];

        for line in BufReader::new(file).lines() {
            match line {
                Ok(line) => {
                    lines.push(Line::from(line));
                }
                _ => (),
            }
        }

        Text { lines }
    }
}
