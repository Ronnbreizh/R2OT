use termion::event::Key;
use tui::{widgets::{Paragraph, Block, Borders}, style::{Style, Color}, text};

use crate::{r2ot::SubApp, event::{Event, EventControl}};

/// Widget to contain text entry
pub struct TextBox {
    selected: bool,
    password_mode: bool,
    message: String,
    title: String,
}

impl TextBox {
    pub fn new(field_name: String) -> Self {
        Self {
            selected: false,
            password_mode: false,
            message: String::new(),
            title: field_name,
        }
    }

    pub fn set_selected(&mut self, selected: bool) {
        self.selected = selected;
    }

    /// Set entry in passord mode
    pub fn password_mode(&mut self) {
        self.password_mode = true;
    }

    /// Return content of the textbox
    pub fn content(&self) -> String {
        self.message.clone()
    }
}

impl SubApp for TextBox {
    fn handle_event(&mut self, event: crate::event::Event) -> EventControl {
        match event {
            Event::Input(key) => match key {
                Key::Backspace => {self.message.pop();},
                Key::Left => todo!(),
                Key::Right => todo!(),
                Key::End => todo!(),
                Key::Char(ch) => self.message.push(ch),
                _ => (),
            },
            Event::Tick => todo!(),
        }
        EventControl::OK
    }

    fn draw<B:tui::backend::Backend>(&mut self, f: &mut tui::Frame<B>, rect: tui::layout::Rect) {
        let display = if self.password_mode {
            str::repeat("*", self.message.len())
        } else {
            self.message.clone()
        };

        let textbox = if self.selected {
            Paragraph::new(display)
                .block(Block::default().borders(Borders::ALL).title(self.title.clone()))
        } else {
            Paragraph::new(display)
                .block(Block::default().borders(Borders::ALL).title(self.title.clone()))
                .style(Style::default().fg(Color::DarkGray))
        };

        f.render_widget(textbox, rect);
    }
}