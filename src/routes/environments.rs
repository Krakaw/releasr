use crate::errors::ReleasrError;
use crate::models::environment::{Environment, NewEnvironment};
use crate::AppData;
use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct FindQuery {
    pub name: Option<String>,
}

pub async fn get_environments(
    app_data: web::Data<AppData>,
    query: web::Query<FindQuery>,
) -> Result<HttpResponse, ReleasrError> {
    let conn = app_data.conn.lock().unwrap();
    let environment = Environment::find(query.0, &conn).await?;
    Ok(HttpResponse::Ok().json(environment))
}

pub async fn new_environment(
    app_data: web::Data<AppData>,
    new_environment: web::Json<NewEnvironment>,
) -> Result<HttpResponse, ReleasrError> {
    let conn = app_data.conn.lock().unwrap();
    let environment = new_environment.into_inner().save(&conn).await?;
    Ok(HttpResponse::Ok().json(environment))
}
