use tui::layout::Layout;

use crate::{event::EventControl, r2ot::SubApp};

use super::ImapClient;

pub struct MailClient {
    receiver: ImapClient
}

impl SubApp for MailClient {
    fn handle_event(&mut self, _event: crate::event::Event) -> EventControl {
        EventControl::OK
    }

    fn draw<B:tui::backend::Backend>(&mut self, f: &mut tui::Frame<B>, rect: tui::layout::Rect) {

        // let chunks = Layout::default()
        //     .direction(self, direction);

        todo!()
    }
}