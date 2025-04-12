use axum::{Router, routing::get};

use lab5_pis::{get_currency_rate, get_moodle};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(|| async { "Hello!" }))
        .route("/rate", get(get_currency_rate))
        .route("/moodle/{moodle}", get(get_moodle));

    tracing::info!("Server started on http://127.0.0.1:3000");

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
