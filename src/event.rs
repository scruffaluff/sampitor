use crossterm::event::{self, Event, KeyEvent};
use std::sync::mpsc::Sender;
use std::thread::{self, JoinHandle};

pub fn event_thread(sender: Sender<Option<KeyEvent>>) -> JoinHandle<()> {
    thread::spawn(move || loop {
        if let Event::Key(key) = event::read().unwrap() {
            sender.send(Some(key)).unwrap();
        }
    })
}
