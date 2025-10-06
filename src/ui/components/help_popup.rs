use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

pub struct HelpPopup {
    visible: bool,
}

impl HelpPopup {
    pub fn new() -> Self {
        Self { visible: false }
    }

    pub fn show(&mut self) {
        self.visible = true;
    }

    pub fn hide(&mut self) {
        self.visible = false;
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }

    pub fn toggle(&mut self) {
        self.visible = !self.visible;
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        if !self.visible {
            return;
        }

        // Calculate popup size (centered, 60% of screen width, auto height)
        let popup_width = (area.width * 60) / 100;
        let popup_height = 24;
        let x = (area.width - popup_width) / 2;
        let y = (area.height - popup_height) / 2;

        let popup_area = Rect {
            x,
            y,
            width: popup_width,
            height: popup_height,
        };

        // Clear the area behind the popup
        frame.render_widget(Clear, popup_area);

        // Create help content
        let help_lines = vec![
            Line::from(vec![Span::styled(
                "Navigation:",
                Style::default()
                    .fg(Color::Rgb(255, 200, 100))
                    .add_modifier(Modifier::BOLD),
            )]),
            Line::from(vec![
                Span::styled("  j / ↓", Style::default().fg(Color::Rgb(100, 200, 255))),
                Span::styled(
                    "        Move down",
                    Style::default().fg(Color::Rgb(200, 200, 200)),
                ),
            ]),
            Line::from(vec![
                Span::styled("  k / ↑", Style::default().fg(Color::Rgb(100, 200, 255))),
                Span::styled(
                    "        Move up",
                    Style::default().fg(Color::Rgb(200, 200, 200)),
                ),
            ]),
            Line::from(vec![
                Span::styled("  h / ←", Style::default().fg(Color::Rgb(100, 200, 255))),
                Span::styled(
                    "        Previous page",
                    Style::default().fg(Color::Rgb(200, 200, 200)),
                ),
            ]),
            Line::from(vec![
                Span::styled("  l / →", Style::default().fg(Color::Rgb(100, 200, 255))),
                Span::styled(
                    "        Next page",
                    Style::default().fg(Color::Rgb(200, 200, 200)),
                ),
            ]),
            Line::from(vec![
                Span::styled("  gg", Style::default().fg(Color::Rgb(100, 200, 255))),
                Span::styled(
                    "          Go to top",
                    Style::default().fg(Color::Rgb(200, 200, 200)),
                ),
            ]),
            Line::from(vec![
                Span::styled("  G", Style::default().fg(Color::Rgb(100, 200, 255))),
                Span::styled(
                    "           Go to bottom",
                    Style::default().fg(Color::Rgb(200, 200, 200)),
                ),
            ]),
            Line::from(vec![]),
            Line::from(vec![Span::styled(
                "Search:",
                Style::default()
                    .fg(Color::Rgb(255, 200, 100))
                    .add_modifier(Modifier::BOLD),
            )]),
            Line::from(vec![
                Span::styled("  /", Style::default().fg(Color::Rgb(255, 200, 100))),
                Span::styled(
                    "           Start search/filter",
                    Style::default().fg(Color::Rgb(200, 200, 200)),
                ),
            ]),
            Line::from(vec![
                Span::styled("  Enter", Style::default().fg(Color::Rgb(255, 200, 100))),
                Span::styled(
                    "       Apply search filter",
                    Style::default().fg(Color::Rgb(200, 200, 200)),
                ),
            ]),
            Line::from(vec![
                Span::styled("  Esc", Style::default().fg(Color::Rgb(255, 200, 100))),
                Span::styled(
                    "         Exit search/show all",
                    Style::default().fg(Color::Rgb(200, 200, 200)),
                ),
            ]),
            Line::from(vec![]),
            Line::from(vec![Span::styled(
                "Actions:",
                Style::default()
                    .fg(Color::Rgb(255, 200, 100))
                    .add_modifier(Modifier::BOLD),
            )]),
            Line::from(vec![
                Span::styled("  Enter", Style::default().fg(Color::Rgb(100, 255, 100))),
                Span::styled(
                    "       Open selected file",
                    Style::default().fg(Color::Rgb(200, 200, 200)),
                ),
            ]),
            Line::from(vec![
                Span::styled("  q", Style::default().fg(Color::Rgb(255, 100, 100))),
                Span::styled(
                    "           Quit application",
                    Style::default().fg(Color::Rgb(200, 200, 200)),
                ),
            ]),
            Line::from(vec![]),
            Line::from(vec![Span::styled(
                "Help:",
                Style::default()
                    .fg(Color::Rgb(255, 200, 100))
                    .add_modifier(Modifier::BOLD),
            )]),
            Line::from(vec![
                Span::styled("  ?", Style::default().fg(Color::Rgb(255, 200, 100))),
                Span::styled(
                    "           Show/hide this help",
                    Style::default().fg(Color::Rgb(200, 200, 200)),
                ),
            ]),
            Line::from(vec![]),
            Line::from(vec![
                Span::styled("Press ", Style::default().fg(Color::Rgb(150, 150, 150))),
                Span::styled(
                    "?",
                    Style::default()
                        .fg(Color::Rgb(255, 200, 100))
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(" or ", Style::default().fg(Color::Rgb(150, 150, 150))),
                Span::styled(
                    "Esc",
                    Style::default()
                        .fg(Color::Rgb(255, 200, 100))
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    " to close this help",
                    Style::default().fg(Color::Rgb(150, 150, 150)),
                ),
            ]),
        ];

        let help_paragraph = Paragraph::new(help_lines)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Help ")
                    .title_alignment(Alignment::Center)
                    .border_style(Style::default().fg(Color::Rgb(100, 200, 255)))
                    .style(Style::default().bg(Color::Rgb(20, 20, 30))),
            )
            .alignment(Alignment::Left)
            .wrap(ratatui::widgets::Wrap { trim: true });

        frame.render_widget(help_paragraph, popup_area);
    }
}
