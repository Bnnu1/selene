use ratatui::widgets::ListState;
use std::{env, fs, path::PathBuf};

pub struct App {
	pub cwd: PathBuf,
	pub items: Vec<PathBuf>,
	pub list_state: ListState,
}

impl App {
	pub fn new() -> Self {
		let mut list_state = ListState::default()
			.with_selected(Some(0));

		let home = PathBuf::from(
			env::var("HOME").expect("HOME environment variable not set")
		);
		
		Self {
			items: Self::get_items(&home).unwrap(),
			cwd: home,
			list_state,
		}
	}

	pub fn next(&mut self) {
		let selected = self.list_state.selected().unwrap_or(0);

		let next = if selected != self.items.len() - 1 {
			selected + 1
		} else {
			selected
		};

		self.list_state.select(Some(next));
	}

	pub fn previous(&mut self) {
		let selected = self.list_state.selected().unwrap_or(0);

		let previous = if selected != 0 {
			selected - 1
		} else {
			selected
		};


		self.list_state.select(Some(previous));
	}

	pub fn next_dir(&mut self) {
		let selected = self.list_state.selected().unwrap_or(0);

		if let Some(path) = self.items.get(selected) {
			if path.is_dir() {
				self.cwd = path.to_path_buf();
			}
		}

		self.items = Self::get_items(&self.cwd).unwrap()
	}

	pub fn previous_dir(&mut self) {
		let selected = self.list_state.selected().unwrap_or(0);

		if let Some(parent) = self.cwd.parent() {
			self.cwd = parent.to_path_buf();
		}

		self.items = Self::get_items(&self.cwd).unwrap()
	}

	fn get_items(dir: &PathBuf) -> std::io::Result<Vec<PathBuf>> {
		let entries = fs::read_dir(dir)?
			.map(|entry| entry.map(|e| e.path()))
			.collect::<Result<Vec<_>, _>>()?;

		Ok(entries)
	}
}