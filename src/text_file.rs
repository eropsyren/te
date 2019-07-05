use std::convert::From;
use std::fs::File;
use std::path::Path;

use crate::Text;

pub struct TextFile {
    text: Text,
    path: Box<Path>,
    file: File,
}

impl TextFile {
    pub fn get_text(&mut self) -> &mut Text {
        &mut self.text
    }

    pub fn save(&self) {
        unimplemented!();
    } 
}

impl<'a> From<Box<Path>> for TextFile {
    fn from(path: Box<Path>) -> Self {
        let file = File::open(&path).expect("error opening file");

        TextFile {
            text: Text::from(&file),
            path,
            file,
        }
    }
}
