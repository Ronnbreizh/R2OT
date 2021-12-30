use termion::event::Key;
use tui::layout::{Layout, Constraint};

use crate::{utils::TextBox, r2ot::SubApp, event::{Event, EventControl}};

enum CurrentField {
    Email,
    Password,
    MailServer,
}

impl CurrentField {
    fn next(&self) -> Self {
        match self {
            CurrentField::Email => CurrentField::Password,
            CurrentField::Password => CurrentField::MailServer,
            CurrentField::MailServer => CurrentField::Email,
        }
    }

    fn previous(&self) -> Self {
        match self {
            CurrentField::Email => CurrentField::MailServer,
            CurrentField::Password => CurrentField::Email,
            CurrentField::MailServer => CurrentField::Password,
        }
    }
}

/// Connection widget for email
pub struct ConnectionWidget {
    current_field: CurrentField,
    email: TextBox,
    password: TextBox,
    mail_server: TextBox,
}

impl ConnectionWidget {
    pub fn new() -> Self {
        let mut email = TextBox::new("username / email".to_string());
        email.set_selected(true);
        let mut password = TextBox::new("password".to_string());
        password.password_mode();
        let mail_server = TextBox::new("Imap server".to_string());

        Self {
            current_field: CurrentField::Email,
            email,
            password,
            mail_server,
        }
    }

    /// Return all credentials requiered to auth on mail server
    pub fn credentials(&self) -> (String, String, String) {
        (self.email.content(), self.password.content(), self.mail_server.content())
    }

    fn select_current_field(&mut self) {
        let current_box = match self.current_field {
            CurrentField::Email => &mut self.email,
            CurrentField::Password => &mut self.password,
            CurrentField::MailServer => &mut self.mail_server,
        };
        current_box.set_selected(true);
    }

    fn unselect_current_field(&mut self) {
        let current_box = match self.current_field {
            CurrentField::Email => &mut self.email,
            CurrentField::Password => &mut self.password,
            CurrentField::MailServer => &mut self.mail_server,
        };
        current_box.set_selected(false);
    }

}

impl SubApp for ConnectionWidget {
    fn handle_event(&mut self, event: Event) -> EventControl {
        match event {
            Event::Input(Key::Down) | Event::Input(Key::Char('\t')) => {
                self.unselect_current_field();
                self.current_field = self.current_field.next();
                self.select_current_field();
                return EventControl::OK
            },
            Event::Input(Key::Up) | Event::Input(Key::BackTab) => {
                self.unselect_current_field();
                self.current_field = self.current_field.previous();
                self.select_current_field();
                return EventControl::OK
            },
            
            _ => (),
        }

        let current_box = match self.current_field {
            CurrentField::Email => &mut self.email,
            CurrentField::Password => &mut self.password,
            CurrentField::MailServer => &mut self.mail_server,
        };

        current_box.handle_event(event)

    }

    fn draw<B:tui::backend::Backend>(&mut self, f: &mut tui::Frame<B>, rect: tui::layout::Rect) {
        let chunks = Layout::default()
            .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
            .split(rect);
        self.email.draw(f, chunks[0]);

        let chunks = Layout::default()
            .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
            .split(chunks[1]);
        self.password.draw(f, chunks[0]);

        let chunks = Layout::default()
            .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
            .split(chunks[1]);
        self.mail_server.draw(f, chunks[0]);


    }
}