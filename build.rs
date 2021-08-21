use std::{
	env,
	fs::File,
	io::{self, Write},
	path::Path,
};

fn main() -> io::Result<()> {
	let out_dir = env::var_os("OUT_DIR").unwrap();
	let dest_path = Path::new(&out_dir).join("pj-init.sh");

	let mut file = File::create(&dest_path)?;
	file.write_all(
		b"
        #!/bin/sh

        ./pj
        alias ${PJ_CMD:-pj}='./pj --add \"$1\" && builtin cd \"$_\"'
        ",
	)?;

	println!("cargo:rerun-if-changed=build.rs");
	Ok(())
}
