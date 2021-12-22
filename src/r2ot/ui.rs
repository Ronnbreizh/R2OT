use tui::{layout::{Layout, Constraint}, Frame, backend::Backend, widgets::{Tabs, Borders, Block}, text::{Spans, Span}, style::{Style, Color}};

use super::{application::R2otTerminal, R2ot, subapp::SubApp};

/// Function to draw the application
pub fn draw(terminal: &mut R2otTerminal, app : &mut R2ot) {
    terminal.draw(|f | {
        inner_draw(f, app)
    }).unwrap();
}

fn inner_draw<B:Backend>(f: &mut Frame<B>, app : &mut R2ot) {
    // tab chunk
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.size());

    let titles = app.tabs_titles
        .iter()
        .map(|t| Spans::from(Span::styled(*t, Style::default().fg(Color::Green))))
        .collect();

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title(app.app_name))
        .highlight_style(Style::default().fg(Color::Yellow))
        .select(app.current_tab);

    f.render_widget(tabs, chunks[0]);

    // tab drawing chunk
    match app.current_tab {
        0 => app.mail_client.draw(f, chunks[1]),
        1 => app.calendar.draw(f, chunks[1]),
        2 => app.task_manager.draw(f, chunks[1]),
        _ => unreachable!()
    }
}