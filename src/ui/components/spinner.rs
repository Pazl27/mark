use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};
use std::time::{Duration, Instant};

pub struct Spinner {
    frames: Vec<&'static str>,
    current_frame: usize,
    last_update: Instant,
    update_interval: Duration,
}

impl Spinner {
    pub fn new() -> Self {
        Self {
            frames: vec!["|", "/", "-", "\\"],
            current_frame: 0,
            last_update: Instant::now(),
            update_interval: Duration::from_millis(150),
        }
    }

    pub fn tick(&mut self) {
        if self.last_update.elapsed() >= self.update_interval {
            self.current_frame = (self.current_frame + 1) % self.frames.len();
            self.last_update = Instant::now();
        }
    }

    pub fn get_current_frame(&self) -> &str {
        self.frames[self.current_frame]
    }

    pub fn render_inline(&self) -> Span {
        Span::styled(
            self.get_current_frame(),
            Style::default().fg(Color::Rgb(100, 150, 255)),
        )
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let spinner_line = Line::from(self.render_inline());
        let spinner_widget = Paragraph::new(spinner_line);
        frame.render_widget(spinner_widget, area);
    }
}

impl Default for Spinner {
    fn default() -> Self {
        Self::new()
    }
}
