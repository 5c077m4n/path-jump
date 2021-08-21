use std::path::PathBuf;

use rusqlite::{Connection, Result};

use super::types::DirScore;

pub fn clear_history(conn: &Connection) -> Result<usize> {
	conn.execute("DELETE FROM dir_scoring", [])
}

pub fn get_dump(conn: &Connection) -> Result<Vec<DirScore>> {
	let mut stmt =
		conn.prepare("select ds.path, ds.score, ds.created_at from dir_scoring as ds")?;
	let mut rows = stmt.query([])?;

	let mut dir_list: Vec<DirScore> = Vec::new();
	while let Some(row) = rows.next()? {
		dir_list.push(DirScore {
			path: PathBuf::from(row.get::<usize, String>(0)?),
			score: row.get(1)?,
			created_at: row.get(2)?,
		});
	}
	Ok(dir_list)
}

pub fn upsert_path(conn: &Connection, dir_path: &str) -> Result<usize> {
	conn.execute(
		"INSERT INTO dir_scoring (path) VALUES (:path)
            ON CONFLICT(path) DO UPDATE SET score = score + 1 WHERE path = :path",
		&[(":path", dir_path)],
	)
}

pub fn create_table(conn: &Connection) -> Result<usize> {
	conn.execute(
		"CREATE TABLE IF NOT EXISTS dir_scoring (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            path TEXT NOT NULL UNIQUE,
            score INTEGER NOT NULL DEFAULT 0,
            created_at INTEGER DEFAULT (CAST((julianday('now') - 2440587.5) * 86400000 AS INTEGER)) NOT NULL
        )",
		[],
	)
}

pub fn find_dir(conn: &Connection, dir_path: &str) -> Result<String> {
	let dir_path = &dir_path.to_lowercase();
	conn.query_row(
		"SELECT ds.path
            FROM dir_scoring as ds
            WHERE lower(ds.path) LIKE :path
            ORDER BY ds.score DESC, ds.created_at DESC, ds.path ASC
        ",
		&[(":path", dir_path)],
		|row| row.get(0),
	)
}
