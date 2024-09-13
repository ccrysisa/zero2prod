use actix_web::{get, HttpResponse, Responder};

#[get("/")]
pub async fn index() -> impl Responder {
    "Hello, World!"
}

#[get("/health_check")]
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
