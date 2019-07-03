extern crate termion;

use std::sync::mpsc::{self, Receiver, TryIter};
use std::thread;
use std::time::Duration;

use termion::event::Key;
use termion::input::TermRead;
use termion::AsyncReader;

const INPUT_THREAD_SLEEP_MS: u64 = 50;

pub struct InputFetcher {
    keys_reciever: Receiver<Key>,
}

impl InputFetcher {
    pub fn new(stdin: AsyncReader) -> Self {
        let mut stdin = stdin.keys();
        let (keys_sender, keys_reciever) = mpsc::channel();

        thread::spawn(move || loop {
            let key = stdin.next();

            match key {
                Some(Ok(key)) => keys_sender.send(key).unwrap(),
                _ => thread::sleep(Duration::from_millis(INPUT_THREAD_SLEEP_MS)),
            }
        });

        InputFetcher { keys_reciever }
    }

    pub fn try_fetch(&self) -> TryIter<'_, Key> {
        self.keys_reciever.try_iter()
    }
}
