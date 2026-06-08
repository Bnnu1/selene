use ratatui::widgets::ListState;
use serde::Deserialize;
use std::{env, fs};
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::item_info::ItemInfo;

pub struct App {
	pub running: bool,
	pub cwd: PathBuf,
	pub items: Vec<ItemInfo>,
	pub list_state: ListState,
	pub hidden: bool,
	pub marked: Vec<ItemInfo>,

	pub preview: String,

	pub command_mode: bool,
	pub input: String,
	pub cursor_pos: usize,

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
			env::current_dir()
				.expect("Couldn't get current directory")
		);

		let file_content =
			fs::read_to_string("/usr/share/selene/config.json")
				.expect("No config.json");

		let config: Config = serde_json::from_str(&file_content)
			.expect("Couldn't create config struct");

		Self {
			running: true,
			items: vec![],
			cwd,
			list_state,
			hidden: false,
			marked: vec![],
			command_mode: false,
			input: String::new(),
			cursor_pos: 0,
			preview: String::new(),
			config,
		}
	}

	pub fn next(&mut self) {
		if self.items.is_empty() {
			return;
		}

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
		if self.items.is_empty() {
			return;
		}

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

		let item = match self.items.get(selected) {
			Some(item) => item,
			None => return,
		};

		if item.is_dir {
			self.cwd = item.path.clone();

			self.items = self.get_items().unwrap();

			self.list_state.select(
				if self.items.is_empty() {
					None
				} else {
					Some(0)
				}
			);
		}

		self.update_preview();
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

		self.update_preview();
	}

	pub fn get_items(&self) -> std::io::Result<Vec<ItemInfo>> {
		let mut items: Vec<ItemInfo> = fs::read_dir(&self.cwd)?
			.filter_map(|entry| {
				let path = entry.ok()?.path();
				ItemInfo::new(&path).ok()
			})
			.filter(|item| {
				self.hidden || !item.name.starts_with('.')
			})
			.collect();

		items.sort_by(|a, b| {
			b.is_dir
				.cmp(&a.is_dir)
				.then_with(|| a.name.cmp(&b.name))
		});

		Ok(items)
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

		let item = match self.items.get(selected) {
			Some(item) => item.clone(),
			None => return,
		};

		if let Some(pos) = self.marked
			.iter()
			.position(|p| p.path == item.path)
		{
			self.marked.remove(pos);
		} else {
			self.marked.push(item);
		}
	}

	pub fn run_command(&mut self) {
		fn shell_escape(path: &Path) -> String {
			format!(
				"'{}'",
				path.to_string_lossy()
					.replace('\'', "'\\''")
			)
		}

		let selected = self
			.list_state
			.selected()
			.and_then(|i| self.items.get(i));

		let marked = self
			.marked
			.iter()
			.map(|p| shell_escape(&p.path))
			.collect::<Vec<_>>()
			.join(" ");

		let cmd = self
			.input
			.replace("%m", &marked)
			.replace("%s", 
				&selected
					.map(|item| shell_escape(&item.path))
					.unwrap_or_default(),
			)
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

		self.marked.clear();
		self.cursor_pos = 0;
		self.items = self.get_items().unwrap();
		if self.items.is_empty() {
			self.list_state.select(None);
		} else {
			self.list_state.select(Some(0));
		}
		self.update_preview();
	}

	pub fn update_preview(&mut self) {
		let Some(index) = self.list_state.selected() else {
			self.preview.clear();
			return;
		};

		let Some(selected) = self.items.get(index) else {
			self.preview.clear();
			return;
		};

		self.preview = if !selected.is_dir {
			std::fs::read_to_string(&selected.path)
				.map(|s| {
					s.lines()
						.take(50)
						.collect::<Vec<_>>()
						.join("\n")
				})
				.unwrap_or_else(|_| {
					"<unable to read file>".into()
				})
		} else {
			match std::fs::read_dir(&selected.path) {
				Ok(entries) => entries
					.flatten()
					.take(100)
					.map(|e| {
						e.file_name()
							.to_string_lossy()
							.into_owned()
					})
					.collect::<Vec<_>>()
					.join("\n"),
				Err(_) => {
					"<unable to read directory>".into()
				}
			}
		};
	}

	pub fn open_in_editor(&self) {
		let Some(index) = self.list_state.selected() else {
			return;
		};

		let Some(selected) = self.items.get(index) else {
			return;
		};

		if selected.is_dir {
			return;
		}

		match Command::new(&self.config.editor)
			.arg(&selected.path)
			.current_dir(&self.cwd)
			.status()
		{
			Ok(_) => {}
			Err(e) => eprintln!("{e}"),
		}
	}
}