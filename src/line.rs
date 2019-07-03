const DEFAULT_LINE_LENGTH: usize = 120;

pub struct Line {
    content: String,
}

impl Line {
    pub fn empty() -> Line {
        Line {
            content: String::with_capacity(DEFAULT_LINE_LENGTH),
        }
    }

    pub fn from_string(string: String) -> Line {
        Line { content: string }
    }

    pub fn from_char(c: char) -> Line {
        let mut line = Line::empty();

        line.append(c);

        line
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
