use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use crate::app::App;

impl App {
    pub fn render(&mut self, f: &mut ratatui::Frame<'_>) {
        // Split the screen horizontally: 30% left, 70% right
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
            .split(f.area());

        // Left panel: the list
        let list_items: Vec<ListItem> = self
            .asdf_commands
            .iter()
            .map(|(name, _)| ListItem::new(name.to_string()))
            .collect();

        let list = List::new(list_items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("ASDF Commands"),
            )
            .highlight_style(
                Style::default()
                    .bg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("> ");

        f.render_stateful_widget(list, chunks[0], &mut self.list_state);
        let asdf_version = format!("ASDF Version: {}", self.asdf_version);

        // Right panel: description tied to selection
        let detail = Paragraph::new(vec![
            Line::from(self.selected_description()),
            Line::from(self.log_message.clone()),
        ])
            .block(Block::default().borders(Borders::ALL).title(&*asdf_version))
            .wrap(ratatui::widgets::Wrap { trim: true })
            .scroll((self.detail_scroll, 0));

        f.render_widget(detail, chunks[1]);
    }
}
