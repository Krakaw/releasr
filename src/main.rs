mod config;
mod errors;
mod models;
mod routes;

use crate::config::Config;
use crate::errors::ReleasrError;
use crate::models::get_connection;
use crate::routes::environments::{get_environments, new_environment};
use crate::routes::notes::{complete_note, delete_note, get_notes, new_note};
use actix_web::{web, App, HttpServer};
use std::sync::Mutex;

pub struct AppData {
    pub conn: Mutex<rusqlite::Connection>,
    pub config: Config,
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let config = Config::init().expect("Invalid config");
    let listen = config.clone().listen;

    HttpServer::new(move || {
        let db_path = config.clone().db_path;
        let conn = get_connection(db_path).expect("Could not connect");
        let app_data = web::Data::new(AppData {
            conn: Mutex::new(conn),
            config: config.clone(),
        });
        App::new()
            .app_data(app_data)
            .service(
                web::resource("/environments")
                    .route(web::post().to(new_environment))
                    .route(web::get().to(get_environments)),
            )
            .service(
                web::resource("/notes")
                    .route(web::get().to(get_notes))
                    .route(web::post().to(new_note))
                    .route(web::patch().to(complete_note)),
            )
            .service(web::resource("/notes/{id}").route(web::delete().to(delete_note)))
    })
    .bind(listen)?
    .run()
    .await
}
