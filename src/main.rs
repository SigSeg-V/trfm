mod app;
mod event;
mod tui;
mod ui;
mod update;

use anyhow::Result;
use app::App;
use event::{EventHandler, Event,};
use ratatui::{backend::CrosstermBackend, Terminal,};
use tui::Tui;
use update::update;

fn main() -> Result<()> {
    // init the terminal ui
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    // app state
    let mut app = App::new();

    while !app.should_quit {
        tui.draw(&mut app)?;

        match tui.events.next()? {
            Event::Tick => {},
            Event::Key(e) => update(&mut app, e),
            Event::Mouse(_) => {},
            Event::Resize(_, _) => {},
        };
    }
    
    tui.exit()?;

    Ok(())
}