use axum::{
    Router,
    http::StatusCode,
    response::{Html, Json},
    routing::{get, post},
};
use serde_json::{Value, json};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler))
        .route("/health", get(health_handler))
        .route("/jobs", post(create_jobs));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, world!</h1>")
}

async fn health_handler() -> Json<Value> {
    Json(json!({"message":"ok"}))
}

async fn create_jobs() -> (StatusCode, Json<Value>) {
    let id = Uuid::new_v4();
    (
        StatusCode::ACCEPTED,
        Json(json!({
            "id":id,
            "status":"queued"
        })),
    )
}
