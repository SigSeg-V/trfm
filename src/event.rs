use anyhow::Result;
use crossterm::event::{self, KeyEvent, MouseEvent, Event as XTermEvent};
use std::{sync::mpsc, thread, time::{Duration, Instant},};

/// Terminal/interaction events
pub enum Event {
    /// Terminal tick
    Tick,
    /// Key press or release event
    Key(KeyEvent),
    /// mouse click or scroll
    Mouse(MouseEvent),
    /// size and dimensions of the terminal changing
    Resize(u16, u16),
}

/// Terminal event handler
#[derive(Debug)]
pub struct EventHandler {
    /// Event sender channel
    #[allow(dead_code)]
    sender: mpsc::Sender<Event>,
    /// Event receiver channel
    receiver: mpsc::Receiver<Event>,
    /// Event handler thread
    #[allow(dead_code)]
    handler: thread::JoinHandle<()>,
}

impl EventHandler {
    /// Constructs a new instance of [`EventHandler`]
    pub fn new(tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        let (snd, rec) = mpsc::channel();
        let hnd = {
            let snd = snd.clone();
            thread::spawn(move || {
                let mut last_tick = Instant::now();
                loop {
                    let timeout = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or(tick_rate);

                    if event::poll(timeout).expect("unable to poll for event") {
                        match event::read().expect("unable to read event") {
                            XTermEvent::Key(e) => {
                                if e.kind == event::KeyEventKind::Press {
                                    snd.send(Event::Key(e))
                                } else {
                                    Ok(())
                                }
                            }
                            XTermEvent::Mouse(e) => snd.send(Event::Mouse(e)),
                            XTermEvent::Resize(w, h) => snd.send(Event::Resize(w, h)),
                            _ => unimplemented!()
                        }.expect("failed to send terminal event")
                    }

                    if last_tick.elapsed() >= tick_rate {
                        snd.send(Event::Tick).expect("failed to send tick event");
                    }
                }
            })
        };

        Self { sender: snd, receiver: rec, handler: hnd }
    }

    /// Receive the next event from [`EventHandler`]
    /// Blocks thread until the channel is closed
    pub fn next(&self) -> Result<Event> {
        Ok(self.receiver.recv()?)
    }
}