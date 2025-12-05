use axum::{Router, routing::post};

pub mod quest_edit;
pub mod quest_read;

pub fn get_router() -> Router {
    Router::new()
        .nest(
            "/quest",
            Router::new()
                .route("/read", post(quest_read::read_quest))
                .route("/edit", post(quest_edit::edit_quest)),
        )
}
