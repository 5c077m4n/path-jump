use std::path::PathBuf;

use rusqlite::{Connection, Result};

use crate::types::DirScore;

pub fn upsert(conn: &Connection, dir_path: &str) -> Result<usize> {
	conn.execute(
		"INSERT INTO dir_scoring (path) VALUES (:path)
            ON CONFLICT(path)
                DO UPDATE
                SET score = score + 1
                WHERE path = :path;",
		&[(":path", dir_path)],
	)
}

pub fn find(conn: &Connection, dir_path: &str) -> Result<String> {
	let dir_path = &dir_path.to_lowercase();
	conn.query_row(
		"SELECT ds.path
            FROM dir_scoring as ds
            WHERE lower(ds.path) LIKE '%' || :path || '%'
            ORDER BY ds.score DESC, ds.updated_at DESC, ds.path ASC;
        ",
		&[(":path", dir_path)],
		|row| row.get(0),
	)
}

pub fn clear_history(conn: &Connection) -> Result<usize> {
	conn.execute("DELETE FROM dir_scoring;", [])
}

pub fn get_dump(conn: &Connection) -> Result<Vec<DirScore>> {
	let mut stmt = conn.prepare(
		"SELECT ds.path, ds.score, ds.created_at, ds.updated_at
            FROM dir_scoring AS ds;",
	)?;
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

#[cfg(test)]
mod tests {
	use super::super::init;
	use super::*;

	#[test]
	#[should_panic]
	fn should_make_sure_db_is_empty() {
		let db_conn = Connection::open_in_memory().unwrap();
		upsert(&db_conn, "Whatever, this should panic anyway").unwrap();
	}

	#[test]
	fn should_create_table() -> Result<()> {
		let mut db_conn = Connection::open_in_memory()?;
		init(&mut db_conn)?;

		let dump = get_dump(&db_conn)?;
		assert_eq!(dump.len(), 0);

		Ok(())
	}

	#[test]
	fn should_add_dir() -> Result<()> {
		let mut db_conn = Connection::open_in_memory()?;
		init(&mut db_conn)?;

		let n = upsert(&db_conn, "path")?;
		assert_eq!(n, 1);

		Ok(())
	}

	#[test]
	fn should_get_best_scored_dir() -> Result<()> {
		let mut db_conn = Connection::open_in_memory()?;
		init(&mut db_conn)?;

		upsert(&db_conn, "path")?;
		upsert(&db_conn, "path")?;
		upsert(&db_conn, "path")?;
		upsert(&db_conn, "other/path")?;
		upsert(&db_conn, "another/path")?;
		upsert(&db_conn, "yet/another/path")?;

		let best_path = find(&db_conn, "path")?;
		assert_eq!(best_path, "path");

		Ok(())
	}
}
