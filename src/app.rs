use std::{fs, path::PathBuf, io};
use std::rc::Rc;
use crate::event::{Event, EventHandler};
use tui_textarea::Input;

#[derive(Debug)]
pub struct App {
    /// handles if the app should exit
    pub should_quit: bool,
    /// counter
    pub counter: i32,
    pub paths: Vec<Box<str>>,
    pub current_path: PathBuf,
    pub state: State,
    pub cli: String,
}

impl App {
    /// Creates a new instance of [`App`]
    pub fn new(cli_events: Rc<EventHandler>) -> Self {
       Self::default()
    }

    /// Handles the tick event from the terminal
    pub fn tick(&self) {

    }

    pub fn chdir(&mut self) {
        std::env::set_current_dir("/Users/charlie/Documents").unwrap();
        self.current_path = "/Users/charlie/Documents".into();
    }

    pub fn update_current_path(&mut self) {
        self.current_path = std::env::current_dir().unwrap();
    }

    pub fn list_paths(&mut self) {
        self.paths = get_list_at_dir(self.current_path.clone());
        self.paths.sort()
    }

    /// Flag to quit the app
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    /// Increments the counter on screen
    pub fn inc_counter(&mut self) {
        if let Some(result) = self.counter.checked_add(1) {
            self.counter = result
        }
    }

    /// Decrements the counter on screen
    pub fn dec_counter(&mut self) {
        if let Some(result) = self.counter.checked_sub(1) {
            self.counter = result
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self { 
            should_quit: Default::default(), 
            counter: Default::default(), 
            paths: get_list_at_dir(std::env::current_dir().unwrap()), 
            current_path: std::env::current_dir().unwrap(),
            state: State::Normal,
            cli: "".to_string(),
        }
    }
}

#[derive(Debug)]
pub enum State {
    Normal,
    Command(Option<Input>),
    Search(Option<Input>),
}

//#[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_inc_counter() {
//         let mut app = App::new();
//         app.inc_counter();
//         assert_eq!(0, app.counter)
//     }
//     fn test_dec_counter() {
//         let mut app = App::new();
//         app.dec_counter();
//         assert_eq!(-1, app.counter)
//     }
// }

fn get_list_at_dir(dir: PathBuf) -> Vec<Box<str>> {
    fs::read_dir(dir)
    .unwrap()
    .map(|e| e.map(|res| Box::from(res.path().to_str().unwrap().split("/\\").last().unwrap_or_default())))
    .collect::<Result<Vec<Box<str>>, io::Error>>()
    .unwrap()
}