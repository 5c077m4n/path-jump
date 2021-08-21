use std::{env, fs, path::Path};

fn main() {
	let out_dir = env::var_os("OUT_DIR").unwrap();
	let dest_path = Path::new(&out_dir).join("pj.sh");
	fs::write(
		&dest_path,
		r#"
        echo "glue file"
        "#,
	)
	.unwrap();
	println!("cargo:rerun-if-changed=build.rs");
}
