use crate::app::App;

use chrono::{DateTime, Local};
use ratatui::{
	Frame,
	layout::{Constraint, Direction, Layout, Position},
	style::{Color, Modifier, Style},
	widgets::{Block, Borders, List, ListItem, Paragraph},
};
use std::str::FromStr;


fn permissions_string(mode: u32) -> String {
	let chars = [
		(0o400, 'r'), (0o200, 'w'), (0o100, 'x'),
		(0o040, 'r'), (0o020, 'w'), (0o010, 'x'),
		(0o004, 'r'), (0o002, 'w'), (0o001, 'x'),
	];

	chars
		.iter()
		.map(|(bit, c)| {
			if mode & bit != 0 {
				*c
			} else {
				'-'
			}
		})
		.collect()
}

fn format_time(time: std::time::SystemTime) -> String {
	let dt: DateTime<Local> = time.into();

	dt.format("%Y-%m-%d %H:%M").to_string()
}

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
			} else if item.is_dir {
				Style::default()
					.fg(bo)
			} else {
				Style::default()
					.fg(fg)
			};

			ListItem::new(item.name.clone())
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

	let text = if app.command_mode {
		format!(":{}", app.input)
	} else {
		match selected {
			Some(item) => format!(
				"P: {} | S: {} | C: {} | M: {}",
				permissions_string(item.permissions),
				if item.is_dir {
					"<DIR>".to_string()
				} else {
					human_size(item.size)
				},
				format_time(item.created),
				format_time(item.modified),
			),
			None => "Empty directory".to_string(),
		}
	};
	
	let status = Paragraph::new(text)
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

	frame.render_stateful_widget(
		list,
		chunks[0],
		&mut app.list_state
	);

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