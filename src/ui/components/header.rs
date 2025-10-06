use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

pub struct Header {
    title: String,
    file_count: usize,
    original_count: usize,
    search_query: String,
    is_searching: bool,
}

impl Header {
    pub fn new(file_count: usize) -> Self {
        Self {
            title: "Mark".to_string(),
            file_count,
            original_count: file_count,
            search_query: String::new(),
            is_searching: false,
        }
    }

    pub fn set_search_mode(&mut self, is_searching: bool, query: &str, filtered_count: usize, original_count: usize) {
        self.is_searching = is_searching;
        self.search_query = query.to_string();
        self.original_count = original_count;
        if is_searching && !query.is_empty() {
            self.file_count = filtered_count;
        } else {
            self.file_count = original_count;
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),  // Title
                Constraint::Length(1),  // Empty line
                Constraint::Length(1),  // Elements count
                Constraint::Length(1),  // Empty line
            ])
            .split(area);

        if self.is_searching && self.search_query.is_empty() {
            // Don't show title during search input
            let count_text = format!("  {} elements", self.file_count);
            let count_line = Line::from(Span::styled(
                count_text,
                Style::default().fg(Color::Rgb(100, 100, 100)),
            ));
            let count = Paragraph::new(count_line).alignment(Alignment::Left);
            frame.render_widget(count, chunks[2]);
        } else {
            // Create styled title with gradient-like effect
            let title_spans = vec![
                Span::styled("M", Style::default().fg(Color::Rgb(255, 100, 150)).add_modifier(Modifier::BOLD)),
                Span::styled("a", Style::default().fg(Color::Rgb(255, 120, 170)).add_modifier(Modifier::BOLD)),
                Span::styled("r", Style::default().fg(Color::Rgb(255, 140, 190)).add_modifier(Modifier::BOLD)),
                Span::styled("k", Style::default().fg(Color::Rgb(255, 160, 210)).add_modifier(Modifier::BOLD)),
            ];

            let title_line = Line::from(title_spans);
            let title = Paragraph::new(title_line).alignment(Alignment::Center);
            
            // File count info with search query if applicable
            let count_line = if self.is_searching && !self.search_query.is_empty() {
                Line::from(vec![
                    Span::styled(
                        format!("  {} elements | ", self.original_count),
                        Style::default().fg(Color::Rgb(100, 100, 100)), // Greyed out original count
                    ),
                    Span::styled(
                        format!("{} \"{}\"", self.file_count, self.search_query),
                        Style::default().fg(Color::Rgb(150, 150, 150)), // Normal color for filtered count
                    ),
                ])
            } else {
                Line::from(Span::styled(
                    format!("  {} elements", self.file_count),
                    Style::default().fg(Color::Rgb(150, 150, 150)),
                ))
            };
            let count = Paragraph::new(count_line).alignment(Alignment::Left);

            frame.render_widget(title, chunks[0]);
            frame.render_widget(count, chunks[2]);
        }
    }
}
