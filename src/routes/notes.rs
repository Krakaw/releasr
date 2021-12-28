use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Error};
use semver::Version;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Deserialize,  Debug)]
pub struct NewNote {
    version: Version,
    note: String
}


pub async fn get_notes() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

pub async fn new_note(new_note: web::Json<NewNote>) -> Result<HttpResponse, Error> {
    eprintln!("new_note = {:?}", new_note);
    Ok(HttpResponse::Ok().body("Hey there!"))
}
