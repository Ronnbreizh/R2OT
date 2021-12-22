use std::{sync::mpsc::{Receiver, channel}, time::Duration};
use std::thread;
use std::io;

use termion::{event::Key, input::TermRead};

pub enum Event {
    Input(Key),
    Tick,
}

/// Create channel which pool events for stdin
/// Return the end point of this channel
pub fn events(tick_rate: Duration) -> Receiver<Event> {
    let (tx, rx) = channel();
    let keys_tx = tx.clone();
    // stdin event
    thread::spawn(move || {
        let stdin = io::stdin();
        for evt in stdin.keys() {
            if let Ok(key) = evt {
                if let Err(err) = keys_tx.send(Event::Input(key)) {
                    eprintln!("{}", err);
                    return;
                }
            }
        }
    });
    // Tick event
    thread::spawn(move || loop {
        if let Err(err) = tx.send(Event::Tick) {
            eprintln!("{}", err);
            break;
        }
        thread::sleep(tick_rate);
    });
    rx
}