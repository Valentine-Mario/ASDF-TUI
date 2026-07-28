use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph, Widget},
};

use crate::app::App;

impl App {
    pub fn render(&self, f: &mut ratatui::Frame<'_>) {
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

        f.render_stateful_widget(list, chunks[0], &mut self.list_state.clone());

        // Right panel: description tied to selection
        let detail = Paragraph::new(self.selected_description())
            .block(Block::default().borders(Borders::ALL).title("Details"))
            .wrap(ratatui::widgets::Wrap { trim: true });

        f.render_widget(detail, chunks[1]);
    }
}

fn ui(f: &mut ratatui::Frame, app: &mut App) {}
