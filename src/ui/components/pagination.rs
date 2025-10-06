use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

pub struct Pagination {
    current_page: usize,
    total_pages: usize,
}

impl Pagination {
    pub fn new(current_page: usize, total_pages: usize) -> Self {
        Self {
            current_page,
            total_pages,
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let page_text = if self.total_pages > 0 {
            format!("Page {} of {}", self.current_page, self.total_pages)
        } else {
            "Page 1 of 1".to_string()
        };

        let pagination_span =
            Span::styled(page_text, Style::default().fg(Color::Rgb(150, 150, 200)));

        let pagination = Paragraph::new(Line::from(pagination_span)).alignment(Alignment::Center);

        frame.render_widget(pagination, area);
    }
}
