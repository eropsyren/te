use std::convert::From;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::iter::IntoIterator;
use std::ops::Deref;

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

    pub fn get_line(&self, index: usize) -> Option<&Line> {
        self.lines.get(index)
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
                '\n' => self.lines.insert(idx + 1, Line::empty()),
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

    pub fn delete(&mut self, idx: usize, offset: usize) {
        match self.lines.get_mut(idx) {
            Some(row) => row.remove(offset),
            None => eprint!("Row {} does not exist", idx),
        }
    }
}

impl From<File> for Text {
    fn from(file: File) -> Self {
        let mut lines: Vec<Line> = vec![];

        for line in BufReader::new(file).lines() {
            match line {
                Ok(line) => lines.push(Line::from(line)),
                _ => (),
            }
        }

        Text { lines }
    }
}

impl<'a> IntoIterator for &'a Text {
    type Item = &'a Line;
    type IntoIter = std::slice::Iter<'a, Line>;

    fn into_iter(self) -> Self::IntoIter {
        self.lines.iter()
    }
}

impl Deref for Text {
    type Target = [Line];

    fn deref(&self) -> &Self::Target {
        &self.lines[..]
    }
}
