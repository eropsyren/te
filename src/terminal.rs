extern crate termion;

use std::io::{Stdout, StdoutLock, Write};
use std::sync::mpsc::{self, Receiver, TryIter};
use std::thread;
use std::time::Duration;

use termion::clear;
use termion::cursor;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::raw::RawTerminal;
use termion::AsyncReader;

const INPUT_THREAD_SLEEP_MS: u64 = 50;

pub struct Terminal<'a> {
    stdout: RawTerminal<StdoutLock<'a>>,
    keys_reciever: Receiver<Key>,
    width: u16,
    height: u16,
}

impl<'a> Terminal<'a> {
    pub fn new(stdin: AsyncReader, stdout: &'a mut Stdout) -> Self {
        let stdout = stdout
            .lock()
            .into_raw_mode()
            .expect("Failed to enter stdout raw mode");

        let (keys_sender, keys_reciever) = mpsc::channel();
        let mut stdin = stdin.keys();

        thread::spawn(move || loop {
            let key = stdin.next();

            match key {
                Some(Ok(key)) => keys_sender.send(key).unwrap(),
                _ => thread::sleep(Duration::from_millis(INPUT_THREAD_SLEEP_MS)),
            }
        });

        let (width, height) = termion::terminal_size().expect("error getting terminal size");

        Terminal {
            stdout,
            keys_reciever,
            width,
            height,
        }
    }

    pub fn keys(&self) -> TryIter<'_, Key> {
        self.keys_reciever.try_iter()
    }

    pub fn size(&self) -> (u16, u16) {
        (self.width, self.height)
    }

    pub fn flush(&mut self) {
        self.stdout.flush().expect("error flushing Terminal stdout");
    }

    pub fn clear(&mut self) -> &Self {
        write!(self.stdout, "{}", clear::All,).expect("error clearing stdout");

        self
    }

    pub fn clear_line(&mut self, line_number: usize) -> &Self {
        write!(
            self.stdout,
            "{}{}",
            cursor::Goto(line_number as u16, 1),
            clear::CurrentLine,
        )
        .expect("error clearing stdout");

        self
    }

    pub fn clean_after_line(&mut self, line_number: usize) -> &Self {
        write!(
            self.stdout,
            "{}{}",
            cursor::Goto(line_number as u16, 1),
            clear::AfterCursor,
        )
        .expect("error clearing stdout");

        self
    }

    pub fn write_line(&mut self, line_number: usize, content: &str) -> &Self {
        write!(
            self.stdout,
            "{}{}",
            cursor::Goto(line_number as u16, 1),
            content,
        )
        .expect("error clearing stdout");

        self
    }
}
