use std::{env, fs, io, path::Path};

fn main() -> io::Result<()> {
	let out_dir = env::var_os("OUT_DIR").unwrap();
	let dest_path = Path::new(&out_dir).join("pj.sh");
	fs::write(
		&dest_path,
		r#"
        #!/bin/sh

        ./pj
        alias ${PJ_CMD:-pj}='./pj --add "$1" && cd "$_"'
        "#,
	)?;

	println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}
