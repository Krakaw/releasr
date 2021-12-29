use std::convert::TryFrom;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use crate::errors::ReleasrError;
use crate::routes::environments::FindQuery;
use rusqlite::{params,  Row};


#[derive(Deserialize, Debug, Serialize)]
pub struct Environment {
    pub name: String,
    pub version_url: Option<String>,
}

impl Environment {
    pub async fn find(find_query: FindQuery, conn: &Connection) -> Result<Vec<Environment>, ReleasrError> {
        let sql = "SELECT * FROM environments WHERE (?1 IS NULL OR name = ?1)";
        let mut stmt = conn.prepare(sql)?;
        let res = stmt.query_map(params![find_query.name], |row| Environment::try_from(row))?.map(Result::unwrap).collect();
        Ok(res)
    }
}

impl<'stmt> TryFrom<&Row<'stmt>> for Environment {
    type Error = rusqlite::Error;

    fn try_from(row: &Row<'stmt>) -> Result<Self, Self::Error> {
        Ok(Environment {
            name: row.get(0).unwrap(),
            version_url: row.get(0).unwrap()
        })
    }
}
