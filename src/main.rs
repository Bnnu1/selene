mod app;
mod ui;

use crate::app::App;

use crossterm::event::{KeyCode, self};
use std::io;

fn main() -> io::Result<()> {
	let mut app = App::new();
	app.items = app.get_items().unwrap();
	app.update_preview();
	let mut terminal = ratatui::init();

	loop {
		terminal.draw(|frame| ui::render(frame, &mut app))?;
		if let Some(key) = event::read()?.as_key_press_event() {
			match app.command_mode {
				true => {
					match key.code {
						KeyCode::Char(c) => app.input.push(c),
						KeyCode::Backspace => {
							app.input.pop();
						},
						KeyCode::Enter => {
							ratatui::restore();
							app.run_command();
							terminal = ratatui::init();
							app.items = app.get_items().unwrap();
							terminal.draw(|frame| ui::render(frame, &mut app))?;
							app.input.clear();
							app.command_mode = false;
						},
						KeyCode::Esc => {
							app.input.clear();
							app.command_mode = false;
						},
						_ => {}
					}
				},
				false => {
					match key.code {
						KeyCode::Down => app.next(),
						KeyCode::Up => app.previous(),
						KeyCode::Right => app.next_dir(),
						KeyCode::Left => app.previous_dir(),
						KeyCode::Esc => break,
						KeyCode::Char('z') => app.tog_hidden(),
						KeyCode::Char(' ') => app.mark(),
						KeyCode::Char(':') => {
							app.command_mode = true;
						},
						_ => {}
					}
				}
			}
		}
	}

	ratatui::restore();

	Ok(())
}