use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
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

        // Right panel: description tied to selection
        let detail = Paragraph::new(vec![
            Line::from(self.selected_description()),
            Line::from(self.log_message.clone()),
        ])
        .block(Block::default().borders(Borders::ALL).title("Details"))
        .wrap(ratatui::widgets::Wrap { trim: true })
        .scroll((self.detail_scroll, 0));

        f.render_widget(detail, chunks[1]);

        if self.pop_up {
            self.render_popup(f);
        }
    }

    pub fn render_popup(&mut self, f: &mut ratatui::Frame<'_>) {
        let area = centered_rect(60, 50, f.area());

        // Clears anything underneath
        f.render_widget(Clear, area);
        let lines: Vec<Line<'_>> = self
            .user_input
            .iter()
            .enumerate()
            .map(|(_, input)| {
                let mut field = format!("{}", input.name);
                if input.multiple {
                    field.push_str(" (multiple space separated)");
                }
                if input.required {
                    field.push_str("*");
                }
                if input.value.is_some() {
                    field.push_str(&format!(": {}", input.value.as_ref().unwrap()));
                }
                Line::from(vec![Span::styled(
                    field,
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )])
            })
            .collect();

        let list = List::new(lines)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("User Input (Esc to cancel and Enter to submit)"),
            )
            .highlight_style(
                Style::default()
                    .bg(Color::Blue)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("> ");

        f.render_stateful_widget(list, area, &mut self.user_input_state);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(vertical[1]);

    horizontal[1]
}
