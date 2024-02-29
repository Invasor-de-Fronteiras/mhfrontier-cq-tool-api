use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;

pub mod editor;
pub mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting api...");

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let api = HttpServer::new(|| {
        App::new()
            .wrap(Logger::new("%a \"%r\" %{User-Agent}i"))
            .service(api::get_all_routers())
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await;

    return api;
}