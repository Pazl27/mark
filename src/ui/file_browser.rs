use crate::error::Result;
use crate::search::{background::BackgroundSearcher, MarkdownFile};
use crate::ui::components::{FileList, Header, Help, HelpPopup, Pagination, SearchBar};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};

pub struct FileBrowser {
    file_list: FileList,
    header: Header,
    help: Help,
    help_popup: HelpPopup,
    search_bar: SearchBar,
    should_quit: bool,
    last_key_was_g: bool,
    background_searcher: Option<BackgroundSearcher>,
}

impl FileBrowser {
    pub fn new(files: Vec<MarkdownFile>) -> Self {
        let file_count = files.len();
        let file_list = FileList::new(files);
        let header = Header::new(file_count);
        let help = Help::new();
        let help_popup = HelpPopup::new();
        let search_bar = SearchBar::new();

        Self {
            file_list,
            header,
            help,
            help_popup,
            search_bar,
            should_quit: false,
            last_key_was_g: false,
            background_searcher: None,
        }
    }

    pub fn new_with_background_search(
        directory: &str,
        ignored_dirs: Vec<String>,
        show_hidden: bool,
        show_all: bool,
    ) -> Result<Self> {
        let file_list = FileList::new(Vec::new());
        let mut header = Header::new(0);
        header.set_loading(true);
        let help = Help::new();
        let help_popup = HelpPopup::new();
        let search_bar = SearchBar::new();

        let background_searcher = BackgroundSearcher::new(directory, ignored_dirs, show_hidden, show_all)?;

        Ok(Self {
            file_list,
            header,
            help,
            help_popup,
            search_bar,
            should_quit: false,
            last_key_was_g: false,
            background_searcher: Some(background_searcher),
        })
    }

    pub fn update_background_search(&mut self) {
        if let Some(ref mut searcher) = self.background_searcher {
            let messages = searcher.try_recv();
            let mut files_added = 0;
            
            for message in messages {
                match message {
                    crate::search::background::SearchMessage::FileFound(file) => {
                        self.file_list.add_file(file);
                        files_added += 1;
                    }
                    crate::search::background::SearchMessage::Finished => {
                        self.header.set_loading(false);
                        break;
                    }
                    crate::search::background::SearchMessage::Error(_) => {
                        self.header.set_loading(false);
                        break;
                    }
                }
            }
            
            if files_added > 0 {
                let current_count = self.file_list.get_original_count();
                self.header.update_file_count(current_count);
            }
            
            // Update spinner animation
            if !searcher.is_complete {
                self.header.tick();
            }
        }
    }

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    pub fn get_selected_file(&self) -> Option<&MarkdownFile> {
        self.file_list.get_current_file()
    }

    pub fn handle_key_event(&mut self, key: KeyEvent) -> Result<Option<&MarkdownFile>> {
        // If help popup is visible, handle help-specific keys
        if self.help_popup.is_visible() {
            match key.code {
                KeyCode::Char('?') | KeyCode::Esc => {
                    self.help_popup.hide();
                    Ok(None)
                }
                _ => Ok(None),
            }
        } else if self.search_bar.is_active() {
            // Search mode
            match key.code {
                KeyCode::Char(c) => {
                    self.search_bar.add_char(c);
                    self.file_list.update_search(self.search_bar.get_query());
                    self.update_header();
                    self.last_key_was_g = false;
                    Ok(None)
                }
                KeyCode::Backspace => {
                    self.search_bar.remove_char();
                    self.file_list.update_search(self.search_bar.get_query());
                    self.update_header();
                    self.last_key_was_g = false;
                    Ok(None)
                }
                KeyCode::Left => {
                    self.search_bar.move_cursor_left();
                    Ok(None)
                }
                KeyCode::Right => {
                    self.search_bar.move_cursor_right();
                    Ok(None)
                }
                KeyCode::Enter => {
                    // Apply search and exit search mode
                    if !self.search_bar.get_query().is_empty() {
                        self.search_bar.deactivate();
                        self.file_list.exit_search_input_mode();
                        // Select first filtered file when applying search
                        if self.file_list.get_file_count() > 0 {
                            self.file_list.select_first();
                        }
                        self.update_header();
                    }
                    self.last_key_was_g = false;
                    Ok(None)
                }
                KeyCode::Esc => {
                    // Cancel search and show all files
                    self.search_bar.deactivate();
                    self.file_list.end_search();
                    self.update_header();
                    self.last_key_was_g = false;
                    Ok(None)
                }
                _ => {
                    self.last_key_was_g = false;
                    Ok(None)
                }
            }
        } else {
            // Normal navigation
            match key.code {
                KeyCode::Char('q') => {
                    self.should_quit = true;
                    Ok(None)
                }
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    self.should_quit = true;
                    Ok(None)
                }
                KeyCode::Esc => {
                    // Only handle Esc if there's an active search filter
                    if self.file_list.is_searching()
                        && !self.file_list.get_search_query().is_empty()
                    {
                        self.file_list.end_search();
                        self.update_header();
                    }
                    self.last_key_was_g = false;
                    Ok(None)
                }
                KeyCode::Char('?') => {
                    self.help_popup.show();
                    self.last_key_was_g = false;
                    Ok(None)
                }
                KeyCode::Char('/') => {
                    self.search_bar.activate();
                    self.file_list.start_search();
                    self.update_header();
                    self.last_key_was_g = false;
                    Ok(None)
                }
                KeyCode::Char('g') => {
                    if self.last_key_was_g {
                        // gg - go to top
                        self.file_list.go_to_top();
                        self.last_key_was_g = false;
                    } else {
                        self.last_key_was_g = true;
                    }
                    Ok(None)
                }
                KeyCode::Char('G') => {
                    // G - go to bottom
                    self.file_list.go_to_bottom();
                    self.last_key_was_g = false;
                    Ok(None)
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    self.file_list.next();
                    self.last_key_was_g = false;
                    Ok(None)
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    self.file_list.previous();
                    self.last_key_was_g = false;
                    Ok(None)
                }
                KeyCode::Left | KeyCode::Char('h') => {
                    self.file_list.previous_page();
                    self.last_key_was_g = false;
                    Ok(None)
                }
                KeyCode::Right | KeyCode::Char('l') => {
                    self.file_list.next_page();
                    self.last_key_was_g = false;
                    Ok(None)
                }
                KeyCode::Enter => {
                    // Return the selected file to open it
                    self.last_key_was_g = false;
                    Ok(self.file_list.get_current_file())
                }
                _ => {
                    self.last_key_was_g = false;
                    Ok(None)
                }
            }
        }
    }

    fn update_header(&mut self) {
        let is_searching = self.file_list.is_searching();
        let query = self.file_list.get_search_query();
        let filtered_count = self.file_list.get_file_count();
        let original_count = self.file_list.get_original_count();
        self.header
            .set_search_mode(is_searching, query, filtered_count, original_count);
    }

    pub fn render(&mut self, frame: &mut Frame) {
        let size = frame.area();

        // Update background search first
        self.update_background_search();

        // Update items per page based on screen size
        self.file_list.update_items_per_page(size.height as usize);

        // Create layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(4), // Header
                Constraint::Min(1),    // File list (flexible)
                Constraint::Length(1), // Pagination
                Constraint::Length(1), // Help
            ])
            .split(size);

        // Render components
        if self.search_bar.is_active() {
            self.search_bar.render(frame, chunks[0]);
        } else {
            self.header.render(frame, chunks[0]);
        }
        self.file_list.render(frame, chunks[1]);

        // Render pagination
        let pagination =
            Pagination::new(self.file_list.current_page(), self.file_list.total_pages());
        pagination.render(frame, chunks[2]);

        self.help.render(frame, chunks[3]);

        // Render help popup on top if visible
        self.help_popup.render(frame, size);
    }
}
