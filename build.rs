use std::{
	env,
	fs::File,
	io::{self, Write},
	path::Path,
};

fn main() -> io::Result<()> {
	let project_root = env::var_os("CARGO_MANIFEST_DIR").unwrap();
	let target_profile = env::var_os("PROFILE").unwrap();
	let dest_path = Path::new(&project_root)
		.join("target")
		.join(&target_profile)
		.join("j.sh");

	let mut file = File::create(&dest_path)?;
	file.write_all(
		r#"#!/bin/sh

__J="$(pwd)/$(dirname "$0")/j"
$__J

cd () {
    "$__J --add $1 &" &>/dev/null
    builtin cd "$1"
}
${J_CUSTOM_CMD:-j} () {
    builtin cd "$($__J "$1")"
}
"#
		.as_bytes(),
	)?;

	println!("cargo:rerun-if-changed=build.rs");
	Ok(())
}
