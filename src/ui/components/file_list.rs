use crate::search::MarkdownFile;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{List, ListItem, ListState},
    Frame,
};

pub struct FileList {
    files: Vec<MarkdownFile>,
    filtered_files: Vec<MarkdownFile>,
    state: ListState,
    items_per_page: usize,
    current_page: usize,
    search_query: String,
    is_searching: bool,
    search_input_mode: bool,
}

impl FileList {
    pub fn new(files: Vec<MarkdownFile>) -> Self {
        let mut state = ListState::default();
        if !files.is_empty() {
            state.select(Some(0));
        }

        let filtered_files = files.clone();

        Self {
            files,
            filtered_files,
            state,
            items_per_page: 10, // Will be updated based on screen size
            current_page: 0,
            search_query: String::new(),
            is_searching: false,
            search_input_mode: false,
        }
    }

    pub fn update_items_per_page(&mut self, height: usize) {
        // Reserve space for header (4), help (1), pagination (1)
        // Each file takes 3 lines (path + created_at + empty space)
        let available_height = height.saturating_sub(6);
        self.items_per_page = (available_height / 3).max(1);
    }

    pub fn total_pages(&self) -> usize {
        let file_count = if self.is_searching {
            self.filtered_files.len()
        } else {
            self.files.len()
        };

        if file_count == 0 {
            1
        } else {
            (file_count + self.items_per_page - 1) / self.items_per_page
        }
    }

    pub fn current_page(&self) -> usize {
        self.current_page + 1 // 1-indexed for display
    }

    pub fn next(&mut self) {
        let file_count = if self.is_searching {
            self.filtered_files.len()
        } else {
            self.files.len()
        };

        let current_selection = self.state.selected().unwrap_or(0);
        let start_index = self.current_page * self.items_per_page;
        let end_index = ((self.current_page + 1) * self.items_per_page).min(file_count);
        let relative_selection = current_selection - start_index;

        if relative_selection + 1 < (end_index - start_index) {
            // Move within current page
            self.state.select(Some(current_selection + 1));
        } else if self.current_page + 1 < self.total_pages() {
            // Move to next page and go to top
            self.current_page += 1;
            let new_start = self.current_page * self.items_per_page;
            self.state.select(Some(new_start));
        }
    }

    pub fn previous(&mut self) {
        let current_selection = self.state.selected().unwrap_or(0);
        let start_index = self.current_page * self.items_per_page;
        let relative_selection = current_selection - start_index;

        if relative_selection > 0 {
            // Move within current page
            self.state.select(Some(current_selection - 1));
        } else if self.current_page > 0 {
            // Move to previous page and go to bottom
            self.current_page -= 1;
            let _new_start = self.current_page * self.items_per_page;
            let new_end = ((self.current_page + 1) * self.items_per_page).min(self.files.len());
            self.state.select(Some(new_end - 1));
        }
    }

    pub fn get_current_file(&self) -> Option<&MarkdownFile> {
        // Don't return a file during search input
        if self.is_searching && self.search_query.is_empty() {
            return None;
        }

        if let Some(selected) = self.state.selected() {
            if self.is_searching {
                self.filtered_files.get(selected)
            } else {
                self.files.get(selected)
            }
        } else {
            None
        }
    }

    pub fn go_to_top(&mut self) {
        let file_count = if self.is_searching {
            self.filtered_files.len()
        } else {
            self.files.len()
        };

        if file_count > 0 {
            self.current_page = 0;
            self.state.select(Some(0));
        }
    }

    pub fn go_to_bottom(&mut self) {
        let file_count = if self.is_searching {
            self.filtered_files.len()
        } else {
            self.files.len()
        };

        if file_count > 0 {
            let last_page = self.total_pages().saturating_sub(1);
            self.current_page = last_page;
            self.state.select(Some(file_count - 1));
        }
    }

    pub fn next_page(&mut self) {
        if self.current_page + 1 < self.total_pages() {
            let file_count = if self.is_searching {
                self.filtered_files.len()
            } else {
                self.files.len()
            };

            let current_selection = self.state.selected().unwrap_or(0);
            let start_index = self.current_page * self.items_per_page;
            let relative_position = current_selection - start_index;

            self.current_page += 1;
            let new_start = self.current_page * self.items_per_page;
            let new_end = ((self.current_page + 1) * self.items_per_page).min(file_count);

            // Try to maintain the same relative position
            let new_selection = (new_start + relative_position).min(new_end - 1);
            self.state.select(Some(new_selection));
        }
    }

    pub fn previous_page(&mut self) {
        if self.current_page > 0 {
            let current_selection = self.state.selected().unwrap_or(0);
            let start_index = self.current_page * self.items_per_page;
            let relative_position = current_selection - start_index;

            self.current_page -= 1;
            let new_start = self.current_page * self.items_per_page;

            // Try to maintain the same relative position
            let new_selection = new_start + relative_position;
            self.state.select(Some(new_selection));
        }
    }

    fn get_visible_files(&self) -> &[MarkdownFile] {
        let files = if self.is_searching {
            &self.filtered_files
        } else {
            &self.files
        };

        let start = self.current_page * self.items_per_page;
        let end = ((self.current_page + 1) * self.items_per_page).min(files.len());
        &files[start..end]
    }

    pub fn start_search(&mut self) {
        self.is_searching = true;
        self.search_input_mode = true;
        self.search_query.clear();
        self.filtered_files = self.files.clone();
        self.current_page = 0;
        // Don't select anything during search input
        self.state.select(None);
    }

    pub fn end_search(&mut self) {
        self.is_searching = false;
        self.search_input_mode = false;
        self.search_query.clear();
        self.current_page = 0;
        if !self.files.is_empty() {
            self.state.select(Some(0));
        }
    }

    pub fn update_search(&mut self, query: &str) {
        self.search_query = query.to_string();

        if query.is_empty() {
            self.filtered_files = self.files.clone();
        } else {
            let matcher = SkimMatcherV2::default();
            self.filtered_files = self
                .files
                .iter()
                .filter_map(|file| {
                    let path_str = file.path.to_string_lossy();
                    matcher.fuzzy_match(&path_str, query).map(|_| file.clone())
                })
                .collect();
        }

        self.current_page = 0;
        // Only select when search is not in input mode (i.e., when query is not empty)
        if !query.is_empty() {
            if !self.filtered_files.is_empty() {
                self.state.select(Some(0));
            } else {
                self.state.select(None);
            }
        } else {
            // During search input, don't select anything
            self.state.select(None);
        }
    }

    pub fn get_search_query(&self) -> &str {
        &self.search_query
    }

    pub fn is_searching(&self) -> bool {
        self.is_searching
    }

    pub fn get_file_count(&self) -> usize {
        if self.is_searching {
            self.filtered_files.len()
        } else {
            self.files.len()
        }
    }

    pub fn get_original_count(&self) -> usize {
        self.files.len()
    }

    pub fn select_first(&mut self) {
        if self.is_searching && !self.filtered_files.is_empty() {
            self.state.select(Some(0));
        } else if !self.files.is_empty() {
            self.state.select(Some(0));
        }
    }

    pub fn exit_search_input_mode(&mut self) {
        self.search_input_mode = false;
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let visible_files = self.get_visible_files();

        // Create a local state for the current page
        let current_selection = self.state.selected().unwrap_or(0);
        let start = self.current_page * self.items_per_page;
        let files = if self.is_searching {
            &self.filtered_files
        } else {
            &self.files
        };
        let end = ((self.current_page + 1) * self.items_per_page).min(files.len());

        let relative_selection = if self.search_input_mode {
            // During search input, don't show any selection
            None
        } else if current_selection >= start && current_selection < end {
            Some(current_selection - start)
        } else {
            None
        };

        let mut local_state = ListState::default();
        // Only set selection if not in search input mode
        if !self.search_input_mode {
            local_state.select(relative_selection);
        }

        let items: Vec<ListItem> = visible_files
            .iter()
            .enumerate()
            .map(|(i, file)| {
                // During search input, nothing should be selected
                let is_selected = if self.search_input_mode {
                    false
                } else {
                    relative_selection == Some(i)
                };

                // Path styling - greyed out only during search input, normal after Enter is pressed
                let path_style = if self.search_input_mode {
                    Style::default().fg(Color::Rgb(100, 100, 100)) // Greyed out during search input
                } else if is_selected {
                    Style::default()
                        .fg(Color::Rgb(100, 200, 255))
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::Rgb(200, 200, 200)) // Normal color after search applied
                };

                // Creation date styling (dimmer)
                let date_style = if self.search_input_mode {
                    Style::default().fg(Color::Rgb(60, 60, 60)) // Much darker grey during search input
                } else if is_selected {
                    Style::default().fg(Color::Rgb(70, 140, 180)) // Darker than path when selected
                } else {
                    Style::default().fg(Color::Rgb(120, 120, 120)) // Normal grey after search applied
                };

                let path_display = file.path.to_string_lossy().to_string();
                let created_text = file
                    .created_at
                    .as_ref()
                    .map(|d| d.to_string())
                    .unwrap_or_else(|| "Unknown".to_string());

                let selector_line1 = if is_selected { "│ " } else { "  " };
                let selector_line2 = if is_selected { "│ " } else { "  " };

                // Create highlighted path spans during search input mode, or underlined spans after search applied
                let path_spans = if self.search_input_mode && !self.search_query.is_empty() {
                    self.create_highlighted_spans(&path_display, &self.search_query)
                } else if self.is_searching && !self.search_query.is_empty() {
                    // After Enter is pressed, show underlined matches
                    self.create_underlined_spans(&path_display, &self.search_query, path_style)
                } else {
                    vec![Span::styled(path_display, path_style)]
                };

                let content = vec![
                    Line::from({
                        let mut spans = vec![Span::styled(
                            selector_line1,
                            Style::default().fg(Color::Rgb(100, 200, 255)),
                        )];
                        spans.extend(path_spans);
                        spans
                    }),
                    Line::from(vec![
                        Span::styled(
                            selector_line2,
                            Style::default().fg(Color::Rgb(100, 200, 255)),
                        ),
                        Span::styled(created_text, date_style),
                    ]),
                    Line::from(vec![]), // Empty line for spacing between files
                ];

                ListItem::new(content)
            })
            .collect();

        let list = List::new(items);

        frame.render_stateful_widget(list, area, &mut local_state);
    }

    fn create_highlighted_spans(&self, text: &str, query: &str) -> Vec<Span> {
        let mut spans = Vec::new();

        if query.is_empty() {
            return vec![Span::styled(
                text.to_string(),
                Style::default().fg(Color::Rgb(100, 100, 100)),
            )];
        }

        let matcher = SkimMatcherV2::default();
        if let Some((_, indices)) = matcher.fuzzy_indices(text, query) {
            let mut last_end = 0;

            for &index in &indices {
                // Add text before match (greyed out)
                if index > last_end {
                    spans.push(Span::styled(
                        text[last_end..index].to_string(),
                        Style::default().fg(Color::Rgb(100, 100, 100)),
                    ));
                }

                // Add matched character (normal color)
                let char_end = text[index..].char_indices().nth(1).map(|(i, _)| index + i).unwrap_or(text.len());
                spans.push(Span::styled(
                    text[index..char_end].to_string(),
                    Style::default().fg(Color::Rgb(200, 200, 200)),
                ));

                last_end = char_end;
            }

            // Add remaining text after last match (greyed out)
            if last_end < text.len() {
                spans.push(Span::styled(
                    text[last_end..].to_string(),
                    Style::default().fg(Color::Rgb(100, 100, 100)),
                ));
            }
        } else {
            // No fuzzy match found, return the whole text greyed out
            spans.push(Span::styled(
                text.to_string(),
                Style::default().fg(Color::Rgb(100, 100, 100)),
            ));
        }

        spans
    }

    fn create_underlined_spans(&self, text: &str, query: &str, base_style: Style) -> Vec<Span> {
        let mut spans = Vec::new();

        if query.is_empty() {
            return vec![Span::styled(text.to_string(), base_style)];
        }

        let matcher = SkimMatcherV2::default();
        if let Some((_, indices)) = matcher.fuzzy_indices(text, query) {
            let mut last_end = 0;

            for &index in &indices {
                // Add text before match (normal style)
                if index > last_end {
                    spans.push(Span::styled(
                        text[last_end..index].to_string(),
                        base_style,
                    ));
                }

                // Add matched character (underlined)
                let char_end = text[index..].char_indices().nth(1).map(|(i, _)| index + i).unwrap_or(text.len());
                spans.push(Span::styled(
                    text[index..char_end].to_string(),
                    base_style.add_modifier(Modifier::UNDERLINED),
                ));

                last_end = char_end;
            }

            // Add remaining text after last match (normal style)
            if last_end < text.len() {
                spans.push(Span::styled(text[last_end..].to_string(), base_style));
            }
        } else {
            // No fuzzy match found, return the whole text with normal style
            spans.push(Span::styled(text.to_string(), base_style));
        }

        spans
    }
}
