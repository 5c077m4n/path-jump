use std::path::PathBuf;

#[derive(Debug)]
pub struct DirScore {
	pub path: PathBuf,
	pub score: usize,
	pub created_at: usize,
	pub updated_at: usize,
}

#[derive(Debug)]
pub struct Bookmark {
	pub name: String,
	pub path: PathBuf,
	pub created_at: usize,
	pub updated_at: usize,
}
