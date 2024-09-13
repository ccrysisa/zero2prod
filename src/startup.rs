use super::routes::{health_check, index, subscribe};
use actix_web::dev::Server;
use actix_web::{App, HttpServer};
use std::net::TcpListener;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .service(index)
            .service(health_check)
            .service(subscribe)
    })
    .listen(listener)?
    .run();
    Ok(server)
}
