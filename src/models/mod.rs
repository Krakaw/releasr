use crate::ReleasrError;
use rusqlite::Connection;

pub mod custom_version;
pub mod environment;
pub mod note;

pub fn get_connection(path: String) -> Result<Connection, ReleasrError> {
    let conn = rusqlite::Connection::open(path)?;
    let _res = conn.execute("CREATE TABLE IF NOT EXISTS notes (id INTEGER PRIMARY KEY, version TEXT, version_int DOUBLE, note TEXT, environment_name TEXT, completed_at TEXT NULL, created_at TEXT, modified_at TEXT, deleted_at TEXT NULL)",[])?;
    let _res = conn.execute(
        "CREATE TABLE IF NOT EXISTS environments (name TEXT PRIMARY KEY, version_url NOT NULL)",
        [],
    )?;
    Ok(conn)
}
