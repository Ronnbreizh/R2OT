use tui::{backend::Backend, Frame, layout::Rect};

use crate::event::{Event, EventControl};

pub trait SubApp {
    fn handle_event(&mut self, event: Event) -> EventControl;

    fn draw<B:Backend>(&mut self, f: &mut Frame<B>, rect: Rect);
}


pub struct SubAppStub {
    message: String
}

impl SubAppStub {
    pub fn new(message: String) -> Self {
        Self{message}
    }
}

impl SubApp for SubAppStub {
    fn handle_event(&mut self, event: Event) -> EventControl {
        EventControl::OK
    }

    fn draw<B:Backend>(&mut self, f: &mut Frame<B>, rect: Rect) {
        let text = tui::widgets::Paragraph::new(self.message.clone());
        f.render_widget(text, rect);
    }
    
}