use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use std::sync::mpsc::Sender;
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

pub fn event_thread(tick_rate: Duration, sender: Sender<Option<KeyEvent>>) -> JoinHandle<()> {
    thread::spawn(move || {
        let mut last_tick = Instant::now();

        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).unwrap_or_else(|_| false) {
                if let Event::Key(key) = event::read().unwrap_or_else(|_| {
                    Event::Key(KeyEvent::new(KeyCode::Null, KeyModifiers::NONE))
                }) {
                    sender.send(Some(key)).unwrap();
                }
            }

            if last_tick.elapsed() >= tick_rate {
                sender.send(None).unwrap_or_else(|_| ());
                last_tick = Instant::now();
            }
        }
    })
}
