use std::{io, panic};
use std::rc::Rc;
use anyhow::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture,},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen,},
};

pub type XTermTerminal = ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stderr>>;

use crate::{app::App, event::EventHandler, ui};

pub struct Tui {
    /// Interface to the terminal
    terminal: XTermTerminal,
    /// Terminal event handler
    pub events: Rc<EventHandler>,
}

impl Tui {
    /// Constructs a new instance of [`Tui`]
    pub fn new(terminal: XTermTerminal, events: Rc<EventHandler>) -> Self {
        Self{terminal, events}
    }

    /// Inits the terminal interface
    pub fn enter(&mut self) -> Result<()> {
        terminal::enable_raw_mode()?;
        crossterm::execute!(
            io::stderr(), 
            EnterAlternateScreen, 
            EnableMouseCapture
        )?;

        // panic hook to revert the terminal to normal
        // if something goes wrong in the program
        let panic_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic| {
            Self::reset().expect("failed to reset the terminal");
        }));

        self.terminal.hide_cursor()?;
        self.terminal.clear()?;
        Ok(())
    }

    fn reset() -> Result<()> {
        terminal::disable_raw_mode()?;
        crossterm::execute!(
            io::stderr(),
            LeaveAlternateScreen,
            DisableMouseCapture,
        )?;
        Ok(())
    }

    pub fn exit(&mut self) -> Result<()> {
        Self::reset()?;
        self.terminal.show_cursor()?;
        Ok(())
    }

    pub fn draw(&mut self, app: &mut App) -> Result<()> {
        self.terminal.draw(|f| ui::render(app, f))?;
        Ok(())
    }
}