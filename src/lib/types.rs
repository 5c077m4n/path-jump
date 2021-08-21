use std::path::PathBuf;

#[derive(Debug)]
pub struct DirScore {
	pub path: PathBuf,
	pub score: usize,
	pub created_at: usize,
}
