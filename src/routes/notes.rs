use crate::errors::ReleasrError;
use crate::models::custom_version::CustomVersion;
use crate::models::environment::Environment;
use crate::models::note::{NewNote, Note};
use crate::AppData;
use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize, Serialize, Debug)]
pub struct FindQuery {
    pub environment: Option<String>,
    pub version: Option<CustomVersion>,
    pub show_completed: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CompleteQuery {
    pub environment: String,
    pub version: CustomVersion,
}
pub async fn get_notes(
    app_data: web::Data<AppData>,
    query: web::Query<FindQuery>,
) -> Result<HttpResponse, ReleasrError> {
    let conn = app_data.conn.lock().unwrap();
    let notes = Note::find(query.0, &conn).await?;
    Ok(HttpResponse::Ok().json(notes))
}

pub async fn new_note(
    app_data: web::Data<AppData>,
    new_note: web::Json<NewNote>,
) -> Result<HttpResponse, ReleasrError> {
    let conn = app_data.conn.lock().unwrap();
    let new_note = new_note.into_inner();
    let version = i64::from(new_note.version.clone());
    let name = new_note.environment_name.clone();
    // Check it's a valid environment
    let environment = Environment::get(name, &conn).await?;
    if environment.last_deployed_version > version {
        return Err(ReleasrError::HistoricVersionError(
            environment.last_deployed_version,
            version,
        ));
    }
    let note = new_note.save(&conn).await?;
    Ok(HttpResponse::Ok().json(note))
}

pub async fn delete_note(
    app_data: web::Data<AppData>,
    note_id: web::Path<i64>,
) -> Result<HttpResponse, ReleasrError> {
    let conn = app_data.conn.lock().unwrap();
    let note = Note::get(note_id.into_inner(), &conn).await?;
    let note = note.destroy(&conn).await?;
    Ok(HttpResponse::Ok().json(note))
}

pub async fn complete_note(
    app_data: web::Data<AppData>,
    filter: web::Json<CompleteQuery>,
) -> Result<HttpResponse, ReleasrError> {
    let conn = app_data.conn.lock().unwrap();
    let filter = filter.into_inner();
    let version = filter.version.clone();
    let environment = Environment::get(filter.environment.clone(), &conn).await?;
    let completed_count = Note::complete_filter(filter, &conn).await?;
    let environment = environment.set_version(version.into(), &conn).await?;
    Ok(HttpResponse::Ok()
        .json(json!({"environment": environment, "completed_count": completed_count})))
}
