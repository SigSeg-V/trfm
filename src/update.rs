use crossterm::event::{KeyCode, KeyEvent, KeyModifiers,};

use crate::app::{App, State};

pub fn update(app: &mut App, key_event: KeyEvent) {

    match &app.state {
        State::Normal => {
            match key_event.code {
                KeyCode::Char(':') => app.state = State::Command("".to_string()),
                KeyCode::Char('/') => app.state = State::Search("".to_string()),
                _ => {}
            }
        },
        State::Command(cmd) => cmd_mode_keys(app, key_event),
        State::Search(_) => {},
    }

    // match key_event.code {
    //     // shutting down the app
    //     KeyCode::Esc | KeyCode::Char('q') => app.quit(),
    //     KeyCode::Char('c') | KeyCode::Char('C') => {
    //         if key_event.modifiers == KeyModifiers::CONTROL {
    //             app.quit()
    //         }
    //
    //         // chdir command
    //         app.chdir();
    //         app.list_paths();
    //     }
    //
    //     // counter mutation
    //     KeyCode::Right | KeyCode::Up | KeyCode::Char('j') => app.inc_counter(),
    //     KeyCode::Left| KeyCode::Down | KeyCode::Char('k') => app.dec_counter(),
    //
    //     _=> {}
    // }
}

fn cmd_mode_keys(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc | KeyCode::Enter => app.state = State::Normal,
        KeyCode::Char(c) => {
            if let State::Command(cmd) = &mut app.state {
                cmd.push(c);
                app.state = State::Command(cmd.to_string());
            }
        },
        KeyCode::Backspace => {
            if let State::Command(cmd) = &mut app.state {
                cmd.pop();
                app.state = State::Command(cmd.to_string());
            }
        }
        _ => {}
    }
}