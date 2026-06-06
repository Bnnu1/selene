use crate::app::App;

use ratatui::{
	Frame,
	layout::{Constraint, Direction, Layout, Position},
	style::{Color, Modifier, Style},
	widgets::{Block, Borders, List, ListItem, Paragraph},
};
use std::str::FromStr;

pub fn render(frame: &mut Frame, app: &mut App) {
	let bg: Color = Color::from_str(&app.config.background).unwrap();
	let fg: Color = Color::from_str(&app.config.foreground).unwrap();
	let bo: Color = Color::from_str(&app.config.border).unwrap();

	let selected = app
		.list_state
		.selected()
		.and_then(|i| app.items.get(i));
	
	let vertical = Layout::default()
		.direction(Direction::Vertical)
		.constraints([
			Constraint::Min(0),
			Constraint::Length(3),
		])
		.split(frame.area());

	let chunks = Layout::default()
		.direction(Direction::Horizontal)
		.constraints([
			Constraint::Percentage(50),
			Constraint::Percentage(50),
		])
		.split(vertical[0]);
	
	let items: Vec<ListItem> = app
		.items
		.iter()
		.map(|item| {
			let style = if app.marked.contains(item) {
				Style::default()
					.fg(bo)
					.add_modifier(
						Modifier::BOLD |
						Modifier::ITALIC
					)
				} else {
					Style::default().fg(fg)
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
			.fg(fg)
			.bg(bg)
		)
		.block(
			Block::default()
				.title(app.cwd.to_string_lossy().to_string())
				.borders(Borders::ALL)
				.border_style(
					Style::default().fg(bo)
				),
		)
		.highlight_style(
			Style::default()
				.fg(bg)
				.bg(fg)
		);

	let size_text = match selected {
		Some(path) => match std::fs::metadata(path) {
			Ok(meta) if meta.is_file() => human_size(meta.len()),
			Ok(_) => "<DIR>".to_string(),
			Err(_) => "?".to_string(),
		},
		None => "Empty directory".to_string(),
	};

	let text = if app.command_mode {
		format!(":{}", app.input)
	} else {
		size_text
	};

	let status = Paragraph::new(text.clone())
		.style(
			Style::default()
			.fg(fg)
			.bg(bg)
		)
		.block(
			Block::default()
			.borders(Borders::ALL)
			.border_style(
				Style::default().fg(bo)
			)
		);

	frame.render_stateful_widget(list, chunks[0], &mut app.list_state);
	frame.render_widget(
		Paragraph::new(app.preview.as_str())
			.style(
				Style::default()
				.fg(fg)
				.bg(bg)
			)
			.block(
				Block::default()
					.title("Preview")
					.borders(Borders::ALL)
					.border_style(
						Style::default().fg(bo)
					),
			),
		chunks[1],
	);
	frame.render_widget(status, vertical[1]);

	if app.command_mode {
		frame.set_cursor_position(Position::new(
			vertical[1].x + app.cursor_pos as u16 + 2,
			vertical[1].y + 1,
		));
	}
}

fn human_size(bytes: u64) -> String {
	const KB: f64 = 1024.0;
	const MB: f64 = KB * 1024.0;
	const GB: f64 = MB * 1024.0;

	let bytes = bytes as f64;

	if bytes >= GB {
		format!("{:.1}GB", bytes / GB)
	} else if bytes >= MB {
		format!("{:.1}MB", bytes / MB)
	} else if bytes >= KB {
		format!("{:.1}KB", bytes / KB)
	} else {
		format!("{:.0}B", bytes)
	}
}