use crate::error::Result;
use crossterm::event::{self, Event as CrosstermEvent, KeyEvent, MouseEvent};
use std::time::Duration;

#[derive(Clone, Copy, Debug)]
pub enum Event {
    /// Key press.
    Key(KeyEvent),
    /// Mouse click/scroll.
    Mouse(MouseEvent),
    /// Terminal resize.
    Resize(u16, u16),
}

pub struct EventHandler {
    timeout: Duration,
}

impl EventHandler {
    /// Constructs a new instance of [`EventHandler`].
    pub fn new(tick_rate: u64) -> Self {
        Self {
            timeout: Duration::from_millis(tick_rate),
        }
    }

    /// Poll for the next event with timeout.
    pub fn poll(&self) -> Result<Option<Event>> {
        if event::poll(self.timeout)? {
            match event::read()? {
                CrosstermEvent::Key(e) => Ok(Some(Event::Key(e))),
                CrosstermEvent::Mouse(e) => Ok(Some(Event::Mouse(e))),
                CrosstermEvent::Resize(w, h) => Ok(Some(Event::Resize(w, h))),
                _ => Ok(None),
            }
        } else {
            Ok(None)
        }
    }

    /// Block until next event is available.
    pub fn next(&self) -> Result<Event> {
        loop {
            if let Some(event) = self.poll()? {
                return Ok(event);
            }
        }
    }
}
