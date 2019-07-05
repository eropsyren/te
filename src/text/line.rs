use std::convert::From;

const DEFAULT_LINE_LENGTH: usize = 120;

pub struct Line {
    content: String,
}

impl Line {
    pub fn empty() -> Line {
        let mut content = String::with_capacity(DEFAULT_LINE_LENGTH);

        content.push('\0');

        Line { content }
    }

    pub fn get_content(&self) -> &str {
        &self.content
    }

    pub fn append(&mut self, c: char) {
        self.content.push(c)
    }

    pub fn insert(&mut self, idx: usize, c: char) {
        self.content.insert(idx, c)
    }

    pub fn remove(&mut self, idx: usize) {
        self.content.remove(idx);
    }

    pub fn split(&mut self, at: usize) -> Line {
        Line {
            content: self.content.split_off(at),
        }
    }
}

impl From<String> for Line {
    fn from(string: String) -> Self {
        Line { content: string }
    }
}

impl From<char> for Line {
    fn from(c: char) -> Self {
        let mut line = Line::empty();

        line.append(c);

        line
    }
}
