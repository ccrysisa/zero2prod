use actix_web::dev::Server;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use std::net::TcpListener;

#[get("/")]
async fn index() -> impl Responder {
    "Hello, World!"
}

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().service(index).service(health_check))
        .listen(listener)?
        .run();
    Ok(server)
}
