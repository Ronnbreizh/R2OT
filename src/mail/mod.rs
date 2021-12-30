mod client;
mod connection;
mod imap;
/// Module used to parse raw mails into displayable formats
mod mail;
mod mailbox;

pub use mail::Mail;

pub use client::MailClient;