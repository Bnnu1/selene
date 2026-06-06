use crate::app::App;
use crossterm::event::{self, KeyCode};
use std::io;

pub fn handle_event(
	app: &mut App,
	terminal: &mut ratatui::DefaultTerminal,
) -> io::Result<()> {
	if let Some(key) = event::read()?.as_key_press_event() {
		match app.command_mode {
			true => handle_command_mode(app, terminal, key.code)?,
			false => handle_normal_mode(app, terminal, key.code)?,
		}
	}

	Ok(())
}

fn handle_command_mode(
	app: &mut App,
	terminal: &mut ratatui::DefaultTerminal,
	code: KeyCode,
) -> io::Result<()> {
	match code {
		KeyCode::Char(c) => {
			app.input.insert(app.cursor_pos, c);
			app.cursor_pos += 1;
		}

		KeyCode::Backspace => {
			if app.cursor_pos > 0 {
				app.cursor_pos -= 1;
				app.input.remove(app.cursor_pos);
			}
		}

		KeyCode::Enter => {
			ratatui::restore();
			app.run_command();

			*terminal = ratatui::init();

			app.items = app.get_items().unwrap();
			app.input.clear();
			app.command_mode = false;
		}

		KeyCode::Esc => {
			app.input.clear();
			app.command_mode = false;
		}

		KeyCode::Left => {
			app.cursor_pos = app.cursor_pos.saturating_sub(1);
		}

		KeyCode::Right => {
			if app.cursor_pos < app.input.len() {
				app.cursor_pos += 1;
			}
		}

		_ => {}
	}

	Ok(())
}

fn handle_normal_mode(
	app: &mut App,
	terminal: &mut ratatui::DefaultTerminal,
	code: KeyCode,
) -> io::Result<()> {
	match code {
		KeyCode::Down => app.next(),
		KeyCode::Up => app.previous(),
		KeyCode::Right => app.next_dir(),
		KeyCode::Left => app.previous_dir(),
		KeyCode::Esc => app.running = false,
		KeyCode::Char('z') => app.tog_hidden(),
		KeyCode::Char(' ') => app.mark(),

		KeyCode::Char(':') => {
			app.command_mode = true;
		}

		KeyCode::Enter => {
			ratatui::restore();
			app.open_in_editor();

			*terminal = ratatui::init();

			app.items = app.get_items().unwrap();
		}

		_ => {}
	}

	Ok(())
}