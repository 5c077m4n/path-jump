use std::{env, ffi::OsString, fs, io, path::Path};

fn main() -> io::Result<()> {
	let project_root = env::var_os("CARGO_MANIFEST_DIR").unwrap();
	let target_profile = env::var_os("PROFILE").unwrap();
	let target_dir = env::var_os("CARGO_TARGET_DIR").unwrap_or_else(|| OsString::from("target"));

	let init_script = Path::new(&project_root).join("resources").join("j.sh");
	let dest_path = Path::new(&project_root)
		.join(&target_dir)
		.join(&target_profile)
		.join("j.sh");

	fs::copy(&init_script, &dest_path)?;

	println!("cargo:rerun-if-changed=build.rs");
	println!("cargo:rerun-if-changed=resources/j.sh");
	Ok(())
}
