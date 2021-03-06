//! Keyboard input event handlers.

use crossterm::event::{self, Event, KeyEvent};
use std::sync::mpsc::Sender;
use std::thread::{self, JoinHandle};

/// Spawn a thread to offload polling for keyboard events.
///
/// # Panics
///
/// Will return `Err` if `sender` fails or events are unreadable.
pub fn handler(sender: Sender<Option<KeyEvent>>) -> JoinHandle<()> {
    thread::spawn(move || loop {
        if let Event::Key(key) = event::read().unwrap() {
            sender.send(Some(key)).unwrap();
        }
    })
}
