use std::ops::Deref;
use ratatui::{
    prelude::*,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
};

use tui_textarea::{Input, Key, TextArea};

use crate::app::{App, State};

pub fn render(app: &mut App, f: &mut Frame) {

    let path_list = app.paths.clone()
    .into_iter()
    .fold("".to_string(), |mut acc, s| {
        acc.push_str(&s);
        acc.push('\n');
        acc
    });

    let frame = f.size();

    let panels = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(3),
            Constraint::Length(3),
        ])
        .split(frame);

    f.render_widget(
        Paragraph::new(format!(
            "
            Press `Esc`, `C-c` or `q` to quit the app.

            cli content: {}

            List of paths:
            {}
            ",
            app.cli,
            path_list
        ))
        .block(
            Block::default()
            .title(format!("trfm - {}", app.current_path.to_str().unwrap_or_default()))
            .title_alignment(Alignment::Center)
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::Cyan))
        .alignment(Alignment::Center),
       panels[0]
    );
    match &app.state{
        State::Command(inp) | State::Search(inp) if inp.is_some() => {
            let mut ta = TextArea::default();
            ta.set_style(Style::default().fg(Color::Green));
            ta.set_block(Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Thick));
            ta.input_without_shortcuts(inp.clone().unwrap());

            let cli = ta.lines();

            f.render_widget(
                ta.widget(),

                // Paragraph::new(format!("{}",
                // match &app.state {
                //     State::Command(cmd) => format!(":{cmd}"),
                //     State::Search(srch) => format!("/{srch}"),
                //     _ => "".to_string()
                // }
                // )),
                panels[1]
            );
        },
        _ => {},
    }
}