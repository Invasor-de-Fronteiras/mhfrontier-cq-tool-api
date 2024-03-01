use actix_web::{web, Scope};
use actix_web::{HttpResponse, Responder};
use serde::Serialize;

pub mod quest;

#[derive(Serialize)]
struct HealthCheck {
    status: String,
    message: String
}

async fn health() -> impl Responder {
    HttpResponse::Ok().json(HealthCheck {
        status: String::from("OK"),
        message: String::from("MHFrontier cq tool is alive!!")
    })
}

pub fn get_all_routers() -> Scope {
    web::scope("")
        .service(quest::get_routers())
        .route("/", web::get().to(health))
        .route("/health", web::get().to(health))
}