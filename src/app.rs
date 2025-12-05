use axum::Router;
use tower_http::trace::TraceLayer;
use crate::routes::get_router;
use axum::middleware::from_fn;
use crate::middlewares::api_key::require_api_key;

pub fn create_api() -> Router {
    Router::new()
        .nest(
            "/",
            get_router().layer(from_fn(require_api_key))
        )
        .layer(TraceLayer::new_for_http())
}
