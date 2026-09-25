use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
        mpsc,
    },
    thread,
    time::{Duration, Instant},
};

use ratatui::crossterm::event::{
    self, Event as CrosstermEvent, KeyEvent, KeyEventKind, MouseEvent,
};

use crate::error::{Error, Result};

/// Terminal events.
#[derive(Debug)]
pub enum Event {
    /// Terminal tick.
    Tick,
    /// Key press.
    Key(KeyEvent),
    /// Mouse click/scroll.
    Mouse(MouseEvent),
    /// Terminal resize.
    Resize(u16, u16),
}

/// Terminal event handler.
#[allow(dead_code)]
#[derive(Debug)]
pub struct EventHandler {
    /// Event sender channel.
    pub sender: mpsc::Sender<Event>,
    /// Event receiver channel.
    receiver: mpsc::Receiver<Event>,
    /// Event handler thread.
    handler: thread::JoinHandle<Result<()>>,
    /// Is it listening for events?
    running: Arc<AtomicBool>,
}

impl EventHandler {
    /// Constructs a new instance of [`EventHandler`].
    pub(crate) fn new(tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        let (sender, receiver) = mpsc::channel();
        let running = Arc::new(AtomicBool::new(true));
        let handler = {
            let sender = sender.clone();
            let running = running.clone();
            thread::spawn(move || -> Result<()> {
                let mut last_tick = Instant::now();
                while running.load(Ordering::Relaxed) {
                    let timeout = tick_rate
                        .checked_sub(last_tick.elapsed())
                        .unwrap_or(tick_rate);
                    if event::poll(timeout)? {
                        let event = match event::read()? {
                            CrosstermEvent::Key(e) if e.kind == KeyEventKind::Press => {
                                Some(Event::Key(e))
                            }
                            CrosstermEvent::Mouse(e) => Some(Event::Mouse(e)),
                            CrosstermEvent::Resize(w, h) => Some(Event::Resize(w, h)),
                            // Non-press keys and focus/paste events are ignored.
                            _ => None,
                        };
                        if let Some(event) = event
                            && sender.send(event).is_err()
                        {
                            return Err(Error::Event("failed to send terminal event".to_string()));
                        }
                    }

                    if last_tick.elapsed() >= tick_rate {
                        if sender.send(Event::Tick).is_err() {
                            return Err(Error::Event("failed to send tick event".to_string()));
                        }
                        last_tick = Instant::now();
                    }
                }
                Ok(())
            })
        };
        Self {
            sender,
            receiver,
            handler,
            running,
        }
    }

    /// Receive the next event from the handler thread.
    ///
    /// This function will always block the current thread if
    /// there is no data available and it's possible for more data to be sent.
    pub(crate) fn next(&self) -> Result<Event> {
        Ok(self.receiver.recv()?)
    }

    /// Stops the event listener.
    pub(crate) fn stop(&self) {
        self.running.store(false, Ordering::Relaxed);
    }
}
