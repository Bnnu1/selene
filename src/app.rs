use ratatui::widgets::ListState;
use std::{env, fs, path::PathBuf};
use std::path::Path;
use std::process::Command;

pub struct App {
	pub cwd: PathBuf,
	pub items: Vec<PathBuf>,
	pub list_state: ListState,
	pub hidden: bool,
	pub marked: Vec<PathBuf>,
	pub command_mode: bool,
	pub input: String,
}

impl App {
	pub fn new() -> Self {
		let mut list_state = ListState::default()
			.with_selected(Some(0));

		let home = PathBuf::from(
			env::var("HOME").expect("HOME environment variable not set")
		);
		
		Self {
			items: vec![],
			cwd: home,
			list_state,
			hidden: false,
			marked: vec![],
			command_mode: false,
			input: String::new(),
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

		self.list_state.select(Some(0));
		self.items = self.get_items().unwrap()
	}

	pub fn previous_dir(&mut self) {
		let selected = self.list_state.selected().unwrap_or(0);

		if let Some(parent) = self.cwd.parent() {
			self.cwd = parent.to_path_buf();
		}

		self.list_state.select(Some(0));
		self.items = self.get_items().unwrap()
	}

	pub fn get_items(&self) -> std::io::Result<Vec<PathBuf>> {
		let mut entries: Vec<PathBuf> = fs::read_dir(&self.cwd)?
			.filter_map(Result::ok)
			.map(|e| e.path())
			.filter(|path| {
				self.hidden
					|| !path
					.file_name()
					.and_then(|name| name.to_str())
					.is_some_and(|name| name.starts_with('.'))
			})
			.collect();

		entries.sort();

		Ok(entries)
	}

	pub fn tog_hidden(&mut self) {
		self.hidden = !self.hidden;
		self.items = self.get_items().unwrap();
	}

	pub fn mark(&mut self) {
		let selected = match self.list_state.selected() {
			Some(i) => i,
			None => return,
		};

		let path = match self.items.get(selected) {
			Some(path) => path.clone(),
			None => return,
		};

		if let Some(pos) = self.marked.iter().position(|p| p == &path) {
			self.marked.remove(pos);
		} else {
			self.marked.push(path);
		}
	}

	pub fn run_command(&mut self) {
		fn shell_escape(path: &Path) -> String {
			format!("'{}'", path.to_string_lossy().replace('\'', "'\\''"))
		}

		let selected = &self.items[self.list_state.selected().unwrap()];

		let marked = self
			.marked
			.iter()
			.map(|p| shell_escape(p))
			.collect::<Vec<_>>()
			.join(" ");

		let cmd = self
			.input
			.replace("%m", &marked)
			.replace("%s", &shell_escape(selected))
			.replace("%d", &shell_escape(&self.cwd));

		match Command::new("sh")
			.arg("-c")
			.arg(&cmd)
			.current_dir(&self.cwd)
			.status()
		{
			Ok(_) => {}
			Err(e) => eprintln!("{e}"),
		}

		self.marked = vec![];
	}
}