use crate::ReleasrError;
use rusqlite::Connection;

pub mod custom_version;
pub mod environment;
pub mod note;

pub fn get_connection(path: String) -> Result<Connection, ReleasrError> {
    let conn = rusqlite::Connection::open(path)?;
    let _res = conn.execute("CREATE TABLE IF NOT EXISTS notes (id INTEGER PRIMARY KEY, version TEXT, version_int INTEGER, note TEXT, environment TEXT, completed_at TEXT NULL, created_at TEXT, modified_at TEXT)",[])?;
    let _res = conn.execute(
        "CREATE TABLE IF NOT EXISTS environments (name TEXT PRIMARY KEY, version_url TEXT NOT NULL, last_deployed_version INTEGER NULL)",
        [],
    )?;
    Ok(conn)
}
