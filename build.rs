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
		.join("pj.sh");

	let mut file = File::create(&dest_path)?;
	file.write_all(
		r#"#!/bin/sh

__PJ_BIN_PATH="$(pwd)/$(dirname "$0")/pj"

"$__PJ_BIN_PATH"

cd () {
    "$__PJ_BIN_PATH --add $1 &" &>/dev/null
    builtin cd "$1"
}
${PJ_CUSTOM_CMD:-pj} () {
    builtin cd "$("$__PJ_BIN_PATH" "$1")"
}
"#
		.as_bytes(),
	)?;

	println!("cargo:rerun-if-changed=build.rs");
	Ok(())
}
