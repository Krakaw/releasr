use crate::models::custom_version::CustomVersion;
use crate::models::note::{NewNote, Note};
use crate::AppData;
use actix_web::{web, Error, HttpResponse, Result};
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Debug)]
pub struct FindQuery {
    pub environment: Option<String>,
    pub version: Option<CustomVersion>,
    pub show_completed: Option<bool>,
}
pub async fn get_notes(
    app_data: web::Data<AppData>,
    query: web::Query<FindQuery>,
) -> Result<HttpResponse, Error> {
    let conn = app_data.conn.lock().unwrap();

    let notes = Note::find(query.0, &conn)
        .await
        .map_err(|_| HttpResponse::InternalServerError())?;
    Ok(HttpResponse::Ok().json(notes))
}

pub async fn new_note(
    app_data: web::Data<AppData>,
    new_note: web::Json<NewNote>,
) -> Result<HttpResponse, Error> {
    eprintln!("new_note = {:?}", new_note);
    let conn = app_data.conn.lock().unwrap();
    let note = new_note.into_inner().save(&conn).await.map_err(|e| {
        eprintln!("e = {:?}", e);
        HttpResponse::InternalServerError()
    })?;

    Ok(HttpResponse::Ok().json(note))
}
