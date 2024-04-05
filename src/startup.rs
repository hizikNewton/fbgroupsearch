use actix_web::{dev::Server, web, App, HttpServer};
use actix_web::middleware::Logger;
use sqlx::PgPool;
use std::net::TcpListener;

use crate::routes::{health_check,login_facebook};

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
   
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/health_check", web::get().to(health_check))
           
            .route("/fb_check", web::post().to(login_facebook)) 
             /* .route("/login", web::post().to(login))
           .app_data(db_pool.clone()) */
    })
    .listen(listener)?
    .run();
    Ok(server)
}
