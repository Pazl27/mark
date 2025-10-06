use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

pub struct Help;

impl Help {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let help_spans = vec![
            Span::styled("j/k", Style::default().fg(Color::Rgb(120, 120, 120))),
            Span::styled(": Navigate  ", Style::default().fg(Color::Rgb(120, 120, 120))),
            
            Span::styled("q", Style::default().fg(Color::Rgb(120, 120, 120))),
            Span::styled(": Quit  ", Style::default().fg(Color::Rgb(120, 120, 120))),
            
            Span::styled("?", Style::default().fg(Color::Rgb(120, 120, 120))),
            Span::styled(": Help", Style::default().fg(Color::Rgb(120, 120, 120))),
        ];

        let help_line = Line::from(help_spans);

        let help = Paragraph::new(help_line)
            .alignment(Alignment::Left);

        frame.render_widget(help, area);
    }
}
