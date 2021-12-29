

use crate::AppData;
use actix_web::{web, Error, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use crate::models::environment::Environment;

#[derive(Deserialize, Serialize, Debug)]
pub struct FindQuery {
    pub name: Option<String>,
}

pub async fn get_environments(
    app_data: web::Data<AppData>,
    query: web::Query<FindQuery>,
) -> Result<HttpResponse, Error> {
    let conn = app_data.conn.lock().unwrap();

    let environment = Environment::find(query.0, &conn)
        .await
        .map_err(|_| HttpResponse::InternalServerError())?;
    Ok(HttpResponse::Ok().json(environment))
}
