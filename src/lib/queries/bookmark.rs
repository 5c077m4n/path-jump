use std::path::PathBuf;

use rusqlite::{Connection, Result};

use crate::types::Bookmark;

pub fn get_bookmark(conn: &Connection, name: &str) -> Result<String> {
	conn.query_row(
		"SELECT b.path
           FROM bookmarks AS b",
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
                WHERE name = :name;",
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
