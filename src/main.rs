use std::{
	env::{current_dir, temp_dir},
	path::PathBuf,
};

use rusqlite::{Connection, Result};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Path Jumper options", about = "All of the options for PJ.")]
struct Opt {
	#[structopt(parse(from_os_str))]
	dir: Option<PathBuf>,
	#[structopt(short, long)]
	add: Option<PathBuf>,
	#[structopt(short, long)]
	dump: bool,
	#[structopt(long)]
	clear_history: bool,
}

#[derive(Debug)]
struct DirScore {
	path: PathBuf,
	score: usize,
	created_at: usize,
}

fn main() -> Result<()> {
	let opt = Opt::from_args();
	let db_conn = Connection::open(temp_dir().join(".pj.db"))?;

	if opt.clear_history {
		db_conn.execute("DELETE FROM dir_scoring", [])?;
	} else if opt.dump {
		let mut stmt =
			db_conn.prepare("select ds.path, ds.score, ds.created_at from dir_scoring as ds")?;
		let dump = stmt.query_map([], |row| {
			Ok(DirScore {
				path: PathBuf::from(row.get::<usize, String>(0)?),
				score: row.get(1)?,
				created_at: row.get::<usize, usize>(2)?.into(),
			})
		})?;
		for dump_row in dump {
			let dump_row = dump_row?;
			println!("{:?}", dump_row);
		}
	} else if let Some(dir_path) = opt.add {
		let dir_path = current_dir().unwrap().join(dir_path);
		let dir_path = dir_path.canonicalize().unwrap();
		let dir_path = dir_path.to_str().unwrap();

		db_conn.execute(
			"INSERT INTO dir_scoring (path) VALUES (:path)
                    ON CONFLICT(path) DO UPDATE SET score = score + 1 WHERE path = :path",
			&[(":path", dir_path)],
		)?;
	} else {
		db_conn.execute(
			"CREATE TABLE IF NOT EXISTS dir_scoring (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    path TEXT NOT NULL UNIQUE,
                    score INTEGER NOT NULL DEFAULT 0,
                    created_at INTEGER DEFAULT (STRFTIME('%Y-%m-%d %H:%M:%f', 'NOW', 'localtime')) NOT NULL
                )",
			[],
		)?;
	}

	Ok(())
}
