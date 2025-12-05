use std::env;

use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

pub async fn require_api_key(req: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    let api_key = env::var("API_KEY").unwrap_or("".to_string());

    if api_key == "" {
        return  Ok(next.run(req).await);
    }

    let token = req
        .headers()
        .get("X-API-Key")
        .and_then(|v| v.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;


    if token != api_key {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(next.run(req).await)
}
