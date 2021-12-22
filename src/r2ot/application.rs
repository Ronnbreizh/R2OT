use std::{time::Duration, io::Stdout};

use termion::{raw::{IntoRawMode, RawTerminal}, event::Key};
use tui::{backend::TermionBackend, Terminal};

use crate::event::{events, Event, EventControl};

use super::subapp::{SubAppStub, SubApp};

pub type R2otTerminal = Terminal<TermionBackend<RawTerminal<Stdout>>>;

const SUB_APP_NUMBER: usize = 3;


pub struct Application<'a> {
    pub(super) app_name: &'a str,
    pub(super) tabs_titles: [&'a str; SUB_APP_NUMBER], 
    pub(super) current_tab: usize,
    pub(super) mail_client : SubAppStub,
    pub(super) calendar : SubAppStub,
    pub(super) task_manager : SubAppStub,
}

impl<'a> Application<'a> {
    pub fn new() -> Self {
        Self {
            // generic app
            app_name: "r2ot",
            // tab related
            current_tab: 0,
            tabs_titles: ["emails", "calendar", "tasks"],
            // other
            mail_client : SubAppStub::new("test".to_string()),
            calendar :SubAppStub::new("Lol".to_string()),
            task_manager : SubAppStub::new("Micha".to_string()),
        }
    }

    fn prepare_env() -> R2otTerminal {
        let stdout = std::io::stdout().into_raw_mode().unwrap();

        let backend = TermionBackend::new(stdout);

        let mut terminal = Terminal::new(backend).unwrap();

        terminal.hide_cursor().unwrap();
        terminal.clear().unwrap();
        terminal
    }

    fn clear_env(mut terminal: R2otTerminal) {
        terminal.clear().unwrap();
    }

    fn treat_event(&mut self, event: Event) -> EventControl{
        match event {
            Event::Input(Key::Ctrl('a')) => {
                self.current_tab = 0;
                EventControl::OK
            },
            Event::Input(Key::Ctrl('z')) => {
                self.current_tab = 1;
                EventControl::OK
            },
            Event::Input(Key::Ctrl('e')) => {
                self.current_tab = 2;
                EventControl::OK
            },
            Event::Input(Key::Ctrl('c')) => EventControl::StopApp,
            Event::Tick => EventControl::OK,
            // otherwise delegate treating of keys
            _ => match self.current_tab {
                0 => self.mail_client.handle_event(event),
                1 => self.calendar.handle_event(event),
                2 => self.task_manager.handle_event(event),
                _ => unreachable!(),
            } 
        }
    } 

    pub fn run(mut self) -> Result<(), String> {
        let mut terminal = Self::prepare_env(); 
        let tick_rate = Duration::from_millis(100);
        let events = events(tick_rate);

        'running:  loop {
            // treat events and update models
            let control = match events.recv() {
                Ok(event) => self.treat_event(event),
                Err(e) => panic!("Lost event receiver : {}", e),
            };

            match control {
                EventControl::StopApp => break 'running,
                EventControl::OK => (),
            }

            // redraw terminal
            super::ui::draw(&mut terminal, &mut self);
        }

        Self::clear_env(terminal);

        Ok(())
    }   
}