use std::{
    fs,
    io,
    os::unix::fs::PermissionsExt,
    path::PathBuf,
    time::SystemTime,
};

#[derive(Debug, Clone, PartialEq)]
pub struct ItemInfo {
	pub path: PathBuf,
	pub is_dir: bool,
	pub name: String,
	pub size: u64,
	pub modified: SystemTime,
	pub created: SystemTime,
	pub permissions: u32,
}

impl ItemInfo {
	pub fn new(path: &PathBuf) -> io::Result<Self> {
		let metadata = fs::metadata(path)?;

		Ok(Self {
			path: path.clone(),
			is_dir: metadata.is_dir(),
			name: path
				.file_name()
				.unwrap_or_default()
				.to_string_lossy()
				.into_owned(),
			size: metadata.len(),
			modified: metadata.modified()?,
			created: metadata.created()?,
			permissions: metadata.permissions().mode() & 0o777,
		})
	}
}