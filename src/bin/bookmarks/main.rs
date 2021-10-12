use std::env::current_dir;

use rusqlite::Connection;
use structopt::{self, StructOpt};

use lib::{errors::ErrorType, paths::get_state_dir, queries};

#[derive(Debug, StructOpt)]
#[structopt(
	name = "Pathman bookmarks options",
	about = "All of the options for pathman bookmarks"
)]
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

fn main() -> Result<(), ErrorType> {
	let opt = Opt::from_args();

	let state_dir = get_state_dir()?;
	let mut db_conn = Connection::open(state_dir.join("pathman.db"))?;
	queries::bookmark::init_tables(&mut db_conn)?;

	if let Some(name) = opt.get {
		let bm = queries::bookmark::get(&db_conn, &name)?;
		println!("{:?}", bm);
	} else if let Some(name) = opt.add {
		let dir_path = current_dir()?;

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
