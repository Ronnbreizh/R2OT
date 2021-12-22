mod event;

pub use event::{events, Event};

pub enum EventControl {
    StopApp,
    OK, 
}