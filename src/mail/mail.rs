/// Protocols for receiving from mail servers
pub enum MailServer {
    Unknown,
    Imap(String),
    Pop(String),
}

impl MailServer {
    pub fn address(&self) -> (String, u16) {
        match self {
            MailServer::Unknown => ("Unknown".to_string(), 0),
            MailServer::Imap(domain) => (domain.clone(), 993),
            MailServer::Pop(domain) => (domain.clone(), 993),
        }
    }
}

pub struct Mail {
    sender: String,
    subject: String,
    content: String,
    // TODO
    // _file: !,
}

impl Mail {
    fn new(sender: String, subject: String, content: String) -> Self {
        Self {
            sender, subject, content
        }
    }


}