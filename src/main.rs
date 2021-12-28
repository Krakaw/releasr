mod errors;
mod models;
mod routes;

use crate::errors::ReleasrError;
use crate::models::get_connection;
use crate::routes::notes::{get_notes, new_note};
use actix_web::{web, App, HttpServer};
use std::sync::Mutex;

pub struct AppData {
    pub conn: Mutex<rusqlite::Connection>,
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");

    HttpServer::new(move || {
        let conn = get_connection("./db.sqlite3".to_string()).expect("Could not connect");
        let app_data = web::Data::new(AppData {
            conn: Mutex::new(conn),
        });
        App::new().app_data(app_data).service(
            web::resource("/notes")
                .route(web::get().to(get_notes))
                .route(web::post().to(new_note)),
        )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
