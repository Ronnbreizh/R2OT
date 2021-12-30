use std::net::TcpStream;

use imap::Session;
use native_tls::TlsStream;

use super::{Mail, mail::MailServer};

/// Generic email client able to receive and send mails
pub struct ImapClient {
    mail_server: MailServer,

    current_mail: Option<imap::types::Mailbox>,
    session: Option<Session<TlsStream<TcpStream>>>,
}

impl ImapClient {
    pub fn new() -> Self {
        Self{
            mail_server: MailServer::Unknown,
            session: None,
            current_mail: None,
        }
    }

    pub fn retrieve_mailboxes(&mut self) -> Vec<String> {
        let mailboxes = self.session.as_mut().unwrap()
            .list(Some("*"), Some("*")).unwrap();
        mailboxes.into_iter().map(|name| name.name().to_string()).collect()
    }

    /// Select mail box
    pub fn select_mailbox(&mut self, mail_name: &str) {
        let mailbox = self.session.as_mut().unwrap().select(mail_name).unwrap();
        self.current_mail = Some(mailbox);
    }

    /// Read all mails in current inbox
    pub fn read_mails(&mut self) -> Vec<Mail> {
        // RFC 822 dictates the format of the body of e-mails

        let last_message_id = self.current_mail.as_ref().unwrap().exists;

        let sequence = format!("{}:{}", last_message_id-20, last_message_id);

        let messages = self.session.as_mut().unwrap().fetch(sequence, "RFC822").unwrap();

        let mails = messages.into_iter()
            .map(|message| message.body().expect("No body..."))
            .map(|body| Mail::parse(body))
            .collect();
        mails
    }

    pub fn connect(&mut self, email: String, password: String, hostname: String) {
        let tls = native_tls::TlsConnector::builder()
            .build()
            .unwrap();

        self.mail_server = MailServer::Imap(hostname.clone());
        let client = imap::connect(self.mail_server.address(), hostname, &tls)
            .unwrap();
        let imap_session = client.login(email, password)
            .unwrap();
        self.session = Some(imap_session);
    }

    pub fn disconnect(&mut self) {
        self.session.as_mut().unwrap().logout().unwrap();
    }
}