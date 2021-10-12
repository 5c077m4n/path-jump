use std::{
	env, fs, io,
	path::PathBuf,
	time::{SystemTime, UNIX_EPOCH},
};

pub fn get_state_dir() -> io::Result<PathBuf> {
	let state_dir = if let Some(xdg_state_dir) = env::var_os("XDG_STATE_HOME") {
		PathBuf::from(xdg_state_dir)
	} else {
		let home_dir = env::var_os("HOME").unwrap();
		PathBuf::from(home_dir).join(".local").join("state")
	};
	let state_dir = state_dir.join("pathman");
	fs::create_dir_all(&state_dir)?;

	Ok(state_dir)
}

pub fn get_tmp_file() -> io::Result<PathBuf> {
	let epoch = SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.unwrap()
		.as_secs();
	let tmp_dir = env::temp_dir().join("pathman");
	fs::create_dir_all(&tmp_dir)?;

	let tmp_file = tmp_dir.join(epoch.to_string());
	Ok(tmp_file)
}
