mod routes;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use crate::routes::notes::{get_notes, new_note};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/notes").route(web::get().to(get_notes)).route(web::post().to(new_note)))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
