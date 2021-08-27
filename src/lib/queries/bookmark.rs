use std::path::PathBuf;

use rusqlite::{Connection, Result};

use crate::types::Bookmark;

pub fn init_tables(conn: &mut Connection) -> Result<()> {
	let tx = conn.transaction()?;
	tx.execute(
		"CREATE TABLE IF NOT EXISTS bookmarks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT UNIQUE NOT NULL,
            path TEXT NOT NULL,
            created_at INTEGER DEFAULT (CAST((julianday('now') - 2440587.5) * 86400000 AS INTEGER)) NOT NULL,
            updated_at INTEGER DEFAULT (CAST((julianday('now') - 2440587.5) * 86400000 AS INTEGER)) NOT NULL
        );",
        []
    )?;
	tx.execute(
		"CREATE TRIGGER IF NOT EXISTS bookmarks__updated_at
            AFTER UPDATE
            ON bookmarks
            FOR EACH ROW
            BEGIN
                UPDATE bookmarks
                    SET updated_at = (CAST((julianday('now') - 2440587.5) * 86400000 AS INTEGER))
                    WHERE id = OLD.id;
            END;
        ",
		[],
	)?;
	tx.commit()
}

pub fn get(conn: &Connection, name: &str) -> Result<String> {
	conn.query_row(
		"SELECT b.path FROM bookmarks AS b;",
		&[(":name", name)],
		|row| row.get(0),
	)
}

pub fn add(conn: &Connection, name: &str, path: &str) -> Result<usize> {
	conn.execute(
		"INSERT INTO bookmarks (name, path) VALUES (:name, :path)
            ON CONFLICT(name)
                DO UPDATE
                SET path = :path
                WHERE name = :name;
        ",
		&[(":name", name), (":path", path)],
	)
}

pub fn delete(conn: &Connection, name: &str) -> Result<usize> {
	conn.execute(
		"DELETE FROM bookmarks WHERE name = :name;",
		&[(":name"), name],
	)
}

pub fn get_dump(conn: &Connection) -> Result<Vec<Bookmark>> {
	let mut stmt = conn.prepare(
		"SELECT b.name, b.path, b.created_at, b.updated_at
           FROM bookmarks AS b",
	)?;
	let mut rows = stmt.query([])?;

	let mut bookmark_list: Vec<Bookmark> = Vec::new();
	while let Some(row) = rows.next()? {
		bookmark_list.push(Bookmark {
			name: row.get(0)?,
			path: PathBuf::from(row.get::<usize, String>(1)?),
			created_at: row.get(2)?,
			updated_at: row.get(3)?,
		});
	}
	Ok(bookmark_list)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	#[should_panic]
	fn should_make_sure_db_is_empty() {
		let db_conn = Connection::open_in_memory().unwrap();
		add(
			&db_conn,
			"should not be created",
			"Whatever, this should panic anyway",
		)
		.unwrap();
	}

	#[test]
	fn should_create_table() -> Result<()> {
		let mut db_conn = Connection::open_in_memory()?;
		init_tables(&mut db_conn)?;

		let dump = get_dump(&db_conn)?;
		assert_eq!(dump.len(), 0);

		Ok(())
	}

	#[test]
	fn should_add_dir() -> Result<()> {
		let mut db_conn = Connection::open_in_memory()?;
		init_tables(&mut db_conn)?;

		let n = add(&db_conn, "bm1", "path")?;
		assert_eq!(n, 1);

		Ok(())
	}
}
