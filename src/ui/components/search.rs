use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};
use std::time::{Duration, Instant};

pub struct SearchBar {
    active: bool,
    query: String,
    cursor_position: usize,
    cursor_blink_start: Option<Instant>,
}

impl SearchBar {
    pub fn new() -> Self {
        Self {
            active: false,
            query: String::new(),
            cursor_position: 0,
            cursor_blink_start: None,
        }
    }

    pub fn activate(&mut self) {
        self.active = true;
        self.query.clear();
        self.cursor_position = 0;
        self.cursor_blink_start = Some(Instant::now());
    }

    pub fn deactivate(&mut self) {
        self.active = false;
        self.query.clear();
        self.cursor_position = 0;
        self.cursor_blink_start = None;
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn get_query(&self) -> &str {
        &self.query
    }

    pub fn add_char(&mut self, c: char) {
        if self.active {
            self.query.insert(self.cursor_position, c);
            self.cursor_position += 1;
            self.cursor_blink_start = Some(Instant::now());
        }
    }

    pub fn remove_char(&mut self) {
        if self.active && self.cursor_position > 0 {
            self.cursor_position -= 1;
            self.query.remove(self.cursor_position);
            self.cursor_blink_start = Some(Instant::now());
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
            self.cursor_blink_start = Some(Instant::now());
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor_position < self.query.len() {
            self.cursor_position += 1;
            self.cursor_blink_start = Some(Instant::now());
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        if !self.active {
            return;
        }

        // Calculate cursor visibility (blink every 500ms)
        let cursor_visible = if let Some(start) = self.cursor_blink_start {
            let elapsed = start.elapsed();
            (elapsed.as_millis() / 500) % 2 == 0
        } else {
            true
        };

        let mut spans = vec![Span::styled(
            "Filter: ",
            Style::default().fg(Color::Rgb(100, 200, 255)),
        )];

        // Split query at cursor position
        let before_cursor = &self.query[..self.cursor_position];
        let at_cursor = self.query.chars().nth(self.cursor_position).unwrap_or(' ');
        let after_cursor = if self.cursor_position < self.query.len() {
            &self.query[self.cursor_position + 1..]
        } else {
            ""
        };

        // Add text before cursor
        if !before_cursor.is_empty() {
            spans.push(Span::styled(
                before_cursor.to_string(),
                Style::default().fg(Color::Rgb(100, 200, 255)),
            ));
        }

        // Add cursor (blinking block)
        if cursor_visible {
            let cursor_char = if self.cursor_position >= self.query.len() {
                " "
            } else {
                &at_cursor.to_string()
            };
            spans.push(Span::styled(
                cursor_char.to_string(),
                Style::default()
                    .bg(Color::Rgb(100, 200, 255))
                    .fg(Color::Rgb(20, 20, 30))
                    .add_modifier(Modifier::BOLD),
            ));
        } else if self.cursor_position < self.query.len() {
            // Show character without cursor highlighting when not visible
            spans.push(Span::styled(
                at_cursor.to_string(),
                Style::default().fg(Color::Rgb(100, 200, 255)),
            ));
        }

        // Add text after cursor
        if !after_cursor.is_empty() {
            spans.push(Span::styled(
                after_cursor.to_string(),
                Style::default().fg(Color::Rgb(100, 200, 255)),
            ));
        }

        let search_line = Line::from(spans);
        let search_paragraph = Paragraph::new(search_line).alignment(Alignment::Left);

        frame.render_widget(search_paragraph, area);
    }
}
