use crate::models::custom_version::CustomVersion;

use crate::ReleasrError;
use chrono::{DateTime, Utc};
use rusqlite::{Connection, Row};

use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Deserialize, Serialize, Debug)]
pub struct NewNote {
    version: CustomVersion,
    note: String,
    environment_name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Note {
    id: i64,
    version: CustomVersion,
    version_int: f64,
    note: String,
    environment_name: String,
    completed_at: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    modified_at: DateTime<Utc>,
    deleted_at: Option<DateTime<Utc>>,
}

impl NewNote {
    pub async fn save(self, conn: &Connection) -> Result<Note, ReleasrError> {
        let now = Utc::now().to_rfc3339();
        let version_int: f64 = self.version.clone().into();
        let sql = r#"
            INSERT INTO notes
                (version, version_int, note, environment_name, created_at, modified_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6);
           "#;
        conn.execute(
            sql,
            &[
                &self.version.to_string(),
                &version_int.to_string(),
                &self.note,
                &self.environment_name,
                &now,
                &now,
            ],
        )?;
        let last_id: i64 = conn.query_row("SELECT last_insert_rowid();", [], |r| r.get(0))?;
        let note = Note::get(last_id, conn).await?;
        Ok(note)
    }
}

impl Note {
    pub async fn get(id: i64, conn: &Connection) -> Result<Note, ReleasrError> {
        let sql = "SELECT id, version, version_int, note,environment_name, completed_at, created_at, modified_at, deleted_at FROM notes WHERE id = ?1;";
        let mut stmt = conn.prepare(sql)?;
        let res = stmt.query_row(&[&id], |row| Note::try_from(row))?;
        Ok(res)
    }

    // pub async fn find(
    //     environment: String,
    //     version: CustomVersion,
    //     conn: &Connection,
    // ) -> Result<Vec<Note>, ReleasrError> {
    //     // let sql = "SELECT id, version, note,environment_name, completed_at, created_at, modified_at, deleted_at FROM notes WHERE "
    // }

    pub async fn list(conn: &Connection) -> Result<Vec<Note>, ReleasrError> {
        let sql = "SELECT id, version, version_int, note,environment_name, completed_at, created_at, modified_at, deleted_at FROM notes;";
        let mut stmt = conn.prepare(sql)?;
        let res = stmt
            .query_map([], |row| Note::try_from(row))?
            .map(Result::unwrap)
            .collect();
        Ok(res)
    }
}

impl<'stmt> TryFrom<&Row<'stmt>> for Note {
    type Error = rusqlite::Error;

    fn try_from(row: &Row) -> Result<Self, Self::Error> {
        let version: String = row.get(1).unwrap();
        let custom_version = CustomVersion::parse(&version).unwrap();
        let version = custom_version.clone().0;
        let version_int =
            (version.major.pow(9) + version.minor.pow(6) + version.patch.pow(3)) as f64;
        Ok(Note {
            id: row.get(0).unwrap(),
            version: custom_version,
            version_int,
            note: row.get(2).unwrap(),
            environment_name: row.get(3).unwrap(),
            completed_at: row.get(4).unwrap(),
            created_at: row.get(5).unwrap(),
            modified_at: row.get(6).unwrap(),
            deleted_at: row.get(7).unwrap(),
        })
    }
}
