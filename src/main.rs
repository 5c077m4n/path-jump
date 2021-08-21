use std::{
	env::{current_dir, temp_dir},
	path::PathBuf,
};
use structopt::{self, StructOpt};

use rusqlite::{Connection, Result};

mod lib;
use lib::queries;

#[derive(Debug, StructOpt)]
#[structopt(name = "Path Jumper options", about = "All of the options for PJ.")]
pub struct Opt {
	#[structopt(parse(from_os_str))]
	dir: Option<PathBuf>,
	#[structopt(short, long)]
	add: Option<PathBuf>,
	#[structopt(short, long)]
	dump: bool,
	#[structopt(long)]
	clear_history: bool,
}

fn main() -> Result<()> {
	let opt = Opt::from_args();
	let db_conn = Connection::open(temp_dir().join(".pj.db"))?;

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

		queries::upsert_path(&db_conn, dir_path)?;
	} else if let Some(dir) = opt.dir {
		let dir = current_dir().unwrap().join(dir);
		let dir = dir.canonicalize().unwrap();
		let dir = dir.to_str().unwrap();

		let result = queries::find_dir(&db_conn, &dir)?;
		println!("{}", &result);
	} else {
		queries::create_table(&db_conn)?;
	}

	Ok(())
}
