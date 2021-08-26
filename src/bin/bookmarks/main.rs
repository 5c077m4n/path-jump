use std::env::current_dir;

use lib::{paths::get_state_dir, queries};
use rusqlite::{Connection, Result};
use structopt::{self, StructOpt};

#[derive(Debug, StructOpt)]
#[structopt(name = "Pathman options", about = "All of the options for pathman")]
pub struct Opt {
	#[structopt()]
	get: Option<String>,
	#[structopt(short, long)]
	add: Option<String>,
	#[structopt(short, long)]
	delete: Option<String>,
	#[structopt(short, long)]
	dump: bool,
}

fn main() -> Result<()> {
	let opt = Opt::from_args();

	let state_dir = get_state_dir().unwrap();
	let mut db_conn = Connection::open(state_dir.join("pathman.db"))?;
	queries::init(&mut db_conn)?;

	if let Some(name) = opt.get {
		let bm = queries::bookmark::get_bookmark(&db_conn, &name)?;
		println!("{:?}", bm);
	} else if let Some(name) = opt.add {
		let dir_path = current_dir().unwrap();

		if let Ok(normalized_dir) = dir_path.canonicalize() {
			let normalized_dir = normalized_dir.to_str().unwrap();
			queries::bookmark::add(&db_conn, &name, normalized_dir)?;
		}
	} else if let Some(name) = opt.delete {
		queries::bookmark::delete(&db_conn, &name)?;
	} else if opt.dump {
		let dump = queries::bookmark::get_dump(&db_conn)?;
		for dump_row in dump {
			println!("{:#?}", dump_row);
		}
	}

	Ok(())
}
