use crate::app::App;

use ratatui::{
	Frame,
	layout::{Constraint, Direction, Layout},
	style::{Color, Modifier, Style},
	widgets::{Block, Borders, List, ListItem, Paragraph},
};

const BG: Color = Color::Rgb(10,0,30);
const FG: Color = Color::Rgb(244, 246, 240);
const BO: Color = Color::Rgb(82,47,129);

pub fn render(frame: &mut Frame, app: &mut App) {
	let selected = &app.items[app.list_state.selected().unwrap()];
	
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
				.title(app.cwd.to_string_lossy().to_string())
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

	let size_text = match std::fs::metadata(selected) {
		Ok(meta) if meta.is_file() => human_size(meta.len()),
		Ok(_) => "<DIR>".to_string(),
		Err(_) => "?".to_string(),
	};

	let text = if app.command_mode {
		format!(":{}", app.input)
	} else {
		size_text
	};

	let status = Paragraph::new(text)
		.style(
			Style::default()
			.fg(FG)
			.bg(BG)
		)
		.block(
			Block::default()
			.borders(Borders::ALL)
			.border_style(
				Style::default().fg(BO)
			)
		);

	frame.render_stateful_widget(list, chunks[0], &mut app.list_state);
	frame.render_widget(
		Paragraph::new(app.preview.as_str())
			.style(
				Style::default()
				.fg(FG)
				.bg(BG)
			)
			.block(
				Block::default()
					.title("Preview")
					.borders(Borders::ALL)
					.border_style(
						Style::default().fg(BO)
					),
			),
		chunks[1],
	);
	frame.render_widget(status, vertical[1]);
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