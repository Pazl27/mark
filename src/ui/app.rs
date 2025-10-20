use crate::error::Result;
use crate::search::MarkdownFile;
use crate::ui::{events::EventHandler, file_browser::FileBrowser, Event};
use crossterm::event::KeyEvent;
use ratatui::Frame;

pub struct App {
    file_browser: FileBrowser,
    event_handler: EventHandler,
    running: bool,
}

impl App {
    pub fn new(
        directory: &str,
        config: &crate::config::MarkConfig,
        show_all: bool,
    ) -> Result<Self> {
        let file_browser = FileBrowser::new_with_background_search(
            directory,
            config.settings.ignored_dirs.clone(),
            config.settings.hidden_files,
            show_all,
        )?;
        let event_handler = EventHandler::new(50); // 50ms tick rate for responsive loading indicator

        Ok(Self {
            file_browser,
            event_handler,
            running: true,
        })
    }

    pub fn run(&mut self, terminal: &mut crate::ui::Tui) -> Result<Option<MarkdownFile>> {
        while self.running {
            terminal.draw(|frame| self.render(frame))?;

            if let Some(event) = self.event_handler.poll()? {
                match event {
                    Event::Key(key_event) => {
                        if let Some(selected_file) = self.handle_key_event(key_event)? {
                            return Ok(Some(selected_file.clone()));
                        }
                    }
                    Event::Resize(_, _) => {
                        // Terminal will handle resize automatically
                    }
                    Event::Mouse(_) => {
                        // Mouse events not handled yet
                    }
                }
            }

            if self.file_browser.should_quit() {
                self.running = false;
            }
        }

        Ok(None)
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<Option<&MarkdownFile>> {
        self.file_browser.handle_key_event(key_event)
    }

    fn render(&mut self, frame: &mut Frame) {
        self.file_browser.render(frame);
    }

    pub fn is_running(&self) -> bool {
        self.running
    }
}
