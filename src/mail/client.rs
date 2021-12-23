use termion::event::Key;
use tui::layout::Layout;

use crate::{event::{EventControl, Event}, r2ot::SubApp, utils::TextBox};

use super::{connection::ConnectionWidget, imap::ImapClient};

pub struct MailClient {
    receiver: ImapClient,
    connected: bool,
    connection_widget: ConnectionWidget,
}

impl MailClient {
    pub fn new() -> Self {
        Self {
            receiver: ImapClient::new(),
            connected: false,
            connection_widget: ConnectionWidget::new(),
        }
    }

    fn connect(&mut self) {

        let (email, password, hostname) = self.connection_widget.credentials();

        self.receiver.connect(email, password, hostname);

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

}

impl SubApp for MailClient {
    fn handle_event(&mut self, event: Event) -> EventControl {
        if !self.connected {
            self.handle_event_connection(event)
        } else {
            EventControl::OK
        }
    }

    fn draw<B:tui::backend::Backend>(&mut self, f: &mut tui::Frame<B>, rect: tui::layout::Rect) {
        // CONNECT MODE
        if !self.connected {
            self.connection_widget.draw(f, rect);
        } else {
            let text = tui::widgets::Paragraph::new("DES MAILS EN PAGAILLE".to_string());
            f.render_widget(text, rect);
        }
    }
}