mod app;
mod middlewares;
mod routes;
mod editor;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            "info,tower_http=debug"
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5001").await.unwrap();
    axum::serve(listener, app::create_api()).await.unwrap();

    println!("MH Frontier CQ Tool started!");
}