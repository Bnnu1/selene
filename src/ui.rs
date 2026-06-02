use crate::app::App;

use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, HighlightSpacing, List, ListItem},
    Frame,
    style::{Color, Modifier, Style},
};

const BG: Color = Color::Rgb(5,14,57);
const FG: Color = Color::Rgb(244, 246, 240);
const BO: Color = Color::Rgb(82,47,129);

pub fn render(frame: &mut Frame, app: &mut App) {
	let chunks = Layout::default()
		.direction(Direction::Horizontal)
		.constraints([
			Constraint::Percentage(50),
			Constraint::Percentage(50),
		])
		.split(frame.area());
	
	let items: Vec<ListItem> = app
		.items
		.iter()
		.map(|item| {
			let style = if app.marked.contains(item) {
				Style::default()
					.fg(BO)
					.add_modifier(
						Modifier::BOLD |
						Modifier::ITALIC
					)
				} else {
					Style::default().fg(FG)
				};
		
			ListItem::new(item
				.file_name()
				.and_then(|name| name.to_str())
				.unwrap()
			)
			.style(style)
		})
		.collect();

	let list = List::new(items)
		.style(
			Style::default()
			.fg(FG)
			.bg(BG)
		)
		.block(
			Block::default()
				.title("Files")
				.borders(Borders::ALL)
				.border_style(
					Style::default().fg(BO)
				),
		)
		.highlight_style(
		Style::default()
			.fg(BG)
			.bg(FG)
		);

	frame.render_stateful_widget(list, chunks[0], &mut app.list_state);
}