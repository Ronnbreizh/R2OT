use termion::event::Key;

use crate::{event::{EventControl, Event}, r2ot::SubApp};

use super::{connection::ConnectionWidget, mailbox::MailBox};

pub struct MailClient {
    connected: bool,
    connection_widget: ConnectionWidget,
    mailbox: MailBox,
}

impl MailClient {
    pub fn new() -> Self {
        Self {
            connected: false,
            connection_widget: ConnectionWidget::new(),
            mailbox: MailBox::new(),
        }
    }

    fn connect(&mut self) {
        let (email, password, hostname) = self.connection_widget.credentials();
        self.mailbox.connect(email, password, hostname);
        self.connected = true;
    }

    fn handle_event_connection(&mut self, event: Event) -> EventControl {
        match event {
            Event::Input(Key::Char('\n')) => {
                self.connect();
                EventControl::OK
            },
            _ => {
                self.connection_widget.handle_event(event)
            }
        }
    }

    fn handle_event_mailbox(&mut self, event: Event) -> EventControl {
        self.mailbox.handle_event(event)
    }

}

impl SubApp for MailClient {
    fn handle_event(&mut self, event: Event) -> EventControl {
        if !self.connected {
            self.handle_event_connection(event)
        } else {
            self.handle_event_mailbox(event)
        }
    }

    fn draw<B:tui::backend::Backend>(&mut self, f: &mut tui::Frame<B>, rect: tui::layout::Rect) {
        // CONNECT MODE
        if !self.connected {
            self.connection_widget.draw(f, rect);
        } else {
            self.mailbox.draw(f, rect);
        }
    }
}