use std::{env, fs, io, path::Path};

fn main() -> io::Result<()> {
	let project_root = env::var_os("CARGO_MANIFEST_DIR").unwrap();
	let target_profile = env::var_os("PROFILE").unwrap();

	let origin_path = Path::new(&project_root)
		.join("resources")
		.join("install.sh");
	let dest_path = Path::new(&project_root)
		.join("target")
		.join(&target_profile)
		.join("j.sh");

	fs::copy(&origin_path, &dest_path)?;

	println!("cargo:rerun-if-changed=build.rs");
	Ok(())
}
