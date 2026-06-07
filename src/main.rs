mod app;
mod ui;
mod event;
mod item_info;

use crate::app::App;
use std::io;

fn main() -> io::Result<()> {
	let mut app = App::new();
	app.items = app.get_items().unwrap();
	app.update_preview();

	let mut terminal = ratatui::init();

	while app.running {
		terminal.draw(|frame| ui::render(frame, &mut app))?;
		event::handle_event(&mut app, &mut terminal)?;
	}

	ratatui::restore();
	Ok(())
}