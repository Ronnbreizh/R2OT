use std::{collections::BTreeMap, net::TcpStream};

use imap::Session;
use native_tls::TlsStream;

use crate::{r2ot::SubApp, event::EventControl};

use super::{Mail, mail::MailServer};

/// Generic email client able to receive and send mails
pub struct ImapClient {
    email: String,
    password: String,
    mail_server: MailServer,

    session: Option<Session<TlsStream<TcpStream>>>
}

impl ImapClient {
    fn new() -> Self {
        Self{
            email: String::from("Unknown"),
            password: String::from("Unknown"),
            mail_server: MailServer::Unknown,
            session: None,
        }
    }

    fn set_password<'a>(&'a mut self, password: String) -> &'a mut Self {
        self.password = password;
        self
    }

    fn set_email<'a>(&'a mut self, email: String) -> &'a mut Self {
        self.email = email;
        self
    }

    fn set_server<'a>(&'a mut self, server: MailServer) -> &'a mut Self {
        self.mail_server = server;
        self
    }

    /// Select mail box
    fn select_mailbox(&mut self, mail_name: &str) {
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

    fn connect(&mut self) {
        let domain = "imap.gmail.com";
        let tls = native_tls::TlsConnector::builder().build().unwrap();
        let client = imap::connect(self.mail_server.address(), domain, &tls).unwrap();
        let imap_session = client.login(self.email.clone(), self.password.clone()).unwrap();
        self.session = Some(imap_session);
    }

    fn disconnect(&mut self) {
        self.session.as_mut().unwrap().logout().unwrap();
    }
}
