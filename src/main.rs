use actix_web::{App, HttpServer};

pub mod editor;
pub mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new()
            .service(api::get_all_routers())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}