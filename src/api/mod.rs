use actix_web::{web, Scope};

pub mod quest;

pub fn get_all_routers() -> Scope {
    web::scope("")
        .service(quest::get_routers())
}