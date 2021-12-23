use std::net::TcpStream;

use imap::Session;
use native_tls::TlsStream;

use super::{Mail, mail::MailServer};

/// Generic email client able to receive and send mails
pub struct ImapClient {
    mail_server: MailServer,
    session: Option<Session<TlsStream<TcpStream>>>
}

impl ImapClient {
    pub fn new() -> Self {
        Self{
            mail_server: MailServer::Unknown,
            session: None,
        }
    }

    /// Select mail box
    pub fn select_mailbox(&mut self, mail_name: &str) {
        self.session.as_mut().unwrap().select(mail_name).unwrap();
    }

    /// Read all mails in current inbox
    fn read_mails(&mut self) -> Vec<Mail> {
        // fetch message number 1 in this mailbox, along with its RFC822 field.
        // RFC 822 dictates the format of the body of e-mails
        let messages = self.session.as_mut().unwrap().fetch("1", "RFC822").unwrap();
        let message = messages.first().unwrap();
        // extract the message's body

        let body = message.body().expect("message did not have a body!");
        let body = std::str::from_utf8(body)
            .expect("message was not valid utf-8")
            .to_string();

        println!("Message: {:?}", body);
        Vec::new()
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