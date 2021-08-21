use std::{
	env,
	fs::File,
	io::{self, Write},
	path::Path,
};

fn main() -> io::Result<()> {
	let out_dir = env::var_os("OUT_DIR").unwrap();
	let dest_path = Path::new(&out_dir).join("pj.sh");

	let mut file = File::create(&dest_path)?;
	file.write_all(
		b"
        #!/bin/sh

        ./pj
        alias cd='./pj --add \"$1\" && builtin cd \"$_\"'
        alias ${PJ_CUSTOM_CMD:-pj}='builtin cd \"$(./pj $1)\"'
        ",
	)?;

	println!("cargo:rerun-if-changed=build.rs");
	Ok(())
}
