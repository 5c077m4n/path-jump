use std::path::PathBuf;

use rusqlite::{Connection, Result};

use super::types::DirScore;


pub fn create_table(conn: &Connection) -> Result<usize> {
	conn.execute(
		"CREATE TABLE IF NOT EXISTS dir_scoring (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            path TEXT NOT NULL UNIQUE,
            score INTEGER NOT NULL DEFAULT 0,
            created_at INTEGER DEFAULT (CAST((julianday('now') - 2440587.5) * 86400000 AS INTEGER)) NOT NULL,
            updated_at INTEGER DEFAULT (CAST((julianday('now') - 2440587.5) * 86400000 AS INTEGER)) NOT NULL
        )",
		[],
	)?;
	conn.execute(
		"CREATE TRIGGER IF NOT EXISTS dir_scoring__updated_at
            AFTER UPDATE
            ON dir_scoring
            FOR EACH ROW
            BEGIN
                UPDATE dir_scoring
                    SET updated_at = (CAST((julianday('now') - 2440587.5) * 86400000 AS INTEGER))
                    WHERE id = OLD.id;
            END
        ",
		[],
	)
}

pub fn upsert_dir(conn: &Connection, dir_path: &str) -> Result<usize> {
	conn.execute(
		"INSERT INTO dir_scoring (path) VALUES (:path)
            ON CONFLICT(path) DO UPDATE SET score = score + 1 WHERE path = :path",
		&[(":path", dir_path)],
	)
}

pub fn find_dir(conn: &Connection, dir_path: &str) -> Result<String> {
	let dir_path = &dir_path.to_lowercase();
	conn.query_row(
		"SELECT ds.path
            FROM dir_scoring as ds
            WHERE lower(ds.path) LIKE '%' || :path || '%'
            ORDER BY ds.score DESC, ds.updated_at DESC, ds.path ASC
        ",
		&[(":path", dir_path)],
		|row| row.get(0),
	)
}

pub fn clear_history(conn: &Connection) -> Result<usize> {
	conn.execute("DELETE FROM dir_scoring", [])
}

pub fn get_dump(conn: &Connection) -> Result<Vec<DirScore>> {
	let mut stmt = conn
		.prepare("SELECT ds.path, ds.score, ds.created_at, ds.updated_at FROM dir_scoring AS ds")?;
	let mut rows = stmt.query([])?;

	let mut dir_list: Vec<DirScore> = Vec::new();
	while let Some(row) = rows.next()? {
		dir_list.push(DirScore {
			path: PathBuf::from(row.get::<usize, String>(0)?),
			score: row.get(1)?,
			created_at: row.get(2)?,
			updated_at: row.get(3)?,
		});
	}
	Ok(dir_list)
}
