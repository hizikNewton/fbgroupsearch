//! lib.rs
use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpResponse, HttpServer};

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    password: String
}


async fn login(_form:web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
    }


pub fn run(listener:TcpListener) -> Result<Server, std::io::Error> {

    let server = HttpServer::new(|| {
        App::new()
        .route("/health_check", web::get().to(health_check))
        .route("/login",web::post().to(login))
    })
    .listen(listener)?
    .run();

    Ok(server)
}