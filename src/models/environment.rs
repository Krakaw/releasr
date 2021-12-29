use crate::errors::ReleasrError;
use crate::routes::environments::FindQuery;
use rusqlite::Connection;
use rusqlite::{params, Row};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Deserialize, Debug, Serialize)]
pub struct NewEnvironment {
    pub name: String,
    pub version_url: Option<String>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Environment {
    pub name: String,
    pub version_url: Option<String>,
}

impl NewEnvironment {
    pub async fn save(&self, conn: &Connection) -> Result<Environment, ReleasrError> {
        let name = self.name.clone();
        let sql = r#"
            INSERT INTO environments (name, version_url)
            VALUES (?1, ?2)
            ON CONFLICT (name) DO UPDATE SET
                version_url = COALESCE(excluded.version_url, version_url);
        "#;
        conn.execute(sql, params![name, self.version_url])?;
        Environment::get(name, conn).await
    }
}

impl Environment {
    pub async fn get(name: String, conn: &Connection) -> Result<Environment, ReleasrError> {
        let sql = "SELECT * FROM environments WHERE name = ?1";
        let mut stmt = conn.prepare(sql)?;
        let environment = stmt.query_row(params![name], |row| Environment::try_from(row))?;
        Ok(environment)
    }

    pub async fn find(
        find_query: FindQuery,
        conn: &Connection,
    ) -> Result<Vec<Environment>, ReleasrError> {
        let sql = "SELECT * FROM environments WHERE (?1 IS NULL OR name = ?1)";
        let mut stmt = conn.prepare(sql)?;
        let res = stmt
            .query_map(params![find_query.name], |row| Environment::try_from(row))?
            .map(Result::unwrap)
            .collect();
        Ok(res)
    }
}

impl<'stmt> TryFrom<&Row<'stmt>> for Environment {
    type Error = rusqlite::Error;

    fn try_from(row: &Row<'stmt>) -> Result<Self, Self::Error> {
        Ok(Environment {
            name: row.get(0).unwrap(),
            version_url: row.get(1).unwrap(),
        })
    }
}
