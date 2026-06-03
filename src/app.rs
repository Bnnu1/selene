use ratatui::widgets::ListState;
use serde::Deserialize;
use std::{env, fs};
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct App {
	pub cwd: PathBuf,
	pub items: Vec<PathBuf>,
	pub list_state: ListState,
	pub hidden: bool,
	pub marked: Vec<PathBuf>,
	pub command_mode: bool,
	pub input: String,
	pub preview: String,
	pub config: Config,
}

#[derive(Deserialize, Debug)]
pub struct Config {
	pub editor: String,
	pub background: String,
	pub foreground: String,
	pub border: String,
}

impl App {
	pub fn new() -> Self {
		let list_state = ListState::default()
			.with_selected(Some(0));

		let cwd = PathBuf::from(
			env::current_dir().expect("Couldn't get current directory")
		);

		let file_content = fs::read_to_string("/usr/share/selene/config.json").expect("No config.json");
		
		let config: Config = serde_json::from_str(&file_content).expect("Couldn't create config struct");

		Self {
			items: vec![],
			cwd: cwd,
			list_state,
			hidden: false,
			marked: vec![],
			command_mode: false,
			input: String::new(),
			preview: String::new(),
			config: config,
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
		self.update_preview();
	}

	pub fn previous(&mut self) {
		let selected = self.list_state.selected().unwrap_or(0);

		let previous = if selected != 0 {
			selected - 1
		} else {
			selected
		};


		self.list_state.select(Some(previous));
		self.update_preview();
	}

	pub fn next_dir(&mut self) {
		let selected = self.list_state.selected().unwrap_or(0);

		if let Some(path) = self.items.get(selected) {
			if path.is_dir() {
				self.cwd = path.to_path_buf();
			}
		}

		self.items = self.get_items().unwrap();

		self.list_state.select(
			if self.items.is_empty() {
				None
			} else {
				Some(0)
			}
		);
	}

	pub fn previous_dir(&mut self) {
		if let Some(parent) = self.cwd.parent() {
			self.cwd = parent.to_path_buf();
		}

		self.items = self.get_items().unwrap();

		self.list_state.select(
			if self.items.is_empty() {
				None
			} else {
				Some(0)
			}
		);
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

		self.list_state.select(
			if self.items.is_empty() {
				None
			} else {
				Some(0)
			}
		);
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

	pub fn update_preview(&mut self) {
		let Some(index) = self.list_state.selected() else {
			self.preview.clear();
			return;
		};

		let selected = &self.items[index];

		self.preview = if selected.is_file() {
		std::fs::read_to_string(selected)
			.map(|s| {
			s.lines()
				.take(50)
				.collect::<Vec<_>>()
				.join("\n")
			})
			.unwrap_or_else(|_| "<unable to read file>".into())
		} else {
		match std::fs::read_dir(selected) {
			Ok(entries) => entries
				.flatten()
				.take(100)
				.map(|e| e.file_name().to_string_lossy().into_owned())
				.collect::<Vec<_>>()
				.join("\n"),
			Err(_) => "<unable to read directory>".into(),
		}
		};
	}

	pub fn open_in_editor(&self) {
		let Some(index) = self.list_state.selected() else {
			return;
		};

		let selected = &self.items[index];

		if !selected.is_file() {
			return;
		}

		match Command::new(&self.config.editor)
			.arg(selected)
			.current_dir(&self.cwd)
			.status()
		{
			Ok(_) => {}
			Err(e) => eprintln!("{e}"),
		}
	}
}