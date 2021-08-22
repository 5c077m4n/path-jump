use std::{
	env::{self, current_dir},
	fs,
	path::PathBuf,
};
use structopt::{self, StructOpt};

use rusqlite::{Connection, Result};

mod lib;
use lib::queries;

#[derive(Debug, StructOpt)]
#[structopt(name = "Path Jump options", about = "All of the options for Path Jump.")]
pub struct Opt {
	#[structopt()]
	dir: Option<String>,
	#[structopt(short, long)]
	add: Option<PathBuf>,
	#[structopt(short, long)]
	dump: bool,
	#[structopt(long)]
	clear_history: bool,
}

fn main() -> Result<()> {
	let opt = Opt::from_args();
	let home_dir = PathBuf::from(env::var_os("HOME").unwrap());
	let db_path = home_dir.join(".config").join("j");
	fs::create_dir_all(&db_path).unwrap();

	let mut db_conn = Connection::open(db_path.join("j.db"))?;

	if opt.clear_history {
		queries::clear_history(&db_conn)?;
	} else if opt.dump {
		let dump = queries::get_dump(&db_conn)?;
		for dump_row in dump {
			let dump_row = dump_row;
			println!("{:#?}", dump_row);
		}
	} else if let Some(dir_path) = opt.add {
		let dir_path = current_dir().unwrap().join(dir_path);
		let dir_path = dir_path.canonicalize().unwrap();
		let dir_path = dir_path.to_str().unwrap();

		queries::upsert_dir(&db_conn, dir_path)?;
	} else if let Some(dir) = opt.dir {
		let result = queries::find_dir(&db_conn, &dir)?;
		println!("{}", &result);
	} else {
		queries::create_table(&mut db_conn)?;
	}

	Ok(())
}
