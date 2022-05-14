use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, get_service},
    Json, Router,
};
use serde_json::json;
use std::{io, net::SocketAddr};
use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "example_static_file_server=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // for serving assets directly at the root you can use `tower_http::services::ServeDir`
    // as the fallback to a `Router`
    let app: _ = Router::new()
        .route("/api/cards", get(list_cards))
        .fallback(get_service(ServeDir::new("frontend/dist/")).handle_error(handle_error))
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handle_error(_err: io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}

async fn list_cards() -> impl IntoResponse {
    Json(json!(
        [
            {
                "id": "1",
                "title": "Hello",
                "options": [{
                    "id": "2",
                    "text": "goodbye"
                }]
            }
        ]
    ))
}
