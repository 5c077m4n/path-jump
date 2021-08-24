use rusqlite::{Connection, Result};

pub mod dir;
pub mod bookmark;

pub fn init(conn: &mut Connection) -> Result<()> {
	let tx = conn.transaction()?;
	tx.execute(
		"CREATE TABLE IF NOT EXISTS dir_scoring (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            path TEXT UNIQUE NOT NULL,
            score INTEGER DEFAULT 0 NOT NULL,
            created_at INTEGER DEFAULT (CAST((julianday('now') - 2440587.5) * 86400000 AS INTEGER)) NOT NULL,
            updated_at INTEGER DEFAULT (CAST((julianday('now') - 2440587.5) * 86400000 AS INTEGER)) NOT NULL
        );",
        []
    )?;
	tx.execute(
		"CREATE TRIGGER IF NOT EXISTS dir_scoring__updated_at
            AFTER UPDATE
            ON dir_scoring
            FOR EACH ROW
            BEGIN
                UPDATE dir_scoring
                    SET updated_at = (CAST((julianday('now') - 2440587.5) * 86400000 AS INTEGER))
                    WHERE id = OLD.id;
            END;",
		[],
	)?;
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
            END;",
		[],
	)?;
	tx.commit()
}
