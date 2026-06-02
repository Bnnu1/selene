mod app;
mod ui;

use crate::app::App;

use crossterm::event::{self, KeyCode};
use std::io;

fn main() -> io::Result<()> {
	let mut app = App::new();
	let mut terminal = ratatui::init();

	loop {
		terminal.draw(|frame| ui::render(frame, &mut app))?;
		if let Some(key) = event::read()?.as_key_press_event() {
			match key.code {
				KeyCode::Char('q') => break,
				KeyCode::Down => app.next(),
				KeyCode::Up => app.previous(),
				KeyCode::Right => app.next_dir(),
				KeyCode::Left => app.previous_dir(),
				_ => {}
			}
		}
	}

	ratatui::restore();

	Ok(())
}