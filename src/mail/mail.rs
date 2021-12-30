use mailparse::MailHeaderMap;

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
    pub sender: String,
    pub subject: String,
    pub content: String,
    // TODO
    // _file: !,
}

impl Mail {
    /// TODO
    pub fn parse(body: &[u8]) -> Self {

        let parsed_mail = mailparse::parse_mail(body).unwrap();

        Self {
            subject: parsed_mail.headers.get_first_value("Subject").unwrap(),
            sender: parsed_mail.headers.get_first_value("From").unwrap(),
            content: parsed_mail.get_body().unwrap(),
        }
    }


}