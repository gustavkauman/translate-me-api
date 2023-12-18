use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use axum::Router;
use dotenvy::dotenv;
use serde_json::json;
use std::net::SocketAddr;
use std::str::FromStr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::router::create_api_router;

mod api;
mod router;

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .nest("/api/v1", create_api_router().await)
        .fallback(handler_404);

    let addr = SocketAddr::from_str("127.0.0.1:3000").unwrap();
    tracing::debug!("listening on {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Json(json!(
            {"message": "The requested resource was not found"}
        )),
    )
}
