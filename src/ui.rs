use crate::app::App;

use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, HighlightSpacing, List, ListItem},
    Frame,
};

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
		.map(|item| ListItem::new(item
			.file_name()
			.and_then(|name| name.to_str())
			.unwrap()
		))
		.collect();

	let list = List::new(items)
		.block(
			Block::default()
				.title("Files")
				.borders(Borders::ALL),
		)
		.highlight_symbol("> ")
		.highlight_spacing(HighlightSpacing::Always);

	frame.render_stateful_widget(list, chunks[0], &mut app.list_state);
}