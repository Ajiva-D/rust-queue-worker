use axum::{Router, response::Html, response::Json, routing::get};
use serde_json::{Value, json};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler))
        .route("/health", get(health_handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await;
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, world!</h1>")
}

async fn health_handler() -> Json<Value> {
    Json(json!({"message":"ok"}))
}
