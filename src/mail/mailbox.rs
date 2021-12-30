use imap::types::Mailbox;
use termion::event::Key;
use tui::{layout::{Constraint, Layout, Direction}, widgets::{List, ListItem, Borders, Block, ListState, Paragraph}, style::{Modifier, Style, Color}, text::Span};

use crate::{r2ot::SubApp, event::{EventControl, Event}};

use super::{imap::ImapClient, Mail};

#[derive(PartialEq)]
enum MailboxWidget {
    Mailboxes,
    Mails,
}

pub struct MailBox {
    receiver: ImapClient,
    // list of mailboxes
    mailboxes: Vec<String>,
    mailboxes_state: ListState,

    mailbox_loaded: bool,

    // list of messages in current mailbox   
    mails: Vec<Mail>,
    mails_state: ListState,

    // selected widget
    selected_widget: MailboxWidget,

}

impl MailBox {
    pub fn new() -> Self {
        Self {
            receiver: ImapClient::new(),

            mailboxes: Vec::new(),
            mailboxes_state: ListState::default(),
            mailbox_loaded: false,

            mails: Vec::new(),
            mails_state: ListState::default(),

            selected_widget: MailboxWidget::Mailboxes,
        }
    }

    pub fn connect(&mut self, email: String, password: String, hostname: String) {
        self.receiver.connect(email, password, hostname);
        let mailboxes = self.receiver.retrieve_mailboxes();
        self.set_mailboxes(mailboxes);
    }

    pub fn set_mailboxes(&mut self, mailboxes: Vec<String>) {
        self.mailboxes_state.select(Some(0));
        self.mailboxes = mailboxes;
    }

    fn select_mailbox(&mut self) {
        let id = self.mailboxes_state.selected().unwrap();
        let mailbox_name = &self.mailboxes[id];

        self.receiver.select_mailbox(mailbox_name);
        self.mailbox_loaded = true;

        self.load_mails();
    }

    // load mails in current mailbox
    fn load_mails(&mut self) {
        self.mails = self.receiver.read_mails();
        self.mails_state.select(Some(0));
    }

    fn handle_event_mailbox(&mut self, event: Event) -> EventControl {
        match event {
            Event::Input(Key::Down) => {
                let mut id = self.mailboxes_state.selected().unwrap() + 1;
                id %= self.mailboxes.len();
                self.mailboxes_state.select(Some(id));
                EventControl::OK
            },
            Event::Input(Key::Up) => {
                let mut id = self.mailboxes_state.selected().unwrap().saturating_sub(1);
                id %= self.mailboxes.len();
                self.mailboxes_state.select(Some(id));
                EventControl::OK
            },
            Event::Input(Key::Char('\n')) => {
                self.select_mailbox();
                EventControl::OK
            }
            _ => {
                EventControl::OK
            }
        }
    }

    fn handle_event_mail(&mut self, event: Event) -> EventControl {
        match event {
            Event::Input(Key::Down) => {
                let mut id = self.mails_state.selected().unwrap() + 1;
                id %= self.mails.len();
                self.mails_state.select(Some(id));
                EventControl::OK
            },
            Event::Input(Key::Up) => {
                let mut id = self.mails_state.selected().unwrap().saturating_sub(1);
                id %= self.mails.len();
                self.mails_state.select(Some(id));
                EventControl::OK
            },
            _ => {
                EventControl::OK
            }
        }
    }
}

impl SubApp for MailBox {
    fn handle_event(&mut self, event: Event) -> EventControl {

        match event {
            Event::Input(Key::Left) => {
                self.selected_widget = MailboxWidget::Mailboxes;
                EventControl::OK
            },
            Event::Input(Key::Right) => {
                self.selected_widget = MailboxWidget::Mails;
                EventControl::OK
            }
            _ => {
                match self.selected_widget {
                    MailboxWidget::Mailboxes => self.handle_event_mailbox(event),
                    MailboxWidget::Mails => self.handle_event_mail(event),
                }
            }
        }
    }

    fn draw<B:tui::backend::Backend>(&mut self, f: &mut tui::Frame<B>, rect: tui::layout::Rect) {
        let chunks = Layout::default()
            .constraints([Constraint::Length(20), Constraint::Min(8)].as_ref())
            .direction(Direction::Horizontal)
            .split(rect);


        // mailboxes

        let items : Vec<ListItem> = self.mailboxes.iter().map(|name| ListItem::new(Span::from(name.clone()))).collect();
        let list = List::new(items)
            .block(Block::default().title("Mailboxes").borders(Borders::ALL))
            .highlight_symbol(">>");        

        let list = if self.selected_widget != MailboxWidget::Mailboxes {
            list
                .style(Style::default().fg(Color::DarkGray))
        } else {
            list
                .highlight_style(Style::default().add_modifier(Modifier::ITALIC).bg(Color::Green))
        };

        f.render_stateful_widget(list, chunks[0], &mut self.mailboxes_state);


        let chunks = Layout::default()
            .constraints([Constraint::Length(20), Constraint::Min(8)].as_ref())
            .direction(Direction::Horizontal)
            .split(chunks[1]);

        // mails
        let items : Vec<ListItem> = self.mails.iter().map(|mail| ListItem::new(Span::from(format!("{}|{}", mail.sender, mail.subject)))).collect();
        let list = List::new(items)
            .block(Block::default().title("Mails").borders(Borders::ALL))
            .style(Style::default())
            .highlight_symbol(">>");        

        let list = if self.selected_widget != MailboxWidget::Mails {
            list
                .style(Style::default().fg(Color::DarkGray))
        } else {
            list
                .highlight_style(Style::default().add_modifier(Modifier::ITALIC).bg(Color::Green))
        };
        f.render_stateful_widget(list, chunks[0], &mut self.mails_state);

        // selected mail
        let mail_content = if self.mailbox_loaded {
            let mail_id = self.mails_state.selected().unwrap();
            let mail_content = &self.mails[mail_id];
            Span::from(mail_content.content.clone())
        } else {
            Span::from("")
        };

        let body = Paragraph::new(mail_content)
            .block(Block::default().title("Body").borders(Borders::ALL));

        f.render_widget(body, chunks[1]);
    }
}